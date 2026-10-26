import { useState, useEffect, useCallback } from 'react';
import type { CharacterSheet } from './types/character';
import type { CampaignSession, LivingSceneData } from './types/session';
import { CharacterCreationWizard } from './components/character-creation/CharacterCreationWizard';
import { BaseGameHUD } from './components/game/BaseGameHUD';
import { SessionLobby } from './components/game/SessionLobby';
import { AuditModal } from './components/game/AuditModal';
import {
  checkBackendHealth,
  syncCharacterWithBackend,
  syncPartyWithCampaign,
  fetchCampaignAudit,
  createCampaign,
} from './services/api';
import { createQuickTestParty } from './utils/quickParty';
import { AuthModal } from './components/auth/AuthModal';
import { AdminPanel } from './components/admin/AdminPanel';
import { getAuthUser, clearAuthSession, type AuthUser } from './services/auth';
import { apiLogout } from './services/api';
import {
  Globe2,
  UserPlus,
  Gamepad2,
  Search,
  Sparkles,
  LogOut,
  Zap,
  Crown,
  Key,
} from 'lucide-react';
import './App.css';

const LOCAL_STORAGE_SESSION_KEY = 'rpg_campaign_session';
const LOCAL_STORAGE_CHAR_KEY = 'rpg_active_character';
const LOCAL_STORAGE_PARTY_KEY = 'rpg_active_party';

export type AppView = 'lobby' | 'character' | 'game' | 'admin';

export interface ToastNotice {
  id: number;
  text: string;
  type: 'info' | 'success' | 'warning';
}

export function App() {
  const [session, setSession] = useState<CampaignSession | null>(() => {
    try {
      const saved = localStorage.getItem(LOCAL_STORAGE_SESSION_KEY);
      return saved ? JSON.parse(saved) : null;
    } catch {
      return null;
    }
  });

  const [character, setCharacter] = useState<CharacterSheet | null>(() => {
    try {
      if (!localStorage.getItem(LOCAL_STORAGE_SESSION_KEY)) return null;
      const saved = localStorage.getItem(LOCAL_STORAGE_CHAR_KEY);
      return saved ? JSON.parse(saved) : null;
    } catch {
      return null;
    }
  });

  const [party, setParty] = useState<CharacterSheet[]>(() => {
    try {
      if (!localStorage.getItem(LOCAL_STORAGE_SESSION_KEY)) return [];
      const saved = localStorage.getItem(LOCAL_STORAGE_PARTY_KEY);
      return saved ? JSON.parse(saved) : [];
    } catch {
      return [];
    }
  });

  // Navegação explícita por abas
  const [currentView, setCurrentView] = useState<AppView>(() => {
    if (window.location.hash === '#admin' || window.location.pathname === '/admin') {
      return 'admin';
    }
    if (!session) return 'lobby';
    if (!character) return 'character';
    return 'game';
  });

  useEffect(() => {
    const handleHashChange = () => {
      if (window.location.hash === '#admin') {
        setCurrentView('admin');
      } else if (currentView === 'admin' && window.location.hash === '') {
        setCurrentView('lobby');
      }
    };
    window.addEventListener('hashchange', handleHashChange);
    return () => window.removeEventListener('hashchange', handleHashChange);
  }, [currentView]);

  const [isAddingPartyMember, setIsAddingPartyMember] = useState<boolean>(false);
  const [showGlobalAudit, setShowGlobalAudit] = useState<boolean>(false);
  const [backendOnline, setBackendOnline] = useState<boolean>(false);
  const [toast, setToast] = useState<ToastNotice | null>(null);

  // SaaS Auth & Admin State
  const [authUser, setAuthUser] = useState<AuthUser | null>(() => getAuthUser());
  const [showAuthModal, setShowAuthModal] = useState<boolean>(false);
  const [authModalMode, setAuthModalMode] = useState<'login' | 'register'>('login');

  const showToast = useCallback(
    (text: string, type: 'info' | 'success' | 'warning' = 'info') => {
      setToast({ id: Date.now(), text, type });
    },
    []
  );

  const handleLogout = async () => {
    try {
      await apiLogout();
    } catch {}
    clearAuthSession();
    setAuthUser(null);
    showToast('Sessão encerrada com sucesso.', 'info');
  };

  useEffect(() => {
    if (!toast) return;
    const timer = setTimeout(() => {
      setToast(null);
    }, 4200);
    return () => clearTimeout(timer);
  }, [toast]);

  useEffect(() => {
    checkBackendHealth().then((health) => {
      setBackendOnline(Boolean(health));
    });
  }, []);

  const persistPartyAndSyncBackend = useCallback(
    (updatedParty: CharacterSheet[], activeSheet: CharacterSheet, targetSession?: CampaignSession | null) => {
      setParty(updatedParty);
      setCharacter(activeSheet);
      try {
        localStorage.setItem(LOCAL_STORAGE_PARTY_KEY, JSON.stringify(updatedParty));
        localStorage.setItem(LOCAL_STORAGE_CHAR_KEY, JSON.stringify(activeSheet));
      } catch (e) {
        console.warn('Erro ao salvar party local:', e);
      }

      const activeSess = targetSession !== undefined ? targetSession : session;
      if (activeSess?.campaign_id) {
        syncPartyWithCampaign(activeSess.campaign_id, updatedParty).then((res) => {
          if (res?.player_slots) {
            setSession((prev) => {
              if (!prev) return prev;
              const nextSess = {
                ...prev,
                player_slots: res.player_slots,
                player_ids: res.player_slots.map((s) => s.player_id),
                player_name: activeSheet.name,
                class_name: activeSheet.className,
              };
              try {
                localStorage.setItem(LOCAL_STORAGE_SESSION_KEY, JSON.stringify(nextSess));
              } catch {}
              return nextSess;
            });
          }
        });
      } else {
        syncCharacterWithBackend(activeSheet);
      }
    },
    [session]
  );

  // Quando a sessão é configurada (via Criação de Novo Mundo ou Importação de Campanha)
  const handleSessionReady = async (newSession: CampaignSession, initialParty?: CharacterSheet[]) => {
    setSession(newSession);
    try {
      localStorage.setItem(LOCAL_STORAGE_SESSION_KEY, JSON.stringify(newSession));
    } catch (e) {
      console.warn('Erro ao salvar sessão local:', e);
    }

    // CASO 1: Heróis já foram fornecidos diretamente pelo Lobby Pré-Campanha (Composição Prévia)
    if (initialParty && initialParty.length > 0) {
      persistPartyAndSyncBackend(initialParty, initialParty[0], newSession);
      showToast(`Comitiva de ${initialParty.length} heróis ingressou nas Crônicas!`, 'success');
      setCurrentView('game');
      return;
    }

    // CASO 2: Campanha existente com histórico e jogadores pré-gravados (backup importado ou retomada no Tab 5)
    if (newSession.campaign_id) {
      try {
        const audit = await fetchCampaignAudit(newSession.campaign_id);
        const rawPlayers = audit?.player_entities || audit?.players || [];
        if (rawPlayers.length > 0) {
          const loadedParty: CharacterSheet[] = rawPlayers.map((p: any) => {
            if (p.custom_flags?.full_sheet && p.custom_flags.full_sheet.vitals) {
              return p.custom_flags.full_sheet as CharacterSheet;
            }
            const str = p.dnd_stats?.ability_scores?.STR || 14;
            const dex = p.dnd_stats?.ability_scores?.DEX || 14;
            const con = p.dnd_stats?.ability_scores?.CON || 14;
            const int = p.dnd_stats?.ability_scores?.INT || 10;
            const wis = p.dnd_stats?.ability_scores?.WIS || 12;
            const cha = p.dnd_stats?.ability_scores?.CHA || 10;
            const maxHp = p.dnd_stats?.max_hp || 12;
            const curHp = p.dnd_stats?.current_hp ?? maxHp;
            const lvl = p.custom_flags?.level || p.level || 1;
            return {
              id: p.id || `char-${Date.now()}`,
              name: p.name || 'Herói',
              race: p.custom_flags?.race || 'Humano',
              className: (p.custom_flags?.class_name || p.class_name || p.className || 'Fighter') as any,
              level: lvl,
              background: p.custom_flags?.background || 'Aventureiro',
              alignment: 'Neutro',
              proficiencyBonus: Math.ceil(lvl / 4) + 1,
              abilities: {
                strength: str,
                dexterity: dex,
                constitution: con,
                intelligence: int,
                wisdom: wis,
                charisma: cha,
              },
              savingThrowProficiencies: ['strength', 'constitution'],
              skillProficiencies: p.dnd_stats?.proficiencies || ['Percepção'],
              vitals: {
                hpCurrent: curHp,
                hpMax: maxHp,
                hpTemp: p.dnd_stats?.temporary_hp || 0,
                armorClass: p.dnd_stats?.armor_class || 15,
                initiativeModifier: Math.floor((dex - 10) / 2),
                speedFeet: 30,
                hitDiceTotal: lvl,
                hitDiceCurrent: lvl,
                hitDiceType: 'd10',
                deathSaves: { successes: 0, failures: 0 },
                conditions: p.dnd_stats?.conditions || [],
              },
              spellSlots: { levels: {} },
              spellsKnownOrPrepared: [],
              classResources: {},
              inventory: p.custom_flags?.inventory || [
                { name: 'Equipamento de Aventureiro', quantity: 1, weightLbs: 5, isEquipped: true },
              ],
            };
          });
          setParty(loadedParty);
          setCharacter(loadedParty[0]);
          try {
            localStorage.setItem(LOCAL_STORAGE_PARTY_KEY, JSON.stringify(loadedParty));
            localStorage.setItem(LOCAL_STORAGE_CHAR_KEY, JSON.stringify(loadedParty[0]));
          } catch {}
          setCurrentView('game');
          return;
        }
      } catch (err) {
        console.warn('Campanha sem jogadores pré-gravados:', err);
      }
    }

    // CASO 3: Nova Campanha sem heróis pré-definidos (fallback legado)
    setCharacter(null);
    setParty([]);
    setIsAddingPartyMember(false);
    try {
      localStorage.removeItem(LOCAL_STORAGE_CHAR_KEY);
      localStorage.removeItem(LOCAL_STORAGE_PARTY_KEY);
    } catch {}

    // Direciona o jogador imediatamente para a Forja de Personagem (Tela 2) para criar seu novo herói!
    setCurrentView('character');
  };

  const handleCharacterCreated = async (sheet: CharacterSheet) => {
    let updatedParty: CharacterSheet[];
    if (isAddingPartyMember && party.length > 0 && party.length < 4) {
      updatedParty = [...party, sheet];
    } else if (party.length === 0) {
      updatedParty = [sheet];
    } else {
      // Substitui o herói ativo atual ou adiciona se ainda não estava na lista
      const idx = character ? party.findIndex((p) => p.id === character.id) : -1;
      if (idx >= 0) {
        updatedParty = [...party];
        updatedParty[idx] = sheet;
      } else {
        updatedParty = [...party.slice(0, 3), sheet];
      }
    }

    setIsAddingPartyMember(false);
    let currentSession = session;
    if (!currentSession) {
      try {
        currentSession = await createCampaign({
          premise: 'Crônicas do Reino — Campanha Inicial',
          literary_style: 'Dark Fantasy',
          host_player_name: sheet.name,
          host_class: sheet.className,
        });
        setSession(currentSession);
        try {
          localStorage.setItem(LOCAL_STORAGE_SESSION_KEY, JSON.stringify(currentSession));
        } catch {}
      } catch (e) {
        console.warn('Erro ao auto-criar campanha para herói:', e);
      }
    }

    persistPartyAndSyncBackend(updatedParty, sheet, currentSession);
    showToast(`Herói "${sheet.name}" (${sheet.race} ${sheet.className} Nv.${sheet.level}) sincronizado com a campanha!`, 'success');
    setCurrentView('game');
  };

  const handleSelectPartyMember = (selected: CharacterSheet) => {
    setCharacter(selected);
    try {
      localStorage.setItem(LOCAL_STORAGE_CHAR_KEY, JSON.stringify(selected));
    } catch {}
  };

  const handleUpdateCharacter = useCallback(
    (updatedSheet: CharacterSheet) => {
      const nextParty =
        party.length > 0
          ? party.map((p) => (p.id === updatedSheet.id ? updatedSheet : p))
          : [updatedSheet];
      persistPartyAndSyncBackend(nextParty, updatedSheet);
    },
    [party, persistPartyAndSyncBackend]
  );

  const handleSyncSceneAndParty = useCallback(
    (scene?: LivingSceneData | null, playerEntities?: Array<Record<string, unknown>>) => {
      setParty((prevParty) => {
        if (prevParty.length === 0) return prevParty;
        let changed = false;
        const nextParty = prevParty.map((hero) => {
          const partMatch = scene?.participants?.find(
            (p) =>
              p.is_player &&
              (p.id === hero.id || p.name.toLowerCase() === hero.name.toLowerCase())
          );
          const entMatch = playerEntities?.find(
            (ent: any) =>
              ent?.id === hero.id ||
              (typeof ent?.name === 'string' && ent.name.toLowerCase() === hero.name.toLowerCase())
          ) as any;

          const newHp =
            partMatch?.current_hp ??
            entMatch?.dnd_stats?.current_hp ??
            entMatch?.current_hp;
          const newMaxHp =
            partMatch?.max_hp ??
            entMatch?.dnd_stats?.max_hp ??
            entMatch?.max_hp;

          if (
            (newHp !== undefined && newHp !== hero.vitals.hpCurrent) ||
            (newMaxHp !== undefined && newMaxHp !== hero.vitals.hpMax)
          ) {
            changed = true;
            return {
              ...hero,
              vitals: {
                ...hero.vitals,
                hpCurrent: newHp !== undefined ? Math.max(0, Number(newHp)) : hero.vitals.hpCurrent,
                hpMax: newMaxHp !== undefined ? Math.max(1, Number(newMaxHp)) : hero.vitals.hpMax,
              },
            };
          }
          return hero;
        });

        if (changed) {
          try {
            localStorage.setItem(LOCAL_STORAGE_PARTY_KEY, JSON.stringify(nextParty));
          } catch {}
        }

        if (scene?.participants && scene.participants.length > 0) {
          const activeIdx =
            (scene.current_participant_index ?? 0) % scene.participants.length;
          const activePart = scene.participants[activeIdx];
          if (activePart?.is_player) {
            const matchedHero = nextParty.find(
              (h) =>
                h.id === activePart.id ||
                h.name.toLowerCase() === activePart.name.toLowerCase()
            );
            if (matchedHero) {
              setCharacter(matchedHero);
              try {
                localStorage.setItem(LOCAL_STORAGE_CHAR_KEY, JSON.stringify(matchedHero));
              } catch {}
            }
          }
        }

        return changed ? nextParty : prevParty;
      });
    },
    []
  );

  const handleRequestAddPartyMember = () => {
    if (party.length >= 4) {
      showToast('Limite máximo de 4 heróis na mesa atingido.', 'warning');
      return;
    }
    setIsAddingPartyMember(true);
    setCurrentView('character');
  };

  const handleResetCharacter = () => {
    setIsAddingPartyMember(false);
    setCurrentView('character');
  };

  const handleExitSession = () => {
    setSession(null);
    setCharacter(null);
    setParty([]);
    setIsAddingPartyMember(false);
    try {
      localStorage.removeItem(LOCAL_STORAGE_SESSION_KEY);
      localStorage.removeItem(LOCAL_STORAGE_CHAR_KEY);
      localStorage.removeItem(LOCAL_STORAGE_PARTY_KEY);
    } catch {}
    setCurrentView('lobby');
  };

  const handleUseQuickTestParty = async () => {
    const quick = createQuickTestParty();
    setIsAddingPartyMember(false);
    let currentSession = session;
    if (!currentSession) {
      try {
        currentSession = await createCampaign({
          premise: 'A Caverna dos Lamentos — Aventura de Teste Rápido',
          literary_style: 'Dark Fantasy',
          host_player_name: quick[0].name,
          host_class: quick[0].className,
        });
        setSession(currentSession);
        try {
          localStorage.setItem(LOCAL_STORAGE_SESSION_KEY, JSON.stringify(currentSession));
        } catch {}
      } catch (e) {
        console.warn('Erro ao auto-criar campanha de teste rápido:', e);
      }
    }
    persistPartyAndSyncBackend(quick, quick[0], currentSession);
    showToast('Party Pronta (4 Heróis Nível 1) carregada e sincronizada com o servidor!', 'success');
    setCurrentView('game');
  };

  return (
    <div className="app-container" style={{ position: 'relative', minHeight: '100vh', display: 'flex', flexDirection: 'column' }}>
      {/* TOAST NOTIFICATION (DARK FANTASY) */}
      {toast && (
        <div
          style={{
            position: 'fixed',
            top: '62px',
            right: '20px',
            zIndex: 100000,
            backgroundColor:
              toast.type === 'success'
                ? 'rgba(6, 78, 59, 0.96)'
                : toast.type === 'warning'
                ? 'rgba(120, 53, 15, 0.96)'
                : 'rgba(15, 23, 42, 0.96)',
            border:
              toast.type === 'success'
                ? '1px solid #10b981'
                : toast.type === 'warning'
                ? '1px solid #f59e0b'
                : '1px solid #38bdf8',
            color: '#f8fafc',
            padding: '10px 16px',
            borderRadius: '10px',
            boxShadow: '0 8px 24px rgba(0, 0, 0, 0.75)',
            fontSize: '12.5px',
            fontWeight: 600,
            maxWidth: '420px',
            display: 'flex',
            alignItems: 'center',
            gap: '10px',
          }}
        >
          <span>{toast.text}</span>
          <button
            onClick={() => setToast(null)}
            style={{
              background: 'transparent',
              border: 'none',
              color: '#94a3b8',
              cursor: 'pointer',
              fontSize: '14px',
              padding: '0 4px',
            }}
          >
            ✕
          </button>
        </div>
      )}

      {/* BARRA DE NAVEGAÇÃO PRINCIPAL (UX SPA) */}
      {currentView !== 'admin' && (
        <header
          style={{
            backgroundColor: '#07090e',
          borderBottom: '1px solid rgba(255, 255, 255, 0.1)',
          padding: '8px 20px',
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          flexWrap: 'wrap',
          gap: '12px',
          position: 'sticky',
          top: 0,
          zIndex: 1000,
          boxShadow: '0 4px 20px rgba(0,0,0,0.6)',
        }}
      >
        {/* Lado Esquerdo: Identidade & Badges */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '14px', flexWrap: 'wrap' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <Sparkles size={16} color="#facc15" />
            <strong style={{ fontSize: '13px', color: '#fff', letterSpacing: '0.04em' }}>
              RPG ENGINE D&amp;D 5E
            </strong>
          </div>

          {session ? (
            <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
              <span
                style={{
                  backgroundColor: 'rgba(234, 179, 8, 0.16)',
                  color: '#facc15',
                  padding: '2px 8px',
                  borderRadius: '6px',
                  border: '1px solid rgba(234, 179, 8, 0.4)',
                  fontSize: '11px',
                  fontWeight: 800,
                  fontFamily: 'monospace',
                }}
              >
                {session.is_offline ? '🖥️ OFFLINE' : `SALA #${session.room_code}`}
              </span>
              <span style={{ fontSize: '11px', color: '#94a3b8' }}>
                {character ? `• ${character.name} (${character.className})` : '• Sem herói ativo'}
              </span>
            </div>
          ) : (
            <span style={{ fontSize: '11px', color: '#64748b' }}>
              • Nenhuma campanha ativa
            </span>
          )}
        </div>

        {/* Centro: NAVEGAÇÃO ENTRE AS TELAS (Exibida apenas quando há sessão ativa) */}
        {session && (
          <nav
            style={{
              display: 'flex',
              alignItems: 'center',
              backgroundColor: '#0c0f17',
              padding: '3px',
              borderRadius: '10px',
              border: '1px solid rgba(255, 255, 255, 0.08)',
              gap: '4px',
            }}
          >
            <button
              onClick={() => setCurrentView('lobby')}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '6px',
                padding: '6px 14px',
                borderRadius: '7px',
                fontSize: '12px',
                fontWeight: 700,
                cursor: 'pointer',
                border: currentView === 'lobby' ? '1px solid rgba(234, 179, 8, 0.5)' : '1px solid transparent',
                backgroundColor: currentView === 'lobby' ? 'rgba(234, 179, 8, 0.18)' : 'transparent',
                color: currentView === 'lobby' ? '#facc15' : '#94a3b8',
                transition: 'all 0.15s ease',
              }}
            >
              <Globe2 size={14} /> Lobby da Campanha
            </button>

            <button
              onClick={() => {
                if (!character) {
                  showToast('Crie ou selecione um personagem primeiro!', 'warning');
                  return;
                }
                setCurrentView('game');
              }}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '6px',
                padding: '6px 14px',
                borderRadius: '7px',
                fontSize: '12px',
                fontWeight: 700,
                cursor: 'pointer',
                border: currentView === 'game' ? '1px solid rgba(16, 185, 129, 0.5)' : '1px solid transparent',
                backgroundColor: currentView === 'game' ? 'rgba(16, 185, 129, 0.18)' : 'transparent',
                color: currentView === 'game' ? '#34d399' : '#94a3b8',
                transition: 'all 0.15s ease',
              }}
            >
              <Gamepad2 size={14} /> 🎮 Cena Viva (Jogo)
            </button>
          </nav>
        )}

        {/* Lado Direito: Ações de Auth, Auditoria, Reset, Status */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px', flexWrap: 'wrap' }}>
          {authUser ? (
            <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
              <span
                style={{
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '5px',
                  padding: '4px 10px',
                  borderRadius: '6px',
                  backgroundColor: 'rgba(234, 179, 8, 0.15)',
                  border: '1px solid rgba(234, 179, 8, 0.4)',
                  color: '#fde047',
                  fontSize: '11px',
                  fontWeight: 700,
                }}
              >
                <Crown size={12} />
                {authUser.name} (Host)
              </span>
              <button
                onClick={handleLogout}
                title="Encerrar sessão"
                style={{
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '4px',
                  padding: '4px 8px',
                  borderRadius: '6px',
                  backgroundColor: 'rgba(255, 255, 255, 0.05)',
                  border: '1px solid rgba(255, 255, 255, 0.15)',
                  color: '#cbd5e1',
                  fontSize: '11px',
                  cursor: 'pointer',
                }}
              >
                <LogOut size={12} /> Sair
              </button>
            </div>
          ) : (
            <button
              onClick={() => {
                setAuthModalMode('login');
                setShowAuthModal(true);
              }}
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '5px',
                padding: '5px 12px',
                borderRadius: '6px',
                backgroundColor: 'rgba(234, 179, 8, 0.15)',
                border: '1px solid var(--accent-gold)',
                color: 'var(--accent-gold)',
                fontSize: '11px',
                fontWeight: 700,
                cursor: 'pointer',
              }}
            >
              <Key size={12} /> Entrar / Conta Host
            </button>
          )}

          {session && (
            <button
              onClick={() => setShowGlobalAudit(true)}
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '5px',
                padding: '5px 10px',
                borderRadius: '6px',
                backgroundColor: 'rgba(168, 85, 247, 0.16)',
                border: '1px solid rgba(168, 85, 247, 0.4)',
                color: '#e9d5ff',
                fontSize: '11px',
                fontWeight: 700,
                cursor: 'pointer',
              }}
            >
              <Search size={13} color="#c084fc" /> 🔍 Auditoria
            </button>
          )}

          {session && (
            <button
              onClick={handleExitSession}
              title="Sair da campanha e zerar sessão"
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '5px',
                padding: '5px 10px',
                borderRadius: '6px',
                backgroundColor: '#1e293b',
                border: '1px solid #334155',
                color: '#f87171',
                fontSize: '11px',
                fontWeight: 600,
                cursor: 'pointer',
              }}
            >
              <LogOut size={13} /> Sair Sala
            </button>
          )}

          <span
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: '6px',
              padding: '4px 8px',
              borderRadius: '12px',
              backgroundColor: backendOnline ? 'rgba(16, 185, 129, 0.12)' : 'rgba(234, 179, 8, 0.12)',
              color: backendOnline ? '#34d399' : '#facc15',
              border: backendOnline ? '1px solid rgba(16, 185, 129, 0.3)' : '1px solid rgba(234, 179, 8, 0.3)',
              fontSize: '11px',
            }}
          >
            <span
              style={{
                width: '6px',
                height: '6px',
                borderRadius: '50%',
                backgroundColor: backendOnline ? '#34d399' : '#facc15',
              }}
            />
            {backendOnline ? 'Online' : 'Local'}
          </span>
        </div>
      </header>
      )}

      {/* ÁREA DE CONTEÚDO PRINCIPAL (SPA VIEWPORT) */}
      <main style={{ flex: 1, display: 'flex', flexDirection: 'column' }}>
        {/* TELA 1: LOBBY & MUNDO */}
        {currentView === 'lobby' && (
          <SessionLobby
            authUser={authUser}
            onRequestLogin={() => {
              setAuthModalMode('login');
              setShowAuthModal(true);
            }}
            onSessionReady={(sess, initialParty) => {
              handleSessionReady(sess, initialParty);
            }}
          />
        )}

        {/* TELA ADMIN: PAINEL DE CONTROLE SAAS (PORTAL EXCLUSIVO) */}
        {currentView === 'admin' && (
          <AdminPanel
            onClose={() => {
              window.location.hash = '';
              setCurrentView('lobby');
            }}
          />
        )}

        {/* TELA 2: FORJA & EVOLUÇÃO DE PERSONAGEM */}
        {currentView === 'character' && (
          <div style={{ position: 'relative', width: '100%', flex: 1 }}>
            {/* Barra de ação rápida para navegação e party */}
            <div
              style={{
                maxWidth: '1180px',
                margin: '12px auto 0',
                padding: '0 16px',
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                flexWrap: 'wrap',
                gap: '8px',
              }}
            >
              <div style={{ display: 'flex', gap: '8px', alignItems: 'center', flexWrap: 'wrap' }}>
                <button
                  onClick={() => {
                    if (isAddingPartyMember && character) {
                      setIsAddingPartyMember(false);
                      setCurrentView('game');
                    } else {
                      setCurrentView('lobby');
                    }
                  }}
                  style={{
                    display: 'inline-flex',
                    alignItems: 'center',
                    gap: '6px',
                    padding: '6px 12px',
                    backgroundColor: 'rgba(255, 255, 255, 0.05)',
                    border: '1px solid rgba(255, 255, 255, 0.15)',
                    borderRadius: '8px',
                    color: '#cbd5e1',
                    fontSize: '12px',
                    fontWeight: 600,
                    cursor: 'pointer',
                  }}
                >
                  {isAddingPartyMember && character
                    ? '← Cancelar e Voltar ao Jogo'
                    : '← Voltar ao Lobby do Mundo'}
                </button>

                {party.length > 0 && (
                  <span style={{ fontSize: '11.5px', color: '#94a3b8' }}>
                    Grupo atual: <strong style={{ color: '#38bdf8' }}>{party.length}/4 herói(s)</strong> (
                    {party.map((p) => p.name).join(', ')})
                  </span>
                )}
              </div>

              <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap' }}>
                {party.length > 0 && party.length < 4 && !isAddingPartyMember && (
                  <button
                    onClick={() => setIsAddingPartyMember(true)}
                    style={{
                      display: 'inline-flex',
                      alignItems: 'center',
                      gap: '6px',
                      padding: '6px 12px',
                      backgroundColor: 'rgba(56, 189, 248, 0.15)',
                      border: '1px solid #38bdf8',
                      borderRadius: '8px',
                      color: '#7dd3fc',
                      fontSize: '12px',
                      fontWeight: 700,
                      cursor: 'pointer',
                    }}
                  >
                    <UserPlus size={14} /> + Criar Novo Integrante ({party.length + 1}/4)
                  </button>
                )}

                <button
                  onClick={handleUseQuickTestParty}
                  style={{
                    display: 'inline-flex',
                    alignItems: 'center',
                    gap: '6px',
                    padding: '6px 14px',
                    backgroundColor: 'rgba(234, 179, 8, 0.18)',
                    border: '1px solid var(--accent-gold)',
                    borderRadius: '8px',
                    color: 'var(--accent-gold)',
                    fontSize: '12px',
                    fontWeight: 700,
                    cursor: 'pointer',
                  }}
                >
                  <Zap size={14} /> ⚡ Pular &amp; Usar Party Pronta (4 Heróis)
                </button>
              </div>
            </div>

            <CharacterCreationWizard
              key={isAddingPartyMember ? `add-${party.length + 1}` : 'main-wizard'}
              onCharacterCreated={handleCharacterCreated}
              isAddingToParty={isAddingPartyMember}
              nextPlayerNumber={party.length + 1}
            />
          </div>
        )}

        {/* TELA 3: CENA VIVA & HUD DE JOGO */}
        {currentView === 'game' && character && (
          <BaseGameHUD
            character={character}
            session={session}
            party={party}
            onSelectPartyMember={handleSelectPartyMember}
            onUpdateCharacter={handleUpdateCharacter}
            onSyncSceneAndParty={handleSyncSceneAndParty}
            onRequestAddPartyMember={handleRequestAddPartyMember}
            onResetCharacter={handleResetCharacter}
            onShowToast={showToast}
            onOpenAudit={() => setShowGlobalAudit(true)}
          />
        )}
      </main>

      {/* Modal de Auditoria Única e Canônica */}
      {showGlobalAudit && session?.campaign_id && (
        <AuditModal
          campaignId={session.campaign_id}
          onClose={() => setShowGlobalAudit(false)}
        />
      )}

      {/* Modal de Autenticação SaaS (Host / Jogador / Admin) */}
      <AuthModal
        isOpen={showAuthModal}
        initialMode={authModalMode}
        onClose={() => setShowAuthModal(false)}
        onSuccess={(user) => {
          setAuthUser(user);
          showToast(`Bem-vindo, ${user.name}! (${user.role === 'admin' ? 'Administrador' : 'Host'})`, 'success');
        }}
      />
    </div>
  );
}

export default App;
