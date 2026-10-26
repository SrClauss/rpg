import { useState } from 'react';
import type { FC } from 'react';
import type { NarrativeBeat } from '../../types/game';
import type { CharacterSheet } from '../../types/character';
import type {
  LivingSceneData,
  ChronicleEntryData,
  SceneLootItemData,
  TurnParticipantData,
} from '../../types/session';
import { TacticalBattleCanvas } from './TacticalBattleCanvas';
import {
  Users,
  Grid,
  Scroll,
  MapPin,
  CheckCircle2,
  Circle,
  Sword,
  Clock,
  BookOpen,
  Sparkles,
  Lock,
  SkipForward,
  Crosshair,
} from 'lucide-react';

interface WorldViewPanelProps {
  beat: NarrativeBeat;
  livingScene?: LivingSceneData | null;
  chronicleEntries?: ChronicleEntryData[];
  worldPrologue?: string;
  isPrologueModalOpen?: boolean;
  onTogglePrologueModal?: (open: boolean) => void;
  onStartLivingScene?: () => void;
  isStartingScene?: boolean;
  onCellClick?: (x: number, y: number) => void;
  playerPos?: { x: number; y: number };
  enemyPos?: { x: number; y: number };
  onClaimItem?: (item: SceneLootItemData) => void;
  onResolveScene?: () => void;
  onToggleCombatMode?: (combat: boolean) => void;
  isCombatMode?: boolean;
  party?: CharacterSheet[];
  activeCharacter?: CharacterSheet | null;
  selectedTargetId?: string | null;
  onSelectTarget?: (target: TurnParticipantData) => void;
  onMoveParticipant?: (xFt: number, yFt: number, actorId?: string) => void;
  onAttackTarget?: (target: TurnParticipantData, distanceFt: number) => void;
  onEndTurn?: (actorId?: string) => void;
  onFleeBattle?: () => void;
}

export const WorldViewPanel: FC<WorldViewPanelProps> = ({
  beat,
  livingScene,
  chronicleEntries = [],
  worldPrologue,
  isPrologueModalOpen,
  onTogglePrologueModal,
  onStartLivingScene,
  isStartingScene = false,
  onCellClick,
  playerPos: _playerPos = { x: 1, y: 1 },
  enemyPos: _enemyPos = { x: 4, y: 3 },
  onClaimItem,
  onResolveScene,
  onToggleCombatMode,
  isCombatMode = false,
  party = [],
  activeCharacter = null,
  selectedTargetId = null,
  onSelectTarget,
  onMoveParticipant,
  onAttackTarget,
  onEndTurn,
  onFleeBattle,
}) => {
  const [activeTab, setActiveTab] = useState<'scene' | 'chronicle' | 'tactical'>('scene');
  const [internalShowPrologue, setInternalShowPrologue] = useState<boolean>(() => {
    if (typeof window === 'undefined') return false;
    const sceneKey = livingScene?.scene_id || beat.id;
    return !sessionStorage.getItem(`prologue_seen_${sceneKey}`);
  });

  const showPrologueModal = isPrologueModalOpen !== undefined ? isPrologueModalOpen : internalShowPrologue;

  const setShowPrologueModal = (open: boolean) => {
    setInternalShowPrologue(open);
    if (onTogglePrologueModal) {
      onTogglePrologueModal(open);
    }
  };

  const [modalEntry, setModalEntry] = useState<{
    title: string;
    chapter?: number;
    text: string;
    isPrologue: boolean;
  } | null>(null);

  const effectivePrologue =
    worldPrologue ||
    livingScene?.world_prologue ||
    chronicleEntries.find(
      (e) => e.entry_type === 'LiteraryChapter' || e.scene_title.toLowerCase().includes('prólogo')
    )?.narrative_prose ||
    beat.proseContent;

  const handleOpenPrologue = (entry?: ChronicleEntryData) => {
    if (entry) {
      setModalEntry({
        title: entry.scene_title,
        chapter: entry.chapter,
        text: entry.narrative_prose,
        isPrologue: entry.scene_title.toLowerCase().includes('prólogo'),
      });
    } else {
      setModalEntry({
        title: `Contextualização da História • Ato ${beat.beatIndex}: ${livingScene?.title || beat.title}`,
        chapter: beat.beatIndex,
        text: effectivePrologue,
        isPrologue: true,
      });
    }
    setShowPrologueModal(true);
  };

  const handleClosePrologue = () => {
    const sceneKey = livingScene?.scene_id || beat.id;
    sessionStorage.setItem(`prologue_seen_${sceneKey}`, 'true');
    setShowPrologueModal(false);
  };

  const handleStartActionFromPrologue = () => {
    handleClosePrologue();
    setActiveTab('scene');
    if (!livingScene?.is_active && onStartLivingScene) {
      onStartLivingScene();
    }
  };

  const currentParticipant =
    livingScene?.participants && livingScene.participants.length > 0
      ? livingScene.participants[livingScene.current_participant_index % livingScene.participants.length]
      : null;

  const livingHostiles = (livingScene?.participants || []).filter(
    (p) => !p.is_player && p.is_hostile !== false && !p.is_dead && (p.current_hp === undefined || p.current_hp > 0)
  );
  const inActiveCombat =
    Boolean(livingScene?.is_active) &&
    (livingScene?.in_combat !== false || isCombatMode) &&
    livingHostiles.length > 0;

  return (
    <div
      style={{
        backgroundColor: 'var(--bg-card)',
        border: '1px solid var(--border-color)',
        borderRadius: '14px',
        padding: '20px',
        display: 'flex',
        flexDirection: 'column',
        gap: '14px',
        boxShadow: '0 6px 24px rgba(0, 0, 0, 0.4)',
      }}
    >
      {/* Header */}
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          borderBottom: '1px solid var(--border-color)',
          paddingBottom: '12px',
          flexWrap: 'wrap',
          gap: '10px',
        }}
      >
        <div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', flexWrap: 'wrap' }}>
            <button
              type="button"
              onClick={() => handleOpenPrologue()}
              data-testid="header-prologue-badge-btn"
              title="Clique para abrir a Contextualização da História e o Prólogo do Mundo"
              style={{
                fontSize: '11px',
                fontWeight: 700,
                padding: '3px 10px',
                borderRadius: '4px',
                backgroundColor: livingScene?.is_active
                  ? 'rgba(239, 68, 68, 0.2)'
                  : 'var(--accent-gold-bg)',
                color: livingScene?.is_active ? '#ef4444' : 'var(--accent-gold)',
                border: livingScene?.is_active
                  ? '1px solid rgba(239, 68, 68, 0.4)'
                  : '1px solid rgba(234, 179, 8, 0.3)',
                cursor: 'pointer',
                display: 'inline-flex',
                alignItems: 'center',
                gap: '5px',
                transition: 'all 0.15s ease',
              }}
            >
              {livingScene?.is_active ? '⚔️ CENA VIVA ATIVA (Ver Prólogo)' : '📜 PRÓLOGO & LIVRO DA HISTÓRIA'}
            </button>
            <h2 className="rpg-title" style={{ fontSize: '18px', color: '#fff', margin: 0 }}>
              {livingScene ? livingScene.title : beat.title}
            </h2>
          </div>
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              fontSize: '12px',
              color: 'var(--text-muted)',
              marginTop: '4px',
            }}
          >
            <MapPin size={13} color="var(--accent-gold)" />{' '}
            {livingScene ? livingScene.location : beat.location}
          </div>
        </div>

        {/* View mode toggle: Cena Viva, Livro da Crônica, Mapa Tático & Iniciativa */}
        <div style={{ display: 'flex', gap: '6px', flexWrap: 'wrap' }}>
          <button
            type="button"
            onClick={() => setActiveTab('scene')}
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: '6px',
              padding: '6px 14px',
              borderRadius: '6px',
              fontSize: '12px',
              fontWeight: 600,
              backgroundColor: activeTab === 'scene' ? 'var(--accent-gold)' : '#101318',
              color: activeTab === 'scene' ? '#000' : 'var(--text-muted)',
              border: '1px solid var(--border-color)',
              transition: 'all 0.15s',
              cursor: 'pointer',
            }}
          >
            <Sword size={14} /> {livingScene?.is_active ? 'Cena Viva' : 'História & Cena'}
          </button>
          <button
            type="button"
            onClick={() => setActiveTab('chronicle')}
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: '6px',
              padding: '6px 14px',
              borderRadius: '6px',
              fontSize: '12px',
              fontWeight: 600,
              backgroundColor: activeTab === 'chronicle' ? 'var(--accent-gold)' : '#101318',
              color: activeTab === 'chronicle' ? '#000' : 'var(--text-muted)',
              border: '1px solid var(--border-color)',
              transition: 'all 0.15s',
              cursor: 'pointer',
            }}
          >
            <BookOpen size={14} /> Livro da Crônica ({chronicleEntries.length})
          </button>
          <button
            type="button"
            onClick={() => setActiveTab('tactical')}
            data-testid="tactical-map-tab-btn"
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: '6px',
              padding: '6px 14px',
              borderRadius: '6px',
              fontSize: '12px',
              fontWeight: 700,
              backgroundColor: activeTab === 'tactical' ? 'var(--accent-gold)' : '#101318',
              color: activeTab === 'tactical' ? '#000' : '#facc15',
              border: activeTab === 'tactical' ? '1px solid var(--accent-gold)' : '1px solid rgba(234, 179, 8, 0.35)',
              transition: 'all 0.15s',
              cursor: 'pointer',
            }}
          >
            <Grid size={14} /> 🗺️ Mapa Tático &amp; Iniciativa
          </button>
          <button
            type="button"
            onClick={() => handleOpenPrologue()}
            data-testid="open-prologue-modal-btn"
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: '6px',
              padding: '6px 14px',
              borderRadius: '6px',
              fontSize: '12px',
              fontWeight: 700,
              backgroundColor: 'rgba(234, 179, 8, 0.15)',
              color: '#facc15',
              border: '1px solid rgba(234, 179, 8, 0.4)',
              transition: 'all 0.15s',
              cursor: 'pointer',
            }}
            title="Ver Prólogo Narrativo & Contexto da História"
          >
            <Scroll size={14} /> 📜 Prólogo &amp; Contexto
          </button>
        </div>
      </div>

      {/* Tab 1: Cena Viva ou Prólogo Denso de Abertura */}
      {activeTab === 'scene' && (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '14px' }}>
          {/* Se a Cena Viva NÃO estiver ativa: Apresenta a História Densa antes da ação */}
          {!livingScene?.is_active && (
            <div
              style={{
                backgroundColor: '#0c0f14',
                borderRadius: '10px',
                padding: '20px 24px',
                border: '1px solid var(--border-color)',
                display: 'flex',
                flexDirection: 'column',
                gap: '16px',
              }}
            >
              <div
                style={{
                  display: 'flex',
                  justifyContent: 'space-between',
                  alignItems: 'center',
                  flexWrap: 'wrap',
                  gap: '8px',
                }}
              >
                <div
                  style={{
                    fontSize: '11px',
                    fontWeight: 700,
                    color: 'var(--accent-gold)',
                    textTransform: 'uppercase',
                    letterSpacing: '0.08em',
                    display: 'flex',
                    alignItems: 'center',
                    gap: '6px',
                  }}
                >
                  <Sparkles size={14} /> Ambientação Densa &amp; Prólogo Literário:
                </div>
                <button
                  type="button"
                  onClick={() => handleOpenPrologue()}
                  style={{
                    display: 'inline-flex',
                    alignItems: 'center',
                    gap: '5px',
                    padding: '4px 10px',
                    borderRadius: '6px',
                    backgroundColor: 'rgba(234, 179, 8, 0.12)',
                    border: '1px solid rgba(234, 179, 8, 0.35)',
                    color: '#facc15',
                    fontSize: '11px',
                    fontWeight: 700,
                    cursor: 'pointer',
                  }}
                >
                  <Scroll size={13} /> 📖 Ler Prólogo Completo
                </button>
              </div>

              <div
                style={{
                  fontSize: '14px',
                  lineHeight: '1.8',
                  color: '#e2e8f0',
                  whiteSpace: 'pre-line',
                  fontStyle: 'normal',
                }}
              >
                {effectivePrologue || beat.proseContent}
              </div>

              {/* Botão de Destaque: Iniciar Cena Viva */}
              {onStartLivingScene && (
                <div
                  style={{
                    marginTop: '8px',
                    padding: '16px 20px',
                    backgroundColor: 'rgba(234, 179, 8, 0.05)',
                    border: '1px solid rgba(234, 179, 8, 0.3)',
                    borderRadius: '8px',
                    textAlign: 'center',
                    display: 'flex',
                    flexDirection: 'column',
                    alignItems: 'center',
                    gap: '10px',
                  }}
                >
                  <h3 className="rpg-title" style={{ fontSize: '14px', color: 'var(--accent-gold)', margin: 0 }}>
                    Pronto para Entrar em Ação?
                  </h3>
                  <p style={{ fontSize: '12px', color: 'var(--text-muted)', margin: 0, maxWidth: '540px' }}>
                    Após absorver a narrativa de introdução, inicie a <strong>Cena Viva</strong>. O Agente de Ambiente sintetizará o cenário, os NPCs e os objetivos diretamente do Esqueleto Narrativo desta história.
                  </p>
                  <button
                    disabled={isStartingScene}
                    onClick={onStartLivingScene}
                    style={{
                      marginTop: '4px',
                      padding: '12px 28px',
                      backgroundColor: isStartingScene ? '#334155' : 'var(--accent-gold)',
                      color: isStartingScene ? '#cbd5e1' : '#000',
                      fontWeight: 800,
                      fontSize: '13px',
                      textTransform: 'uppercase',
                      letterSpacing: '0.06em',
                      borderRadius: '8px',
                      boxShadow: isStartingScene ? 'none' : '0 4px 16px rgba(234, 179, 8, 0.35)',
                      cursor: isStartingScene ? 'wait' : 'pointer',
                      display: 'inline-flex',
                      alignItems: 'center',
                      gap: '8px',
                      transition: 'all 0.15s',
                    }}
                  >
                    <Sword size={16} />{' '}
                    {isStartingScene
                      ? '⏳ Sintetizando Cena Viva da História...'
                      : `⚔️ Iniciar Cena Viva: ${beat.title}`}
                  </button>
                </div>
              )}
            </div>
          )}

          {/* Se a Cena Viva ESTIVER ativa: Objetivos, Fila de Turnos e Feed Interativo */}
          {livingScene?.is_active && (
            <>
              {/* Barra de Ações Rápidas da Cena: Trava de Combate Tático e Conclusão */}
              <div
                style={{
                  display: 'flex',
                  justifyContent: 'space-between',
                  alignItems: 'center',
                  flexWrap: 'wrap',
                  gap: '8px',
                  backgroundColor: inActiveCombat ? 'rgba(127, 29, 29, 0.18)' : '#0c0f15',
                  padding: '8px 12px',
                  borderRadius: '8px',
                  border: inActiveCombat
                    ? '1px solid rgba(239, 68, 68, 0.45)'
                    : '1px solid rgba(255, 255, 255, 0.08)',
                }}
              >
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px', flexWrap: 'wrap' }}>
                  {inActiveCombat ? (
                    <>
                      <span
                        data-testid="scene-battle-lock-badge"
                        style={{
                          padding: '5px 10px',
                          borderRadius: '6px',
                          backgroundColor: 'rgba(239, 68, 68, 0.22)',
                          border: '1px solid #ef4444',
                          color: '#fca5a5',
                          fontSize: '11.5px',
                          fontWeight: 800,
                          display: 'inline-flex',
                          alignItems: 'center',
                          gap: '6px',
                        }}
                      >
                        <Lock size={13} color="#ef4444" />
                        🔒 EM BATALHA ATIVA ({livingHostiles.length} inimigo(s) vivo(s) • Derrote-os ou faça Retirada Tática CD 12)
                      </span>
                      <button
                        type="button"
                        onClick={() => setActiveTab('tactical')}
                        style={{
                          padding: '5px 11px',
                          borderRadius: '6px',
                          backgroundColor: 'rgba(234, 179, 8, 0.16)',
                          border: '1px solid #eab308',
                          color: '#fef08a',
                          fontSize: '11.5px',
                          fontWeight: 700,
                          cursor: 'pointer',
                          display: 'inline-flex',
                          alignItems: 'center',
                          gap: '5px',
                        }}
                      >
                        <Grid size={13} /> 🗺️ Ver Mapa Tático &amp; Distâncias
                      </button>
                    </>
                  ) : (
                    onToggleCombatMode && (
                      <button
                        type="button"
                        onClick={() => onToggleCombatMode(!isCombatMode)}
                        style={{
                          padding: '6px 12px',
                          borderRadius: '6px',
                          backgroundColor: isCombatMode ? 'rgba(239, 68, 68, 0.2)' : 'rgba(234, 179, 8, 0.15)',
                          border: isCombatMode ? '1px solid #ef4444' : '1px solid #facc15',
                          color: isCombatMode ? '#fca5a5' : '#fef08a',
                          fontSize: '12px',
                          fontWeight: 700,
                          cursor: 'pointer',
                          display: 'inline-flex',
                          alignItems: 'center',
                          gap: '6px',
                          transition: 'all 0.15s ease',
                        }}
                      >
                        <Sword size={14} />
                        {isCombatMode ? '🕊️ Mudar para Exploração Livre' : '⚔️ Rolar Iniciativa & Combate'}
                      </button>
                    )
                  )}
                </div>

                <div style={{ display: 'flex', alignItems: 'center', gap: '8px', flexWrap: 'wrap' }}>
                  {inActiveCombat ? (
                    onFleeBattle && (
                      <button
                        type="button"
                        onClick={onFleeBattle}
                        data-testid="scene-flee-battle-btn"
                        style={{
                          padding: '6px 12px',
                          borderRadius: '6px',
                          backgroundColor: 'rgba(245, 158, 11, 0.16)',
                          border: '1px solid #f59e0b',
                          color: '#fde047',
                          fontSize: '11.5px',
                          fontWeight: 700,
                          cursor: 'pointer',
                          display: 'inline-flex',
                          alignItems: 'center',
                          gap: '5px',
                        }}
                        title="Tentar fugir e desengajar do combate com teste de Destreza/Atletismo CD 12"
                      >
                        🏃‍♂️ Retirada Tática / Fugir (CD 12)
                      </button>
                    )
                  ) : (
                    onResolveScene && (
                      <button
                        type="button"
                        onClick={onResolveScene}
                        style={{
                          padding: '6px 14px',
                          borderRadius: '6px',
                          backgroundColor: 'rgba(16, 185, 129, 0.15)',
                          border: '1px solid #10b981',
                          color: '#6ee7b7',
                          fontSize: '12px',
                          fontWeight: 700,
                          cursor: 'pointer',
                          display: 'inline-flex',
                          alignItems: 'center',
                          gap: '6px',
                          transition: 'all 0.15s ease',
                        }}
                        title="Consolidar esta cena na Crônica e avançar para o próximo ato"
                      >
                        <CheckCircle2 size={14} /> 📜 Concluir Cena &amp; Avançar História
                      </button>
                    )
                  )}
                </div>
              </div>

              {/* Objectives Checklist & Prevenção de Loops */}
              {livingScene.objectives && livingScene.objectives.length > 0 && (
                <div
                  style={{
                    backgroundColor: '#0d1117',
                    border: '1px solid #30363d',
                    borderRadius: '8px',
                    padding: '12px 16px',
                  }}
                >
                  <div
                    style={{
                      fontSize: '11px',
                      fontWeight: 700,
                      color: 'var(--accent-gold)',
                      textTransform: 'uppercase',
                      letterSpacing: '0.05em',
                      marginBottom: '8px',
                      display: 'flex',
                      alignItems: 'center',
                      gap: '6px',
                    }}
                  >
                    <Clock size={14} /> Objetivos da Cena Viva (Prevenção de Loop):
                  </div>
                  <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                    {livingScene.objectives.map((obj) => (
                      <div
                        key={obj.id}
                        style={{
                          display: 'flex',
                          alignItems: 'center',
                          gap: '8px',
                          fontSize: '13px',
                          color: obj.completed ? '#10b981' : '#cbd5e1',
                        }}
                      >
                        {obj.completed ? (
                          <CheckCircle2 size={16} color="#10b981" />
                        ) : (
                          <Circle size={16} color="#64748b" />
                        )}
                        <span style={{ textDecoration: obj.completed ? 'line-through' : 'none' }}>
                          {obj.description}
                        </span>
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* Resumo Compacto da Ordem de Iniciativa e HP dos Inimigos & Heróis */}
              {livingScene.participants && livingScene.participants.length > 0 && (
                <div
                  data-testid="scene-initiative-summary"
                  style={{
                    backgroundColor: '#0d1118',
                    border: '1px solid #1e293b',
                    borderRadius: '10px',
                    padding: '10px 14px',
                    display: 'flex',
                    flexDirection: 'column',
                    gap: '10px',
                  }}
                >
                  <div
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'space-between',
                      flexWrap: 'wrap',
                      gap: '8px',
                    }}
                  >
                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px', flexWrap: 'wrap' }}>
                      <span
                        style={{
                          fontSize: '11px',
                          fontWeight: 800,
                          color: '#facc15',
                          textTransform: 'uppercase',
                          letterSpacing: '0.04em',
                        }}
                      >
                        🎲 Fila de Iniciativa • Rodada #{livingScene.round_number || 1}
                      </span>
                      <span
                        style={{
                          fontSize: '11.5px',
                          fontWeight: 800,
                          color: currentParticipant?.is_player ? '#fef08a' : '#fca5a5',
                          backgroundColor: '#171f2e',
                          padding: '2px 8px',
                          borderRadius: '4px',
                          border: '1px solid #2d3748',
                        }}
                      >
                        ⚡ Turno de: {currentParticipant ? currentParticipant.name : 'Jogador'} (Inic:{' '}
                        {currentParticipant?.initiative})
                      </span>
                    </div>

                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px', flexWrap: 'wrap' }}>
                      <div style={{ display: 'flex', gap: '6px', fontSize: '11px', color: '#94a3b8', fontWeight: 700 }}>
                        <span
                          style={{
                            color: currentParticipant?.actions_budget.action_used ? '#f87171' : '#34d399',
                          }}
                        >
                          Ação: {currentParticipant?.actions_budget.action_used ? 'Gasta' : 'Disp.'}
                        </span>
                        <span>•</span>
                        <span
                          style={{
                            color: currentParticipant?.actions_budget.bonus_action_used ? '#f87171' : '#fb923c',
                          }}
                        >
                          Bônus: {currentParticipant?.actions_budget.bonus_action_used ? 'Gasta' : 'Disp.'}
                        </span>
                        <span>•</span>
                        <span style={{ color: '#38bdf8' }}>
                          Mov: {currentParticipant?.actions_budget.movement_remaining_ft ?? 30}ft
                        </span>
                      </div>

                      {onEndTurn && (
                        <button
                          type="button"
                          onClick={() => onEndTurn(currentParticipant?.id)}
                          data-testid="scene-end-turn-btn"
                          style={{
                            display: 'inline-flex',
                            alignItems: 'center',
                            gap: '4px',
                            padding: '3px 10px',
                            borderRadius: '5px',
                            backgroundColor: '#eab308',
                            color: '#000',
                            border: 'none',
                            fontSize: '11px',
                            fontWeight: 800,
                            cursor: 'pointer',
                          }}
                          title="Encerrar turno atual e passar para o próximo na iniciativa"
                        >
                          <SkipForward size={12} /> ⏭️ Encerrar Turno
                        </button>
                      )}
                    </div>
                  </div>

                  {/* Mini-Cards de Iniciativa e PV em Tempo Real */}
                  <div
                    style={{
                      display: 'grid',
                      gridTemplateColumns: 'repeat(auto-fill, minmax(165px, 1fr))',
                      gap: '6px',
                    }}
                  >
                    {livingScene.participants.map((p, idx) => {
                      const isTurn =
                        idx === (livingScene.current_participant_index % livingScene.participants.length);
                      const matchedHero = p.is_player
                        ? party.find(
                            (h) =>
                              h.id === p.id ||
                              h.name.toLowerCase() === p.name.toLowerCase()
                          )
                        : undefined;
                      const maxHp = Math.max(
                        1,
                        p.max_hp ?? matchedHero?.vitals.hpMax ?? 12
                      );
                      const curHp = Math.max(
                        0,
                        p.current_hp ?? matchedHero?.vitals.hpCurrent ?? maxHp
                      );
                      const ac = p.armor_class ?? matchedHero?.vitals.armorClass ?? (p.is_player ? 15 : 13);
                      const isDead = Boolean(p.is_dead || curHp <= 0);
                      const hpPct = Math.max(0, Math.min(100, Math.round((curHp / maxHp) * 100)));
                      const initMod =
                        p.initiative_modifier ?? matchedHero?.vitals.initiativeModifier ?? 2;
                      const initRoll =
                        p.initiative_roll ?? Math.max(1, Math.min(20, p.initiative - initMod));
                      const isTarget = selectedTargetId === p.id && !p.is_player && !isDead;

                      return (
                        <div
                          key={p.id || idx}
                          onClick={() => {
                            if (!p.is_player && !isDead && onSelectTarget) {
                              onSelectTarget(p);
                            }
                          }}
                          style={{
                            backgroundColor: isDead
                              ? 'rgba(15, 23, 42, 0.35)'
                              : isTurn
                              ? 'rgba(234, 179, 8, 0.14)'
                              : '#131822',
                            border: isDead
                              ? '1px dashed #334155'
                              : isTurn
                              ? '1.5px solid #facc15'
                              : isTarget
                              ? '1.5px solid #f43f5e'
                              : '1px solid #1e293b',
                            borderRadius: '6px',
                            padding: '6px 8px',
                            opacity: isDead ? 0.5 : 1,
                            cursor: !p.is_player && !isDead ? 'pointer' : 'default',
                            display: 'flex',
                            flexDirection: 'column',
                            gap: '3px',
                          }}
                          title={
                            !p.is_player && !isDead
                              ? `Clique para mirar em ${p.name}`
                              : `${p.name} (Iniciativa ${p.initiative})`
                          }
                        >
                          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', gap: '4px' }}>
                            <span
                              style={{
                                fontSize: '11.5px',
                                fontWeight: 800,
                                color: isDead
                                  ? '#64748b'
                                  : p.is_player
                                  ? '#fef08a'
                                  : '#fca5a5',
                                textDecoration: isDead ? 'line-through' : 'none',
                                overflow: 'hidden',
                                textOverflow: 'ellipsis',
                                whiteSpace: 'nowrap',
                              }}
                            >
                              {idx + 1}º {p.name}
                            </span>
                            {isDead ? (
                              <span style={{ fontSize: '9.5px', color: '#f87171', fontWeight: 800 }}>
                                💀 0 PV
                              </span>
                            ) : isTarget ? (
                              <span style={{ fontSize: '9px', color: '#fda4af', fontWeight: 800, display: 'inline-flex', alignItems: 'center', gap: '2px' }}>
                                <Crosshair size={10} /> ALVO
                              </span>
                            ) : (
                              <span style={{ fontSize: '9.5px', color: '#94a3b8', fontFamily: 'monospace' }}>
                                CA {ac}
                              </span>
                            )}
                          </div>

                          <div style={{ fontSize: '10px', color: '#94a3b8', fontFamily: 'monospace' }}>
                            🎲 Inic: {p.initiative} ({initRoll}
                            {initMod >= 0 ? `+${initMod}` : initMod})
                          </div>

                          <div style={{ display: 'flex', alignItems: 'center', gap: '5px' }}>
                            <div
                              style={{
                                flex: 1,
                                height: '5px',
                                backgroundColor: '#090d14',
                                borderRadius: '3px',
                                overflow: 'hidden',
                              }}
                            >
                              <div
                                style={{
                                  width: `${hpPct}%`,
                                  height: '100%',
                                  backgroundColor: isDead
                                    ? '#475569'
                                    : p.is_player
                                    ? '#10b981'
                                    : '#ef4444',
                                }}
                              />
                            </div>
                            <span style={{ fontSize: '10px', fontWeight: 700, fontFamily: 'monospace', color: '#e2e8f0' }}>
                              {curHp}/{maxHp} PV
                            </span>
                          </div>
                        </div>
                      );
                    })}
                  </div>
                </div>
              )}

              {/* Narrative / Turn Interaction Feed */}
              <div
                style={{
                  backgroundColor: '#101318',
                  borderRadius: '8px',
                  padding: '16px',
                  border: '1px solid var(--border-color)',
                  lineHeight: '1.7',
                  fontSize: '14px',
                  color: '#e2e8f0',
                  maxHeight: '400px',
                  overflowY: 'auto',
                }}
              >
                {/* Banner de Acesso Rápido ao Prólogo & Lore do Mundo */}
                <div
                  style={{
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'center',
                    padding: '8px 12px',
                    marginBottom: '12px',
                    borderRadius: '6px',
                    backgroundColor: 'rgba(234, 179, 8, 0.08)',
                    border: '1px solid rgba(234, 179, 8, 0.25)',
                    flexWrap: 'wrap',
                    gap: '8px',
                  }}
                >
                  <span style={{ fontSize: '11.5px', color: 'var(--text-muted)' }}>
                    📜 <strong>Prólogo &amp; Lore do Mundo</strong> disponíveis a qualquer momento.
                  </span>
                  <button
                    type="button"
                    onClick={() => handleOpenPrologue()}
                    style={{
                      display: 'inline-flex',
                      alignItems: 'center',
                      gap: '4px',
                      padding: '3px 8px',
                      borderRadius: '4px',
                      backgroundColor: 'rgba(234, 179, 8, 0.2)',
                      border: '1px solid #eab308',
                      color: '#fef08a',
                      fontSize: '11px',
                      fontWeight: 700,
                      cursor: 'pointer',
                    }}
                  >
                    <Scroll size={12} /> Ver Prólogo &amp; Contexto
                  </button>
                </div>

                <div style={{ whiteSpace: 'pre-line', marginBottom: '12px' }}>
                  {livingScene.opening_narration || beat.proseContent}
                </div>

                {/* Living Scene Turn History with Player Action + Environment Counter-turn */}
                {livingScene.turn_history && livingScene.turn_history.length > 0 && (
                  <div
                    style={{
                      borderTop: '1px dashed #222834',
                      paddingTop: '12px',
                      display: 'flex',
                      flexDirection: 'column',
                      gap: '12px',
                    }}
                  >
                    <div style={{ fontSize: '11px', fontWeight: 700, color: 'var(--accent-gold)' }}>
                      HISTÓRICO INTERATIVO DA CENA VIVA (RODADA A RODADA):
                    </div>
                    {livingScene.turn_history.slice(-6).map((turn, idx) => (
                      <div
                        key={idx}
                        style={{
                          backgroundColor: '#0a0d12',
                          padding: '12px 14px',
                          borderRadius: '8px',
                          fontSize: '13px',
                          borderLeft: '3px solid var(--accent-gold)',
                          display: 'flex',
                          flexDirection: 'column',
                          gap: '6px',
                        }}
                      >
                        <div>
                          <span style={{ fontWeight: 700, color: '#93c5fd' }}>
                            [Turno de {turn.actor_name}]:
                          </span>{' '}
                          <span style={{ fontStyle: 'italic', color: '#cbd5e1' }}>
                            "{turn.player_input}"
                          </span>
                        </div>
                        <div style={{ color: '#f1f5f9', whiteSpace: 'pre-line' }}>
                          {turn.environment_reaction}
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </div>

              {/* Tesouros & Itens da Cena (Crescimento de Personagem) */}
              {livingScene.scene_loot && livingScene.scene_loot.length > 0 && (
                <div
                  style={{
                    backgroundColor: '#090d14',
                    border: '1px solid rgba(234, 179, 8, 0.35)',
                    borderRadius: '10px',
                    padding: '14px 16px',
                    display: 'flex',
                    flexDirection: 'column',
                    gap: '10px',
                  }}
                >
                  {/* SEÇÃO DE ITENS E DESCOBERTAS ORGÂNICAS */}
                  {(() => {
                    const revealedLoot = livingScene.scene_loot.filter((item) => !item.is_hidden);
                    const hiddenCount = livingScene.scene_loot.filter((item) => item.is_hidden && !item.is_claimed).length;

                    if (revealedLoot.length === 0 && hiddenCount === 0) return null;

                    return (
                      <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
                        {/* Pista sensorial se houver itens ocultos */}
                        {hiddenCount > 0 && (
                          <div
                            style={{
                              padding: '10px 14px',
                              borderRadius: '8px',
                              backgroundColor: 'rgba(56, 189, 248, 0.08)',
                              border: '1px dashed rgba(56, 189, 248, 0.35)',
                              display: 'flex',
                              alignItems: 'center',
                              justifyContent: 'space-between',
                              gap: '10px',
                            }}
                          >
                            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                              <Sparkles size={16} color="#38bdf8" />
                              <span style={{ fontSize: '12px', color: '#bae6fd' }}>
                                <strong>Pistas no Cenário:</strong> O ambiente guarda segredos e reflexos ocultos entre as sombras e fendas. Ações de <em>Investigação</em>, <em>Percepção</em> ou examinar inimigos podem revelar tesouros!
                              </span>
                            </div>
                            <span style={{ fontSize: '11px', color: '#64748b', fontFamily: 'monospace', whiteSpace: 'nowrap' }}>
                              ({hiddenCount} segredo{hiddenCount > 1 ? 's' : ''} a descobrir)
                            </span>
                          </div>
                        )}

                        {/* Itens revelados para coleta */}
                        {revealedLoot.length > 0 && (
                          <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                            <div style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '12px', fontWeight: 800, color: '#facc15' }}>
                              <Sparkles size={14} /> ✨ Descobertas &amp; Saques Revelados:
                            </div>
                            <div
                              style={{
                                display: 'grid',
                                gridTemplateColumns: 'repeat(auto-fill, minmax(280px, 1fr))',
                                gap: '10px',
                              }}
                            >
                              {revealedLoot.map((item) => (
                                <div
                                  key={item.id}
                                  style={{
                                    backgroundColor: item.is_claimed ? 'rgba(255, 255, 255, 0.02)' : '#10141d',
                                    border: item.is_claimed ? '1px solid #1e293b' : '1px solid rgba(234, 179, 8, 0.4)',
                                    borderRadius: '8px',
                                    padding: '12px',
                                    display: 'flex',
                                    flexDirection: 'column',
                                    justifyContent: 'space-between',
                                    gap: '8px',
                                    opacity: item.is_claimed ? 0.6 : 1,
                                  }}
                                >
                                  <div>
                                    <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: '6px' }}>
                                      <strong style={{ fontSize: '13px', color: item.is_claimed ? '#94a3b8' : '#facc15' }}>
                                        {item.name}
                                      </strong>
                                      <span
                                        style={{
                                          fontSize: '10px',
                                          fontWeight: 700,
                                          padding: '2px 6px',
                                          borderRadius: '4px',
                                          backgroundColor:
                                            item.rarity === 'Muito Raro'
                                              ? 'rgba(168, 85, 247, 0.2)'
                                              : item.rarity === 'Raro'
                                              ? 'rgba(56, 189, 248, 0.2)'
                                              : 'rgba(234, 179, 8, 0.15)',
                                          color:
                                            item.rarity === 'Muito Raro'
                                              ? '#c084fc'
                                              : item.rarity === 'Raro'
                                              ? '#38bdf8'
                                              : '#facc15',
                                        }}
                                      >
                                        {item.rarity} • {item.item_type}
                                      </span>
                                    </div>
                                    <p style={{ fontSize: '12px', color: '#cbd5e1', margin: '6px 0 4px', lineHeight: 1.4 }}>
                                      {item.description}
                                    </p>
                                    {item.mechanical_bonus && (
                                      <div style={{ fontSize: '11px', color: '#34d399', fontWeight: 600 }}>
                                        ⚡ Bônus: {item.mechanical_bonus}
                                      </div>
                                    )}
                                    <div style={{ fontSize: '11px', color: '#64748b', fontStyle: 'italic', marginTop: '2px' }}>
                                      📍 Local: {item.location_hint}
                                    </div>
                                  </div>

                                  <div style={{ paddingTop: '6px', borderTop: '1px solid rgba(255, 255, 255, 0.06)' }}>
                                    {item.is_claimed ? (
                                      <span style={{ fontSize: '11px', color: '#10b981', fontWeight: 700, display: 'flex', alignItems: 'center', gap: '4px' }}>
                                        <CheckCircle2 size={13} /> Coletado por {item.claimed_by || 'Jogador'}
                                      </span>
                                    ) : (
                                      <button
                                        type="button"
                                        onClick={() => onClaimItem?.(item)}
                                        style={{
                                          width: '100%',
                                          padding: '6px 12px',
                                          backgroundColor: 'rgba(234, 179, 8, 0.2)',
                                          border: '1px solid var(--accent-gold)',
                                          borderRadius: '6px',
                                          color: 'var(--accent-gold)',
                                          fontSize: '12px',
                                          fontWeight: 700,
                                          cursor: 'pointer',
                                          display: 'flex',
                                          alignItems: 'center',
                                          justifyContent: 'center',
                                          gap: '6px',
                                          transition: 'all 0.15s ease',
                                        }}
                                      >
                                        🎒 Coletar / Reivindicar Item
                                      </button>
                                    )}
                                  </div>
                                </div>
                              ))}
                            </div>
                          </div>
                        )}
                      </div>
                    );
                  })()}
                </div>
              )}
            </>
          )}

          {/* Active NPCs / Entities */}
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              flexWrap: 'wrap',
              gap: '8px',
              padding: '4px 0',
            }}
          >
            <span
              style={{
                fontSize: '11px',
                fontWeight: 600,
                color: 'var(--text-faint)',
                display: 'flex',
                alignItems: 'center',
                gap: '4px',
              }}
            >
              <Users size={13} /> Entidades no Ambiente:
            </span>
            {(livingScene?.active_npcs && livingScene.active_npcs.length > 0
              ? livingScene.active_npcs
              : beat.activeNpcs
            ).map((npc, idx) => (
              <span
                key={idx}
                style={{
                  fontSize: '11px',
                  backgroundColor: '#1c222d',
                  color: 'var(--accent-sky)',
                  padding: '2px 8px',
                  borderRadius: '4px',
                  border: '1px solid #2d3748',
                }}
              >
                {npc}
              </span>
            ))}
          </div>
        </div>
      )}

      {/* Tab 2: Livro da Crônica (Append-Only Log Geral) */}
      {activeTab === 'chronicle' && (
        <div
          style={{
            backgroundColor: '#101318',
            borderRadius: '8px',
            padding: '16px',
            border: '1px solid var(--border-color)',
            maxHeight: '520px',
            overflowY: 'auto',
            display: 'flex',
            flexDirection: 'column',
            gap: '14px',
          }}
        >
          <div
            style={{
              fontSize: '12px',
              fontWeight: 700,
              color: 'var(--accent-gold)',
              textTransform: 'uppercase',
              letterSpacing: '0.05em',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
            }}
          >
            <Scroll size={14} /> Livro da Crônica (Log Geral Rastreável):
          </div>

          {chronicleEntries.length === 0 ? (
            <div style={{ color: 'var(--text-muted)', fontSize: '13px', fontStyle: 'italic' }}>
              Nenhum capítulo consolidado na crônica ainda.
            </div>
          ) : (
            chronicleEntries.map((entry) => {
              const isPrologue = entry.scene_title.toLowerCase().includes('prólogo');
              return (
                <div
                  key={entry.event_id}
                  onClick={() => handleOpenPrologue(entry)}
                  data-testid={`chronicle-entry-${entry.event_id}`}
                  style={{
                    backgroundColor: isPrologue ? '#0f141f' : '#0a0d12',
                    border: isPrologue ? '1.5px solid rgba(234, 179, 8, 0.4)' : '1px solid #1e293b',
                    borderRadius: '8px',
                    padding: '14px',
                    cursor: 'pointer',
                    transition: 'all 0.15s ease',
                  }}
                  title="Clique para ler este capítulo ou prólogo completo no leitor modal"
                >
                  <div
                    style={{
                      display: 'flex',
                      justifyContent: 'space-between',
                      alignItems: 'center',
                      marginBottom: '8px',
                      borderBottom: '1px solid #1e293b',
                      paddingBottom: '6px',
                      flexWrap: 'wrap',
                      gap: '6px',
                    }}
                  >
                    <span style={{ fontSize: '13px', fontWeight: 700, color: isPrologue ? '#facc15' : 'var(--accent-gold)' }}>
                      [{entry.event_id}] {entry.scene_title}
                    </span>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                      <span
                        style={{
                          fontSize: '10px',
                          backgroundColor: '#1e293b',
                          color: 'var(--text-faint)',
                          padding: '2px 6px',
                          borderRadius: '4px',
                        }}
                      >
                        Cap. {entry.chapter} • {entry.entry_type}
                      </span>
                      <button
                        type="button"
                        onClick={(e) => {
                          e.stopPropagation();
                          handleOpenPrologue(entry);
                        }}
                        style={{
                          display: 'inline-flex',
                          alignItems: 'center',
                          gap: '4px',
                          backgroundColor: isPrologue ? 'rgba(234, 179, 8, 0.2)' : 'rgba(56, 189, 248, 0.15)',
                          color: isPrologue ? '#fef08a' : '#7dd3fc',
                          border: isPrologue ? '1px solid #eab308' : '1px solid rgba(56, 189, 248, 0.35)',
                          padding: '2px 8px',
                          borderRadius: '4px',
                          fontSize: '10.5px',
                          fontWeight: 700,
                          cursor: 'pointer',
                        }}
                      >
                        <BookOpen size={11} /> 📖 Ler na Íntegra
                      </button>
                    </div>
                  </div>
                  <div
                    style={{
                      fontSize: '13px',
                      lineHeight: '1.7',
                      color: '#e2e8f0',
                      whiteSpace: 'pre-line',
                    }}
                  >
                    {entry.narrative_prose}
                  </div>
                {entry.involved_entities.length > 0 && (
                  <div
                    style={{
                      display: 'flex',
                      gap: '4px',
                      marginTop: '8px',
                      fontSize: '10px',
                      color: 'var(--text-faint)',
                    }}
                  >
                    <span>Entidades:</span>
                    {entry.involved_entities.map((ent, i) => (
                      <span
                        key={i}
                        style={{
                          backgroundColor: '#1c222d',
                          color: '#38bdf8',
                          padding: '1px 5px',
                          borderRadius: '3px',
                        }}
                      >
                        {ent}
                      </span>
                    ))}
                  </div>
                )}
              </div>
            );
          }))}
        </div>
      )}

      {/* Tab 3: Mapa Tático & Iniciativa (60ft x 40ft) */}
      {activeTab === 'tactical' && (
        <TacticalBattleCanvas
          livingScene={livingScene}
          party={party}
          activeCharacter={activeCharacter}
          selectedTargetId={selectedTargetId}
          onSelectTarget={onSelectTarget}
          onMoveParticipant={(xFt, yFt, actorId) => {
            if (onMoveParticipant) {
              onMoveParticipant(xFt, yFt, actorId);
            } else if (onCellClick) {
              onCellClick(xFt, yFt);
            }
          }}
          onAttackTarget={onAttackTarget}
          onEndTurn={onEndTurn}
          onFleeBattle={onFleeBattle}
          isCombatMode={isCombatMode}
        />
      )}

      {/* MODAL DE CONTEXTUALIZAÇÃO NARRATIVA & PRÓLOGO DO ATO */}
      {showPrologueModal && (
        <div
          className="modal-backdrop-fade"
          data-testid="prologue-modal"
          style={{
            position: 'fixed',
            inset: 0,
            zIndex: 9999,
            backgroundColor: 'rgba(5, 5, 10, 0.88)',
            backdropFilter: 'blur(8px)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            padding: '20px',
          }}
        >
          <div
            className="modal-content-fade"
            style={{
              width: '100%',
              maxWidth: '880px',
              maxHeight: '90vh',
              overflowY: 'auto',
              backgroundColor: '#0c0f17',
              borderRadius: '16px',
              border: '1.5px solid rgba(234, 179, 8, 0.4)',
              boxShadow: '0 25px 50px -12px rgba(0, 0, 0, 0.8)',
              padding: '28px',
              position: 'relative',
              display: 'flex',
              flexDirection: 'column',
              gap: '20px',
            }}
          >
            <button
              type="button"
              onClick={handleClosePrologue}
              data-testid="close-prologue-btn"
              style={{
                position: 'absolute',
                top: '16px',
                right: '16px',
                background: 'rgba(255, 255, 255, 0.1)',
                border: 'none',
                color: '#cbd5e1',
                width: '32px',
                height: '32px',
                borderRadius: '50%',
                cursor: 'pointer',
                fontWeight: 800,
                fontSize: '14px',
              }}
              title="Fechar Prólogo"
            >
              ✕
            </button>

            <div>
              <div
                style={{
                  fontSize: '11px',
                  fontWeight: 800,
                  color: 'var(--accent-gold)',
                  textTransform: 'uppercase',
                  letterSpacing: '0.1em',
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  marginBottom: '6px',
                }}
              >
                <Scroll size={14} />{' '}
                {modalEntry?.isPrologue
                  ? 'Contextualização da História • Prólogo & Worldbuilding do Mundo'
                  : 'Livro da Crônica • Registro Histórico'}{' '}
                • Ato {modalEntry?.chapter || beat.beatIndex}
              </div>
              <h2 className="rpg-title" style={{ fontSize: '22px', color: '#fff', margin: 0 }}>
                {modalEntry?.title || `${livingScene ? livingScene.title : beat.title}`}
              </h2>
            </div>

            {/* Texto Literário do Prólogo ou Capítulo */}
            <div
              style={{
                backgroundColor: 'rgba(255, 255, 255, 0.03)',
                padding: '22px',
                borderRadius: '10px',
                border: '1px solid rgba(255, 255, 255, 0.08)',
                fontSize: '14.5px',
                lineHeight: '1.85',
                color: '#e2e8f0',
                whiteSpace: 'pre-line',
                maxHeight: '420px',
                overflowY: 'auto',
                fontFamily: 'serif',
              }}
            >
              {modalEntry?.text || effectivePrologue || beat.proseContent || 'A comitiva de heróis se depara com o momento crucial desta saga...'}
            </div>

            {/* O Que os Personagens Sabem vs Mistério */}
            <div
              style={{
                display: 'grid',
                gridTemplateColumns: 'repeat(auto-fit, minmax(320px, 1fr))',
                gap: '12px',
              }}
            >
              <div
                style={{
                  backgroundColor: 'rgba(56, 189, 248, 0.06)',
                  border: '1px solid rgba(56, 189, 248, 0.25)',
                  borderRadius: '10px',
                  padding: '14px 16px',
                }}
              >
                <div style={{ fontSize: '12px', fontWeight: 700, color: '#38bdf8', marginBottom: '4px', display: 'flex', alignItems: 'center', gap: '6px' }}>
                  <MapPin size={14} /> O Que os Heróis Sabem Neste Momento:
                </div>
                <div style={{ fontSize: '12.5px', color: '#cbd5e1', lineHeight: '1.6' }}>
                  {livingScene?.location
                    ? `A comitiva encontra-se em "${livingScene.location}". ${
                        livingScene.objectives && livingScene.objectives.length > 0
                          ? `Marcos imediatos: ${livingScene.objectives.map((o) => o.description).join('; ')}.`
                          : 'Seus sentidos estão alertas para pistas, perigos ocultos e contatos imediatos no cenário.'
                      }`
                    : `Os heróis cruzaram o limiar do Ato ${beat.beatIndex} em "${beat.title}", atentos às ameaças do continente e às primeiras pistas da crise.`}
                </div>
              </div>

              <div
                style={{
                  backgroundColor: 'rgba(234, 179, 8, 0.06)',
                  border: '1px solid rgba(234, 179, 8, 0.25)',
                  borderRadius: '10px',
                  padding: '14px 16px',
                }}
              >
                <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--accent-gold)', marginBottom: '4px', display: 'flex', alignItems: 'center', gap: '6px' }}>
                  <Sparkles size={14} /> Mistério &amp; Revelações Posteriores:
                </div>
                <div style={{ fontSize: '12.5px', color: '#cbd5e1', lineHeight: '1.6' }}>
                  As maquinações mais profundas que operam por trás desta trama ainda jazem encobertas pelas névoas do destino. Os verdadeiros mandantes, pactos e segredos ancestrais do reino serão desvendados aos poucos através de suas escolhas, investigações e combates ao longo dos capítulos.
                </div>
              </div>
            </div>

            <div style={{ display: 'flex', justifyContent: 'flex-end', marginTop: '6px', gap: '10px' }}>
              <button
                type="button"
                onClick={handleClosePrologue}
                style={{
                  padding: '10px 18px',
                  backgroundColor: 'transparent',
                  border: '1px solid rgba(255, 255, 255, 0.2)',
                  borderRadius: '8px',
                  color: '#94a3b8',
                  fontSize: '13px',
                  fontWeight: 600,
                  cursor: 'pointer',
                }}
              >
                Fechar Leitor
              </button>
              <button
                type="button"
                onClick={handleStartActionFromPrologue}
                data-testid="start-action-from-prologue-btn"
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '8px',
                  padding: '12px 24px',
                  backgroundColor: 'var(--accent-gold)',
                  border: 'none',
                  borderRadius: '8px',
                  color: '#000',
                  fontWeight: 800,
                  fontSize: '14px',
                  cursor: 'pointer',
                  boxShadow: '0 0 15px rgba(234, 179, 8, 0.35)',
                }}
              >
                <Sword size={16} /> Entrar na Cena &amp; Agir
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
