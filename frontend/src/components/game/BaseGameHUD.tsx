import { useState, useEffect, useRef } from 'react';
import type { FC, ReactNode } from 'react';
import type { CharacterSheet } from '../../types/character';
import type {
  ActionEconomyBudget,
  NarrativeBeat,
  SheetActionOption,
  ContextualScenarioOption,
  FreeformTextActionSubmission,
} from '../../types/game';
import type {
  CampaignSession,
  ChronicleEntryData,
  LivingSceneData,
  SceneLootItemData,
  PlayerSlot,
} from '../../types/session';
import { VitalsAndAttributesPanel } from './VitalsAndAttributesPanel';
import { WorldViewPanel } from './WorldViewPanel';
import { TurnPossibilitiesConsole } from './TurnPossibilitiesConsole';
import { ClassHUDDispatcher } from './classes/ClassHUDDispatcher';
import { DND_CLASSES } from '../character-creation/constants';
import { calculateAbilityModifier } from '../../types/character';
import {
  RefreshCw,
  Users,
  Wifi,
  Monitor,
  Copy,
  Check,
  Plus,
  Sparkles,
  Scroll,
} from 'lucide-react';
import { PartyChatWidget } from './PartyChatWidget';
import type { TurnParticipantData } from '../../types/session';
import {
  connectCampaignWebSocket,
  getCampaignChronicle,
  getCampaignState,
  getCampaignSlots,
  startLivingScene,
  claimSceneItem,
  submitCampaignTurn,
  submitTurnActionToBackend,
  resolveScene,
  moveSceneParticipant,
  endSceneTurn,
} from '../../services/api';

interface BaseGameHUDProps {
  character: CharacterSheet;
  session?: CampaignSession | null;
  party?: CharacterSheet[];
  onSelectPartyMember?: (char: CharacterSheet) => void;
  onAddPartyMember?: (char: CharacterSheet) => void;
  onUpdateCharacter?: (char: CharacterSheet) => void;
  onSyncSceneAndParty?: (scene?: LivingSceneData | null, playerEntities?: Array<Record<string, unknown>>) => void;
  onMoveParticipant?: (xFt: number, yFt: number, actorId?: string) => Promise<LivingSceneData | null | void> | void;
  onEndTurn?: (actorId?: string) => Promise<LivingSceneData | null | void> | void;
  onRequestAddPartyMember?: () => void;
  onResetCharacter: () => void;
  onExitSession?: () => void;
  onShowToast?: (text: string, type?: 'info' | 'success' | 'warning') => void;
  onOpenAudit?: () => void;
  classSpecificPanel?: ReactNode;
}

export const BaseGameHUD: FC<BaseGameHUDProps> = ({
  character: initialCharacter,
  session,
  party,
  onSelectPartyMember,
  onAddPartyMember: _onAddPartyMember,
  onUpdateCharacter,
  onSyncSceneAndParty,
  onMoveParticipant: externalMoveParticipant,
  onEndTurn: externalEndTurn,
  onRequestAddPartyMember,
  onResetCharacter,
  onExitSession,
  onShowToast,
  onOpenAudit,
  classSpecificPanel,
}) => {
  const [character, setCharacter] = useState<CharacterSheet>(initialCharacter);

  useEffect(() => {
    setCharacter(initialCharacter);
  }, [initialCharacter]);

  // Turn budget state
  const [budget, setBudget] = useState<ActionEconomyBudget>({
    actionAvailable: true,
    bonusActionAvailable: true,
    reactionAvailable: true,
    movementTotalFeet: character.vitals.speedFeet || 30,
    movementRemainingFeet: character.vitals.speedFeet || 30,
  });

  // Action log for the turn
  const [actionLog, setActionLog] = useState<string[]>([]);

  // Recent freeform submission
  const [recentSubmission, setRecentSubmission] =
    useState<FreeformTextActionSubmission | null>(null);

  // Campaign & Living Scene real-time state
  const [livingScene, setLivingScene] = useState<LivingSceneData | null>(null);
  const [selectedTargetId, setSelectedTargetId] = useState<string | null>(null);
  const [isStartingScene, setIsStartingScene] = useState<boolean>(false);
  const [chronicleEntries, setChronicleEntries] = useState<ChronicleEntryData[]>([]);
  const [wsConnected, setWsConnected] = useState<boolean>(false);
  const [playerCount, setPlayerCount] = useState<number>(session?.player_ids?.length ?? 1);
  const wsRef = useRef<WebSocket | null>(null);
  // Modo Combate Tático vs Modo Exploração Livre
  const livingHostilesCount =
    livingScene?.participants?.filter(
      (p) => !p.is_player && p.is_hostile !== false && !p.is_dead && (p.current_hp === undefined || p.current_hp > 0)
    ).length ?? 0;

  const hasHostiles =
    livingHostilesCount > 0 ||
    ((livingScene?.participants?.length ?? 0) === 0 && (livingScene?.active_npcs?.length ?? 0) > 0);

  const [isCombatMode, setIsCombatMode] = useState<boolean>(false);

  // Sincroniza cena viva, orçamento de ações, HP da party e o herói ativo da iniciativa
  const syncSceneState = (
    sc: LivingSceneData | null | undefined,
    playerEntities?: Array<Record<string, unknown>>
  ) => {
    if (!sc) return;
    setLivingScene(sc);

    const hostilesAlive = (sc.participants || []).some(
      (p) => !p.is_player && p.is_hostile !== false && !p.is_dead && (p.current_hp === undefined || p.current_hp > 0)
    );
    if (sc.in_combat !== undefined) {
      setIsCombatMode(sc.in_combat && hostilesAlive);
    } else if (hostilesAlive) {
      setIsCombatMode(true);
    }

    if (sc.participants && sc.participants.length > 0) {
      const idx = (sc.current_participant_index ?? 0) % sc.participants.length;

      const activePart = sc.participants[idx];
      if (activePart?.actions_budget) {
        setBudget((prev) => ({
          actionAvailable: !activePart.actions_budget.action_used,
          bonusActionAvailable: !activePart.actions_budget.bonus_action_used,
          reactionAvailable: !activePart.actions_budget.reaction_used,
          movementTotalFeet: prev.movementTotalFeet || 30,
          movementRemainingFeet: activePart.actions_budget.movement_remaining_ft ?? 30,
        }));
      }

      // Seleciona automaticamente o primeiro inimigo vivo se o alvo atual morreu
      const validTarget = sc.participants.find(
        (p) =>
          p.id === selectedTargetId &&
          !p.is_player &&
          !p.is_dead &&
          (p.current_hp === undefined || p.current_hp > 0)
      );
      if (!validTarget) {
        const firstLivingEnemy = sc.participants.find(
          (p) => !p.is_player && p.is_hostile !== false && !p.is_dead && (p.current_hp === undefined || p.current_hp > 0)
        );
        setSelectedTargetId(firstLivingEnemy ? firstLivingEnemy.id : null);
      }

      // Se o participante da vez na iniciativa for um jogador da party, alterna para ele automaticamente
      if (activePart?.is_player && party && party.length > 0) {
        const matchedHero = party.find(
          (hero) =>
            hero.id === activePart.id ||
            hero.name.toLowerCase() === activePart.name.toLowerCase()
        );
        if (matchedHero) {
          const updatedHero: CharacterSheet =
            activePart.current_hp !== undefined
              ? {
                  ...matchedHero,
                  vitals: {
                    ...matchedHero.vitals,
                    hpCurrent: Math.max(0, activePart.current_hp),
                    hpMax: activePart.max_hp ?? matchedHero.vitals.hpMax,
                  },
                }
              : matchedHero;
          setCharacter(updatedHero);
          onSelectPartyMember?.(updatedHero);
        }
      } else {
        // Sincroniza HP do herói ativo local caso tenha sofrido dano
        const myParticipant = sc.participants.find(
          (p) =>
            p.is_player &&
            (p.id === character.id || p.name.toLowerCase() === character.name.toLowerCase())
        );
        if (myParticipant && myParticipant.current_hp !== undefined) {
          setCharacter((prev) => ({
            ...prev,
            vitals: {
              ...prev.vitals,
              hpCurrent: Math.max(0, myParticipant.current_hp!),
              hpMax: myParticipant.max_hp ?? prev.vitals.hpMax,
            },
          }));
        }
      }
    }

    onSyncSceneAndParty?.(sc, playerEntities);
  };

  const handleToggleCombatMode = (active: boolean) => {
    if (!active && livingHostilesCount > 0 && livingScene?.in_combat !== false) {
      onShowToast?.(
        '🔒 Batalha em andamento! Derrote os inimigos ou execute Retirada Tática (CD 12) para sair do combate.',
        'warning'
      );
      return;
    }
    setIsCombatMode(active);
    if (active) {
      handleResetTurnBudget();
      setActionLog((prev) => [
        ...prev,
        `⚔️ [INICIATIVA DE COMBATE] Modo de Combate Tático Ativado! Ação, Ação Bônus e Reação restauradas para a batalha.`,
      ]);
    }
  };

  // Modo Um Terminal (Presencial) e Slots de Jogadores
  const [isSingleTerminal, setIsSingleTerminal] = useState<boolean>(
    session?.is_single_terminal ?? false
  );
  const [playerSlots, setPlayerSlots] = useState<PlayerSlot[]>(session?.player_slots || []);
  const [copiedUuid, setCopiedUuid] = useState<boolean>(false);

  // Posição no Grid Tático e Movimento
  const [playerPos, setPlayerPos] = useState<{ x: number; y: number }>({ x: 12, y: 20 });

  useEffect(() => {
    if (session?.player_slots && session.player_slots.length > 0) {
      setPlayerSlots(session.player_slots);
    }
  }, [session?.player_slots]);

  useEffect(() => {
    if (session?.campaign_id) {
      getCampaignSlots(session.campaign_id)
        .then((res) => {
          if (res.player_slots && res.player_slots.length > 0) {
            setPlayerSlots(res.player_slots);
          }
        })
        .catch((e) => console.warn('Erro ao obter slots:', e));
    }
  }, [session?.campaign_id]);

  // Ações de cenário consumidas localmente de imediato
  const [localConsumedActions, setLocalConsumedActions] = useState<Set<string>>(new Set());

  // Hotkeys Hot-Seat: Teclas '1', '2', '3', '4' alternam o herói ativo da comitiva instantaneamente
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      const activeEl = document.activeElement;
      const tagName = activeEl?.tagName?.toLowerCase();
      if (tagName === 'input' || tagName === 'textarea' || (activeEl as HTMLElement)?.isContentEditable) {
        return;
      }
      if (['1', '2', '3', '4'].includes(e.key) && party && party.length > 0) {
        const index = parseInt(e.key, 10) - 1;
        if (index < party.length && onSelectPartyMember) {
          const target = party[index];
          onSelectPartyMember(target);
          setCharacter(target);
          onShowToast?.(`Turno alternado para [${index + 1}] ${target.name}!`, 'info');
        }
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [party, onSelectPartyMember, onShowToast]);

  const getActiveActorId = (overrideActorId?: string): string => {
    if (overrideActorId) return overrideActorId;
    if (livingScene?.participants && livingScene.participants.length > 0) {
      const currentP =
        livingScene.participants[
          livingScene.current_participant_index % livingScene.participants.length
        ];
      if (currentP?.is_player) return currentP.id;
      const matchedP = livingScene.participants.find(
        (p) =>
          p.is_player &&
          (p.id === character.id || p.name.toLowerCase() === character.name.toLowerCase())
      );
      if (matchedP) return matchedP.id;
    }
    return character.id || session?.player_id || 'player-1';
  };

  const handleMoveParticipant = async (xFt: number, yFt: number, actorId?: string) => {
    const effectiveActorId = getActiveActorId(actorId);

    if (externalMoveParticipant) {
      const updatedScene = await externalMoveParticipant(xFt, yFt, effectiveActorId);
      if (updatedScene) {
        syncSceneState(updatedScene);
        return;
      }
    }

    if (session?.campaign_id) {
      try {
        const res = await moveSceneParticipant(session.campaign_id, {
          actor_id: effectiveActorId,
          x: xFt,
          y: yFt,
        });
        const nextScene = res.active_scene || res.scene;
        if (nextScene) {
          syncSceneState(nextScene, res.player_entities);
        } else {
          const st = await getCampaignState(session.campaign_id);
          if (st.active_scene) {
            syncSceneState(st.active_scene as LivingSceneData, st.player_entities as any);
          }
        }
        const usedFt = res.movement_used_ft ?? 5;
        setActionLog((prev) => [
          ...prev,
          `📍 [MOVIMENTO TÁTICO] ${character.name} deslocou-se para (${xFt}ft, ${yFt}ft) [-${usedFt}ft].`,
        ]);
        onShowToast?.(`Moveu para (${xFt}ft, ${yFt}ft)`, 'info');
        return;
      } catch (err: any) {
        onShowToast?.(err.message || 'Movimento fora do alcance disponível neste turno.', 'warning');
      }
    }

    // Fallback local caso offline
    const curPos =
      livingScene?.participants?.find((p) => p.id === effectiveActorId)?.position || [
        playerPos.x,
        playerPos.y,
      ];
    const distFt = Math.round(Math.hypot(xFt - curPos[0], yFt - curPos[1]));
    if (distFt === 0) return;
    if (budget.movementRemainingFeet < distFt) {
      onShowToast?.(
        `Deslocamento insuficiente! Necessário ${distFt}ft, disponível: ${budget.movementRemainingFeet}ft`,
        'warning'
      );
      return;
    }
    setPlayerPos({ x: xFt, y: yFt });
    const nextRem = Math.max(0, budget.movementRemainingFeet - distFt);
    setBudget((b) => ({
      ...b,
      movementRemainingFeet: nextRem,
    }));
    setLivingScene((prev) => {
      if (!prev) return prev;
      return {
        ...prev,
        participants: prev.participants.map((p) =>
          p.id === effectiveActorId || p.name.toLowerCase() === character.name.toLowerCase()
            ? {
                ...p,
                position: [xFt, yFt],
                actions_budget: {
                  ...p.actions_budget,
                  movement_remaining_ft: nextRem,
                },
              }
            : p
        ),
      };
    });
    setActionLog((prev) => [
      ...prev,
      `📍 [MOVIMENTO] ${character.name} deslocou-se para (${xFt}ft, ${yFt}ft) [${distFt}ft consumidos, ${nextRem}ft restantes].`,
    ]);
    onShowToast?.(`Moveu para (${xFt}ft, ${yFt}ft) (-${distFt}ft)`, 'info');
  };

  const handleEndTurn = async (actorId?: string) => {
    const effectiveActorId = getActiveActorId(actorId);

    if (externalEndTurn) {
      const updatedScene = await externalEndTurn(effectiveActorId);
      if (updatedScene) {
        syncSceneState(updatedScene);
        return;
      }
    }

    if (session?.campaign_id) {
      try {
        const res = await endSceneTurn(session.campaign_id, {
          actor_id: effectiveActorId,
        });
        const nextScene = res.active_scene || res.scene;
        if (nextScene) {
          syncSceneState(nextScene, res.player_entities);
          const nextIdx =
            (nextScene.current_participant_index ?? 0) %
            Math.max(1, nextScene.participants.length);
          const nextP = nextScene.participants[nextIdx];
          if (res.narrative_reaction) {
            setActionLog((prev) => [...prev, `⚔️ [FIM DE TURNO] ${res.narrative_reaction}`]);
          } else if (res.combat_log && res.combat_log.length > 0) {
            setActionLog((prev) => [...prev, ...res.combat_log!.map((l) => `⚔️ ${l}`)]);
          }
          onShowToast?.(
            `⏭️ Turno encerrado! Agora é a vez de ${nextP?.name || 'Próximo Participante'} (Rodada #${
              nextScene.round_number || 1
            })`,
            'info'
          );
          return;
        } else {
          const st = await getCampaignState(session.campaign_id);
          if (st.active_scene) {
            syncSceneState(st.active_scene as LivingSceneData, st.player_entities as any);
            return;
          }
        }
      } catch (e) {
        console.warn('Fallback ao encerrar turno:', e);
      }
    }

    handleResetTurnBudget();
    onShowToast?.('Turno encerrado e orçamento renovado!', 'info');
  };

  const handleCellClick = (x: number, y: number) => {
    handleMoveParticipant(x, y);
  };

  const handleCopyUuid = () => {
    if (!session?.campaign_id) return;
    navigator.clipboard.writeText(session.campaign_id);
    setCopiedUuid(true);
    setTimeout(() => setCopiedUuid(false), 2000);
  };

  useEffect(() => {
    if (hasHostiles) {
      setIsCombatMode(true);
    }
  }, [hasHostiles]);

  // Active Beat data (100% dynamic from campaign session & chronicle)
  const [activeBeat, setActiveBeat] = useState<NarrativeBeat>({
    id: 'beat-1',
    beatIndex: session ? session.current_act_number : 1,
    title: session ? session.current_act_title : 'Ato Inicial',
    location: session
      ? `Ato ${session.current_act_number} — ${session.current_act_title}`
      : 'Cenário da Aventura',
    proseContent: session
      ? `Carregando a narrativa densa do prólogo gerada pelo Mestre...`
      : 'Aguardando início da sessão narrativa.',
    activeNpcs: [],
    tacticalGridEngaged: true,
  });

  const [worldPrologue, setWorldPrologue] = useState<string>(() => {
    return session?.world_prologue || '';
  });
  const [isPrologueModalOpen, setIsPrologueModalOpen] = useState<boolean>(false);

  // Connect WebSocket & load chronicle if session is active
  useEffect(() => {
    if (!session) return;

    // Load initial chronicle entries (including the dense prologue)
    getCampaignChronicle(session.campaign_id)
      .then((entries) => {
        setChronicleEntries(entries);
        const prologueEntry = entries.find(
          (e) => e.entry_type === 'LiteraryChapter' || e.scene_title.toLowerCase().includes('prólogo')
        );
        if (prologueEntry) {
          setWorldPrologue((prev) => prev || prologueEntry.narrative_prose);
        }
        if (entries.length > 0) {
          const latest = entries[entries.length - 1];
          setActiveBeat((b) => ({
            ...b,
            proseContent: latest.narrative_prose,
            title: latest.scene_title,
          }));
        }
      })
      .catch((e) => console.warn('Erro ao carregar crônica:', e));

    // Load initial scene state & current act title from server
    getCampaignState(session.campaign_id)
      .then((state) => {
        const actTitle = (state.current_act_title as string) || session.current_act_title;
        const actNum = (state.current_act_number as number) || session.current_act_number || 1;
        if (state.world_prologue) {
          setWorldPrologue(state.world_prologue as string);
        }
        setActiveBeat((b) => ({
          ...b,
          beatIndex: actNum,
          location: `Ato ${actNum} — ${actTitle}`,
        }));
        if (state.active_scene) {
          const sc = state.active_scene as LivingSceneData;
          if (sc.world_prologue) {
            setWorldPrologue((prev) => prev || (sc.world_prologue as string));
          }
          syncSceneState(sc, state.player_entities as any);
          setActiveBeat((b) => ({
            ...b,
            title: sc.title,
            location: sc.location,
            proseContent: sc.opening_narration || b.proseContent,
            activeNpcs: sc.active_npcs || [],
          }));
        }
      })
      .catch((e) => console.warn('Erro ao carregar estado da campanha:', e));

    // Connect WebSocket
    try {
      const ws = connectCampaignWebSocket(session.room_code, (msg) => {
        if (msg.type === 'INITIAL_SYNC' && msg.active_scene) {
          const sc = msg.active_scene as LivingSceneData;
          if (sc.world_prologue) {
            setWorldPrologue((prev) => prev || (sc.world_prologue as string));
          }
          syncSceneState(sc, msg.player_entities as any);
          setActiveBeat((b) => ({
            ...b,
            title: sc.title,
            location: sc.location,
            proseContent: sc.opening_narration || b.proseContent,
            activeNpcs: sc.active_npcs || [],
          }));
        } else if (msg.type === 'PARTY_SYNCED' && Array.isArray(msg.player_slots)) {
          const slots = msg.player_slots as PlayerSlot[];
          setPlayerSlots(slots);
          setPlayerCount(slots.length);
          setActionLog((prev) => [...prev, `[SALA] Grupo sincronizado no servidor (${slots.length} heróis).`]);
        } else if (msg.type === 'SCENE_STARTED' && msg.scene) {
          const sc = msg.scene as LivingSceneData;
          syncSceneState(sc, msg.player_entities as any);
          setActiveBeat((b) => ({
            ...b,
            title: sc.title,
            location: sc.location,
            proseContent: sc.opening_narration || b.proseContent,
            activeNpcs: sc.active_npcs || [],
          }));
          setActionLog((prev) => [...prev, `[SISTEMA] Nova Cena Viva Iniciada: ${sc.title}`]);
        } else if (msg.type === 'PLAYER_JOINED') {
          setPlayerCount(typeof msg.total_players === 'number' ? msg.total_players : (c) => c + 1);
          setActionLog((prev) => [...prev, `[SALA] Jogador ${msg.player_name} (${msg.class_name}) conectou-se à sala!`]);
        } else if (msg.type === 'TURN_RESOLVED') {
          if (msg.active_scene) {
            syncSceneState(msg.active_scene as LivingSceneData, msg.player_entities as any);
          }
          if (msg.scene_closed) {
            const closed = msg.scene_closed as { polished_chapter_text?: string; chronicle_event_id?: string };
            setLivingScene(null);
            if (closed.polished_chapter_text) {
              setActiveBeat((b) => ({
                ...b,
                proseContent: closed.polished_chapter_text || b.proseContent,
              }));
              getCampaignChronicle(session.campaign_id).then(setChronicleEntries).catch(() => {});
            }
          }
        } else if (msg.type === 'ITEM_CLAIMED') {
          if (msg.active_scene) {
            syncSceneState(msg.active_scene as LivingSceneData, msg.player_entities as any);
          }
          if (msg.item && msg.player_name) {
            const item = msg.item as SceneLootItemData;
            setActionLog((prev) => [
              ...prev,
              `[TESOURO OBTIDO] ${msg.player_name} coletou e equipou: ${item.name} (${item.rarity})!`,
            ]);
          }
        } else if (
          msg.type === 'SCENE_MOVED' ||
          msg.type === 'PARTICIPANT_MOVED' ||
          msg.type === 'TURN_ENDED' ||
          msg.type === 'SCENE_UPDATED'
        ) {
          if (msg.type === 'PARTICIPANT_MOVED') {
          }
          const sc = (msg.active_scene || msg.scene) as LivingSceneData | undefined;
          if (sc) {
            syncSceneState(sc, msg.player_entities as any);
          }
        }
      });

      ws.onopen = () => setWsConnected(true);
      ws.onclose = () => setWsConnected(false);
      wsRef.current = ws;

      return () => {
        ws.close();
      };
    } catch {
      // WS fallback
    }
  }, [session]);

  const handleStartLivingScene = async () => {
    if (!session || isStartingScene) return;
    setIsStartingScene(true);
    try {
      // Sem payload hardcoded: o servidor gera dinamicamente a partir do Esqueleto Narrativo e da Crônica!
      const newScene = await startLivingScene(session.campaign_id);
      syncSceneState(newScene);
      setActiveBeat((b) => ({
        ...b,
        title: newScene.title,
        location: newScene.location,
        proseContent:
          newScene.opening_narration ||
          `⚔️ A Cena Viva "${newScene.title}" começou em ${newScene.location}!`,
        activeNpcs: newScene.active_npcs || [],
      }));
    } catch (e) {
      console.warn('Erro ao iniciar cena viva:', e);
    } finally {
      setIsStartingScene(false);
    }
  };

  const handleClaimItem = async (item: SceneLootItemData) => {
    if (!session) return;
    try {
      const res = await claimSceneItem(session.campaign_id, item.id, character.name);
      if (res.active_scene) {
        setLivingScene(res.active_scene);
      }
      const newItem = {
        name: item.name,
        quantity: 1,
        weightLbs: 1,
        isEquipped: true,
      };
      const updatedChar: CharacterSheet = {
        ...character,
        inventory: [...(character.inventory || []), newItem],
      };
      setCharacter(updatedChar);
      onUpdateCharacter?.(updatedChar);

      const msgText = `[INVENTÁRIO] Você coletou com sucesso: ${item.name} (${item.rarity})${
        item.mechanical_bonus ? ` [Bônus: ${item.mechanical_bonus}]` : ''
      }!`;
      setActionLog((prev) => [...prev, msgText]);
      onShowToast?.(`Item coletado: ${item.name}!`, 'success');
    } catch (e: any) {
      console.warn('Erro ao coletar item:', e);
      setActionLog((prev) => [...prev, `[ERRO] Falha ao coletar item: ${e.message || 'desconhecido'}`]);
      onShowToast?.(`Falha ao coletar item: ${e.message || 'desconhecido'}`, 'warning');
    }
  };

  const handleResolveScene = async () => {
    if (!session) return;
    try {
      await resolveScene(session.campaign_id, 'objectives_completed');
      onShowToast?.('Cena concluída com sucesso! Nova cena e crônica geradas pelo Mestre IA.', 'success');
      setLocalConsumedActions(new Set());
      const updated = await getCampaignState(session.campaign_id);
      if (updated?.living_scene) {
        const sc = updated.living_scene as LivingSceneData;
        setLivingScene(sc);
        setActiveBeat((b) => ({
          ...b,
          title: sc.title,
          location: sc.location,
          proseContent:
            sc.opening_narration ||
            `📜 Nova Cena: ${sc.title} em ${sc.location}`,
          activeNpcs: sc.active_npcs || [],
        }));
      }
      const chron = await getCampaignChronicle(session.campaign_id);
      if (chron) setChronicleEntries(chron);
      setActionLog((prev) => [
        ...prev,
        `📜 [MESTRE IA] A cena atual foi concluída! A história avança para o próximo ato.`,
      ]);
    } catch (err: any) {
      console.warn('Erro ao concluir cena:', err);
      onShowToast?.(`Erro ao concluir cena: ${err.message || 'Falha no servidor'}`, 'warning');
    }
  };

  const strMod = calculateAbilityModifier(character.abilities.strength);
  const dexMod = calculateAbilityModifier(character.abilities.dexterity);
  const intMod = calculateAbilityModifier(character.abilities.intelligence);
  const wisMod = calculateAbilityModifier(character.abilities.wisdom);
  const chaMod = calculateAbilityModifier(character.abilities.charisma);
  const prof = character.proficiencyBonus;
  const attacksCount = character.attacksPerAction || 1;

  // Build Sheet Actions dynamically based on character capabilities & level
  const spellActions: SheetActionOption[] = (character.spellsKnownOrPrepared || []).map(
    (spellName, idx) => ({
      id: `spell-${idx}`,
      name: `Conjurar: ${spellName}`,
      type: 'action',
      category: 'spell',
      description: `Conjura ${spellName} usando sua CD de Magia (${
        8 + prof + Math.max(intMod, wisMod, chaMod)
      }) ou bônus de ataque mágico (+${prof + Math.max(intMod, wisMod, chaMod)}).`,
      hitBonusOrDc: `CD ${8 + prof + Math.max(intMod, wisMod, chaMod)} / +${
        prof + Math.max(intMod, wisMod, chaMod)
      }`,
      isAvailable: true,
    })
  );

  const sheetActions: SheetActionOption[] = [
    {
      id: 'attack-weapon',
      name:
        attacksCount > 1
          ? `Ataque Marcial (${attacksCount}x Ataques Extras)`
          : 'Ataque Marcial com Arma',
      type: 'action',
      category: 'attack',
      description:
        attacksCount > 1
          ? `Realiza ${attacksCount} ataques consecutivos no turno (Ataque Extra de Nível ${character.level}).`
          : 'Desfere um ataque marcial preciso contra um alvo engajado ou elemento sólido do cenário.',
      rangeFeet: 5,
      hitBonusOrDc: `+${Math.max(strMod, dexMod) + prof} para acertar`,
      damageFormula: `${attacksCount}x (1d8 + ${Math.max(strMod, dexMod)})`,
      isAvailable: true,
    },
    ...spellActions,
    {
      id: 'shove-action',
      name: 'Empurrar (Shove)',
      type: 'action',
      category: 'basic',
      description: 'Usa sua força (Atletismo) para empurrar um alvo ou obstáculo a até 5ft (afastando 5 pés ou derrubando).',
      hitBonusOrDc: `+${strMod + prof} (Atletismo)`,
      rangeFeet: 5,
      isAvailable: true,
    },
    {
      id: 'dash-action',
      name: 'Disparada (Dash)',
      type: 'action',
      category: 'basic',
      description: 'Ganha deslocamento adicional igual ao seu deslocamento máximo para o turno.',
      isAvailable: true,
    },
    {
      id: 'disengage-action',
      name: 'Desengajar (Disengage)',
      type: 'action',
      category: 'basic',
      description: 'Seu movimento não provoca ataques de oportunidade pelo restante do turno.',
      isAvailable: true,
    },
    {
      id: 'dodge-action',
      name: 'Esquivar (Dodge)',
      type: 'action',
      category: 'basic',
      description: 'Ataques contra você têm desvantagem e seus testes de DES têm vantagem até o início do seu próximo turno.',
      isAvailable: true,
    },
    {
      id: 'help-action',
      name: 'Ajudar (Help)',
      type: 'action',
      category: 'basic',
      description: 'Concede vantagem na próxima rolagem de teste ou ataque de um aliado a até 5ft.',
      isAvailable: true,
    },
    {
      id: 'hide-action',
      name: 'Esconder-se (Hide)',
      type: 'action',
      category: 'basic',
      description: 'Realiza teste de Furtividade para ficar oculto no ambiente se houver cobertura.',
      isAvailable: true,
    },
  ];

  // Build Contextual Scenario Actions dynamically from LivingScene (or objectives)
  const serverConsumed = new Set(livingScene?.consumed_scenario_options || []);
  const rawScenarioActions: ContextualScenarioOption[] =
    livingScene?.scenario_options && livingScene.scenario_options.length > 0
      ? livingScene.scenario_options
      : livingScene?.objectives && livingScene.objectives.length > 0
      ? livingScene.objectives.map((obj, idx) => ({
          id: `scen-${obj.id}`,
          title: obj.description,
          description: `Agir diretamente no cenário de "${livingScene.location}" para cumprir este objetivo.`,
          actionCost: 'action' as const,
          tacticalBenefit: 'Avança na resolução dos objetivos da Cena Viva.',
          skillCheckSuggested: {
            skill: idx === 0 ? 'Perception' : idx === 1 ? 'Investigation' : 'Persuasion',
            dcEstimate: 12 + idx,
          },
        }))
      : [
          {
            id: 'scen-analyze',
            title: `Analisar o Ambiente de ${activeBeat.title}`,
            description: 'Examina os arredores em busca de pistas, ameaças ocultas ou rotas vantajosas.',
            actionCost: 'action' as const,
            tacticalBenefit: 'Revela detalhes táticos e segredos da cena atual.',
            skillCheckSuggested: {
              skill: 'Perception',
              dcEstimate: 12,
            },
          },
        ];

  const scenarioActions: ContextualScenarioOption[] = rawScenarioActions.map((opt) => ({
    ...opt,
    consumed:
      opt.consumed ??
      (serverConsumed.has(opt.id) || localConsumedActions.has(opt.id)),
  }));

  const handleGrantExtraAction = () => {
    setBudget((b) => ({ ...b, actionAvailable: true }));
    setActionLog((prev) => [
      ...prev,
      `⚡ [SURTO DE AÇÃO (ACTION SURGE)]: ${character.name} canaliza toda sua força marcial! +1 Ação concedida no turno atual!`,
    ]);
    onShowToast?.('⚡ Surto de Ação: +1 Ação concedida!', 'success');
  };

  const handleConsumeAction = (costType: 'action' | 'bonus_action' | 'reaction' | 'free', label: string) => {
    if (costType === 'action') {
      setBudget((b) => ({ ...b, actionAvailable: false }));
    } else if (costType === 'bonus_action') {
      setBudget((b) => ({ ...b, bonusActionAvailable: false }));
    } else if (costType === 'reaction') {
      setBudget((b) => ({ ...b, reactionAvailable: false }));
    }
    setActionLog((prev) => [...prev, `[HABILIDADE] ${label} consumida (${costType}).`]);
  };

  const handleUpdatePreparedSpells = (spells: string[]) => {
    const updated: CharacterSheet = {
      ...character,
      spellsKnownOrPrepared: spells,
    };
    setCharacter(updated);
    onUpdateCharacter?.(updated);
  };

  const handleRestoreSpellSlots = (levelsToRecover: number) => {
    setCharacter((prev) => {
      const slots = { ...prev.spellSlots, levels: { ...prev.spellSlots.levels } };
      let remaining = levelsToRecover;
      const lvls = Object.keys(slots.levels)
        .map(Number)
        .sort((a, b) => b - a);
      for (const lvl of lvls) {
        while (remaining >= lvl && slots.levels[lvl] && slots.levels[lvl].used > 0) {
          slots.levels[lvl] = {
            ...slots.levels[lvl],
            used: slots.levels[lvl].used - 1,
          };
          remaining -= lvl;
        }
      }
      const updated = { ...prev, spellSlots: slots };
      onUpdateCharacter?.(updated);
      return updated;
    });
  };

  const handleUpdateHp = (amount: number) => {
    setCharacter((prev) => {
      const newCurrent = Math.max(
        0,
        Math.min(prev.vitals.hpMax, prev.vitals.hpCurrent + amount)
      );
      const updated: CharacterSheet = {
        ...prev,
        vitals: {
          ...prev.vitals,
          hpCurrent: newCurrent,
        },
      };
      onUpdateCharacter?.(updated);
      return updated;
    });
  };

  const handleToggleSpellSlot = (level: number, isPactMagic = false) => {
    setCharacter((prev) => {
      const slots = { ...prev.spellSlots };
      if (isPactMagic && slots.pactMagic) {
        slots.pactMagic = {
          ...slots.pactMagic,
          used:
            slots.pactMagic.used >= slots.pactMagic.total
              ? 0
              : slots.pactMagic.used + 1,
        };
      } else if (slots.levels[level]) {
        const current = slots.levels[level];
        slots.levels = {
          ...slots.levels,
          [level]: {
            ...current,
            used: current.used >= current.total ? 0 : current.used + 1,
          },
        };
      }
      const updated = { ...prev, spellSlots: slots };
      onUpdateCharacter?.(updated);
      return updated;
    });
  };

  // Execute Sheet Action
  const handleExecuteSheetAction = async (
    action: SheetActionOption,
    target?: { id?: string; name?: string }
  ) => {
    if (action.type === 'action') {
      setBudget((b) => ({ ...b, actionAvailable: false }));
    } else if (action.type === 'bonus_action') {
      setBudget((b) => ({ ...b, bonusActionAvailable: false }));
    } else if (action.type === 'reaction') {
      setBudget((b) => ({ ...b, reactionAvailable: false }));
    }

    const fallbackTarget = selectedTargetId
      ? livingScene?.participants?.find((p) => p.id === selectedTargetId)
      : undefined;
    const effectiveTargetId = target?.id || fallbackTarget?.id;
    const effectiveTargetName = target?.name || fallbackTarget?.name;
    const effectiveActorId = getActiveActorId();

    if (session) {
      try {
        const turnRes = await submitCampaignTurn(session.campaign_id, {
          via_type: 'via1',
          action_id: action.id,
          player_text: effectiveTargetName
            ? `${action.name} contra ${effectiveTargetName}`
            : action.name,
          character_name: character.name,
          class_name: character.className,
          actor_id: effectiveActorId,
          target_id: effectiveTargetId,
          target_name: effectiveTargetName,
        });

        const rollResult = turnRes.rules_result as any;
        const isCombatAction =
          action.category === 'attack' ||
          action.id.startsWith('attack') ||
          rollResult?.action === 'attack';

        if (isCombatAction) {
          const isHit =
            rollResult?.is_hit === true ||
            (typeof rollResult?.damage_dealt === 'number' && rollResult.damage_dealt > 0) ||
            rollResult?.is_success === true ||
            (rollResult === undefined &&
              !turnRes.narrative_reaction.toLowerCase().includes('errou') &&
              !turnRes.narrative_reaction.toLowerCase().includes('falhou') &&
              !turnRes.narrative_reaction.toLowerCase().includes('esquiva'));

          if (isHit) {
          } else {
          }
        }

        if (turnRes.active_scene) {
          syncSceneState(turnRes.active_scene, turnRes.player_entities);
        } else {
          getCampaignState(session.campaign_id)
            .then((st) => {
              if (st.active_scene) {
                syncSceneState(st.active_scene as LivingSceneData, st.player_entities as any);
              }
            })
            .catch(() => {});
        }

        setActionLog((prev) => [
          ...prev,
          `[VIA 1 — FICHA] ${action.name}${effectiveTargetName ? ` (Alvo: ${effectiveTargetName})` : ''} -> ${turnRes.narrative_reaction}`,
        ]);
        return;
      } catch (e) {
        console.warn('Erro ao enviar via 1 ao servidor:', e);
        if (action.category === 'attack' || action.id.startsWith('attack')) {
        }
      }
    }

    if (action.category === 'attack' || action.id.startsWith('attack')) {
    }

    setActionLog((prev) => [
      ...prev,
      `[${action.type.toUpperCase()}] ${action.name}${
        effectiveTargetName ? ` contra ${effectiveTargetName}` : ''
      } executado com sucesso. (${action.damageFormula || action.description})`,
    ]);
  };

  // Execute Tactical Retreat / Flee Battle (Action • DC 12 Check)
  const handleFleeBattle = async () => {
    setBudget((b) => ({ ...b, actionAvailable: false }));
    const effectiveActorId = getActiveActorId();

    if (session) {
      try {
        const turnRes = await submitCampaignTurn(session.campaign_id, {
          via_type: 'via1',
          action_id: 'flee',
          player_text: 'Retirada Tática: Tentar fugir e desengajar do combate',
          character_name: character.name,
          class_name: character.className,
          actor_id: effectiveActorId,
        });

        if (turnRes.active_scene) {
          syncSceneState(turnRes.active_scene, turnRes.player_entities);
        } else {
          const st = await getCampaignState(session.campaign_id);
          if (st.active_scene) {
            syncSceneState(st.active_scene as LivingSceneData, st.player_entities as any);
          }
        }

        const rollResult = turnRes.rules_result as { total?: number; is_success?: boolean } | undefined;
        const fled =
          rollResult?.is_success === true ||
          turnRes.active_scene?.in_combat === false ||
          turnRes.narrative_reaction.toLowerCase().includes('sucesso') ||
          turnRes.narrative_reaction.toLowerCase().includes('escapa') ||
          turnRes.narrative_reaction.toLowerCase().includes('retirada bem-sucedida');

        setActionLog((prev) => [
          ...prev,
          `🏃‍♂️ [RETIRADA TÁTICA] ${turnRes.narrative_reaction}`,
        ]);

        if (fled) {
          setIsCombatMode(false);
          onShowToast?.('🏃‍♂️ Retirada Tática bem-sucedida! O grupo escapou do combate.', 'success');
        } else {
          onShowToast?.(
            '⚠️ Tentativa de retirada contida pelos inimigos! Sua Ação foi consumida.',
            'warning'
          );
        }
        return;
      } catch (e) {
        console.warn('Erro ao executar retirada tática no servidor:', e);
      }
    }

    const d20 = Math.floor(Math.random() * 20) + 1;
    const bonus = Math.max(strMod, dexMod) + prof;
    const total = d20 + bonus;
    const isSuccess = total >= 12;
    setActionLog((prev) => [
      ...prev,
      `🏃‍♂️ [RETIRADA TÁTICA] Teste D20 (${d20}) + ${bonus} = ${total} vs CD 12 -> ${
        isSuccess
          ? 'SUCESSO! Você desengaja e recua em segurança.'
          : 'FALHA! Os inimigos bloqueiam a rota de fuga.'
      }`,
    ]);
    if (isSuccess) {
      setIsCombatMode(false);
      onShowToast?.('🏃‍♂️ Retirada Tática bem-sucedida! Combate encerrado.', 'success');
    } else {
      onShowToast?.(`⚠️ Falha na Retirada Tática (${total} vs CD 12)!`, 'warning');
    }
  };

  // Attack Target directly from TacticalBattleCanvas
  const handleAttackTargetFromCanvas = (target: TurnParticipantData) => {
    setSelectedTargetId(target.id);
    const primaryAttack =
      sheetActions.find((a) => a.category === 'attack' || a.id === 'attack-weapon') ||
      sheetActions[0];
    if (primaryAttack) {
      handleExecuteSheetAction(primaryAttack, { id: target.id, name: target.name });
    }
  };

  // Execute Scenario Action
  const handleExecuteScenarioAction = async (scen: ContextualScenarioOption) => {
    if (scen.actionCost === 'action') {
      setBudget((b) => ({ ...b, actionAvailable: false }));
    } else if (scen.actionCost === 'bonus_action') {
      setBudget((b) => ({ ...b, bonusActionAvailable: false }));
    }

    setLocalConsumedActions((prev) => new Set([...prev, scen.id, scen.title]));

    const effectiveActorId = getActiveActorId();

    if (session) {
      try {
        const turnRes = await submitCampaignTurn(session.campaign_id, {
          via_type: 'via2',
          action_id: scen.id,
          player_text: scen.title,
          character_name: character.name,
          class_name: character.className,
          actor_id: effectiveActorId,
        });

        const rollResult = turnRes.rules_result as any;
        if (rollResult?.action === 'attack' || scen.title.toLowerCase().includes('ataque')) {
          if (rollResult?.is_hit || rollResult?.damage_dealt > 0 || rollResult?.is_success) {
          } else {
          }
        }

        if (turnRes.active_scene) {
          syncSceneState(turnRes.active_scene, turnRes.player_entities);
        } else {
          getCampaignState(session.campaign_id)
            .then((st) => {
              if (st.active_scene) {
                syncSceneState(st.active_scene as LivingSceneData, st.player_entities as any);
              }
            })
            .catch(() => {});
        }

        setActionLog((prev) => [
          ...prev,
          `[VIA 2 — CENÁRIO] ${scen.title} -> ${turnRes.narrative_reaction}`,
        ]);
        return;
      } catch (e) {
        console.warn('Erro ao enviar via 2 ao servidor:', e);
        if (scen.title.toLowerCase().includes('ataca')) {
        }
      }
    }

    const d20 = Math.floor(Math.random() * 20) + 1;
    const total = d20 + prof + strMod;
    const isSuccess = scen.skillCheckSuggested?.dcEstimate
      ? total >= scen.skillCheckSuggested.dcEstimate
      : d20 > 10;

    if (scen.id.includes('attack') || scen.title.toLowerCase().includes('ataque')) {
      if (isSuccess) {
      } else {
      }
    }

    setActionLog((prev) => [
      ...prev,
      `[CENÁRIO] ${scen.title} -> Teste D20 (${d20}) + Bônus = ${total} vs CD ${
        scen.skillCheckSuggested?.dcEstimate || 12
      }: ${isSuccess ? 'SUCESSO! ' + scen.tacticalBenefit : 'FALHA! Ação consumida.'}`,
    ]);
  };

  // Freeform Text Action (AI Evaluation via Backend / Resolver)
  const handleSubmitFreeformAction = async (text: string) => {
    const effectiveActorId = getActiveActorId();
    const fallbackTarget = selectedTargetId
      ? livingScene?.participants?.find((p) => p.id === selectedTargetId)
      : undefined;

    if (session) {
      try {
        const turnRes = await submitCampaignTurn(session.campaign_id, {
          via_type: 'via3',
          player_text: text,
          character_name: character.name,
          class_name: character.className,
          actor_id: effectiveActorId,
          target_id: fallbackTarget?.id,
          target_name: fallbackTarget?.name,
        });

        setBudget((b) => ({ ...b, actionAvailable: false }));

        const rollResult = turnRes.rules_result as any;
        const textLower = text.toLowerCase();
        const isCombatAction =
          rollResult?.action === 'attack' ||
          textLower.includes('ataca') ||
          textLower.includes('golpe') ||
          textLower.includes('dispara') ||
          textLower.includes('flecha') ||
          textLower.includes('magia') ||
          textLower.includes('acerta');

        if (isCombatAction) {
          const isHit =
            rollResult?.is_hit === true ||
            (typeof rollResult?.damage_dealt === 'number' && rollResult.damage_dealt > 0) ||
            rollResult?.is_success === true ||
            (rollResult === undefined &&
              !turnRes.narrative_reaction.toLowerCase().includes('errou') &&
              !turnRes.narrative_reaction.toLowerCase().includes('falhou'));

          if (isHit) {
          } else {
          }
        }

        if (turnRes.active_scene) {
          syncSceneState(turnRes.active_scene, turnRes.player_entities);
        } else {
          getCampaignState(session.campaign_id)
            .then((st) => {
              if (st.active_scene) {
                syncSceneState(st.active_scene as LivingSceneData, st.player_entities as any);
              }
            })
            .catch(() => {});
        }

        const sub: FreeformTextActionSubmission = {
          playerText: text,
          submittedAt: new Date().toLocaleTimeString(),
          status: 'resolved',
          requiresRulesMechanic: Boolean(turnRes.rules_result),
          detectedIntent: 'Ação Processada pelo Agente de Ambiente',
          resolvedCheck: rollResult
            ? {
                abilityOrSkill: 'D20 + Modificador (Motor Rust)',
                rollFormula: 'd20 + bônus determinístico',
                targetDc: 13,
                result: rollResult.total ?? 15,
                success: rollResult.is_success ?? true,
              }
            : undefined,
          narrativeFeedback: turnRes.narrative_reaction,
        };

        setRecentSubmission(sub);
        setActionLog((prev) => [
          ...prev,
          `[VIA 3 — TEXTO] "${text}" -> ${turnRes.narrative_reaction}`,
        ]);
        return;
      } catch (e) {
        console.warn('Erro ao enviar turno da campanha:', e);
        if (text.toLowerCase().includes('ataca') || text.toLowerCase().includes('golpe')) {
        }
      }
    }

    const sub = await submitTurnActionToBackend('via3', character, text);
    if (sub.requiresRulesMechanic) {
      setBudget((b) => ({ ...b, actionAvailable: false }));
    }

    if (text.toLowerCase().includes('ataca') || text.toLowerCase().includes('golpe')) {
      if (sub.resolvedCheck ? sub.resolvedCheck.success : true) {
      } else {
      }
    }

    setRecentSubmission(sub);
    setActionLog((prev) => [
      ...prev,
      `[${sub.requiresRulesMechanic ? 'MOTOR RUST #regras' : 'IA NARRATIVA'}] "${text}" -> ${
        sub.resolvedCheck
          ? `D20 = ${sub.resolvedCheck.result} (${sub.resolvedCheck.success ? 'SUCESSO' : 'FALHA'})`
          : sub.narrativeFeedback
      }`,
    ]);
  };

  const handleResetTurnBudget = () => {
    setBudget({
      actionAvailable: true,
      bonusActionAvailable: true,
      reactionAvailable: true,
      movementTotalFeet: character.vitals.speedFeet,
      movementRemainingFeet: character.vitals.speedFeet,
    });
    setActionLog([]);
  };

  const classMeta = DND_CLASSES.find((c) => c.id === character.className);

  return (
    <div style={{ maxWidth: '1440px', margin: '0 auto', padding: '16px', display: 'flex', flexDirection: 'column', gap: '16px' }}>
      {/* Top Bar with Character Header, Room UUID & Mode Switcher */}
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          backgroundColor: 'var(--bg-card)',
          border: `1px solid ${classMeta?.color || 'var(--border-color)'}`,
          padding: '12px 20px',
          borderRadius: '12px',
          flexWrap: 'wrap',
          gap: '12px',
        }}
      >
        <div style={{ display: 'flex', alignItems: 'center', gap: '12px', flexWrap: 'wrap' }}>
          <h1 className="rpg-title" style={{ fontSize: '20px', color: '#fff', margin: 0 }}>
            {character.name}
          </h1>
          <span
            style={{
              backgroundColor: `${classMeta?.color}22`,
              color: classMeta?.color,
              border: `1px solid ${classMeta?.color}`,
              borderRadius: '6px',
              padding: '2px 8px',
              fontSize: '12px',
              fontWeight: 'bold',
            }}
          >
            {classMeta?.namePt} Nível {character.level}
          </span>
          <span style={{ fontSize: '13px', color: 'var(--text-muted)' }}>
            {character.race} • {character.background}
          </span>

          {session && (
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginLeft: '6px', flexWrap: 'wrap' }}>
              {/* UUID Badge with 1-Click Copy */}
              <button
                onClick={handleCopyUuid}
                title={`UUID Completo: ${session.campaign_id} (Clique para copiar)`}
                style={{
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '5px',
                  backgroundColor: '#0f172a',
                  color: copiedUuid ? '#34d399' : '#94a3b8',
                  border: copiedUuid ? '1px solid #34d399' : '1px solid #334155',
                  padding: '2px 8px',
                  borderRadius: '6px',
                  fontSize: '11px',
                  fontFamily: 'monospace',
                  cursor: 'pointer',
                  transition: 'all 0.15s ease',
                }}
              >
                {copiedUuid ? <Check size={11} color="#34d399" /> : <Copy size={11} />}
                UUID: {session.campaign_id.slice(0, 8)}...
              </button>

              {/* Room Code Badge */}
              <span
                style={{
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '4px',
                  backgroundColor: '#1e293b',
                  color: 'var(--accent-gold)',
                  border: '1px solid rgba(245, 158, 11, 0.4)',
                  padding: '2px 8px',
                  borderRadius: '6px',
                  fontSize: '11px',
                  fontWeight: 700,
                  fontFamily: 'monospace',
                }}
              >
                SALA #{session.room_code}
              </span>

              {/* Mode Toggle: Modo Um Terminal (Presencial) vs Modo Online */}
              <button
                onClick={() => setIsSingleTerminal((prev) => !prev)}
                title="Alternar entre Modo Um Terminal (mesa presencial compartilhada) e Modo Online (WebSockets)"
                style={{
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '5px',
                  backgroundColor: isSingleTerminal ? 'rgba(56, 189, 248, 0.15)' : 'rgba(16, 185, 129, 0.12)',
                  color: isSingleTerminal ? '#38bdf8' : '#34d399',
                  border: isSingleTerminal ? '1px solid rgba(56, 189, 248, 0.4)' : '1px solid rgba(16, 185, 129, 0.4)',
                  padding: '2px 8px',
                  borderRadius: '6px',
                  fontSize: '11px',
                  fontWeight: 700,
                  cursor: 'pointer',
                }}
              >
                <Monitor size={12} />
                {isSingleTerminal ? '🖥️ Modo Um Terminal' : '🌐 Modo Online'}
              </button>

              {/* Status Connection Indicator */}
              <span
                style={{
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '4px',
                  backgroundColor: '#0f172a',
                  color: isSingleTerminal ? '#38bdf8' : wsConnected ? '#10b981' : '#f59e0b',
                  border: '1px solid #334155',
                  padding: '2px 8px',
                  borderRadius: '6px',
                  fontSize: '11px',
                }}
              >
                {isSingleTerminal ? <Users size={12} /> : <Wifi size={12} />}
                {playerSlots.length > 0 ? playerSlots.length : playerCount}/4 Jogadores
              </span>
            </div>
          )}
        </div>

        <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
          <button
            type="button"
            onClick={() => setIsPrologueModalOpen(true)}
            data-testid="hud-prologue-button"
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: '6px',
              padding: '6px 14px',
              borderRadius: '8px',
              backgroundColor: 'rgba(234, 179, 8, 0.16)',
              border: '1px solid #eab308',
              fontSize: '12px',
              color: '#fef08a',
              fontWeight: 800,
              cursor: 'pointer',
              boxShadow: '0 0 14px rgba(234, 179, 8, 0.22)',
              transition: 'all 0.15s ease',
            }}
            title="Abrir a Contextualização da História, Prólogo do Mundo e Lore da Campanha"
          >
            <Scroll size={13} color="#facc15" />
            <span>📜 Prólogo &amp; Contexto</span>
          </button>

          {onOpenAudit && (
            <button
              onClick={onOpenAudit}
              data-testid="hud-audit-button"
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '6px',
                padding: '6px 14px',
                borderRadius: '8px',
                backgroundColor: 'rgba(234, 179, 8, 0.16)',
                border: '1px solid #eab308',
                fontSize: '12px',
                color: '#fef08a',
                fontWeight: 800,
                cursor: 'pointer',
                boxShadow: '0 0 14px rgba(234, 179, 8, 0.22)',
                transition: 'all 0.15s ease',
              }}
              title="Abrir o Inspetor do Mestre IA: Logs, Entidades, Prompts, Histórico e Auditoria de Conteúdo"
            >
              <Sparkles size={13} color="#facc15" />
              <span>🔬 Inspetor IA (Auditoria &amp; Logs)</span>
            </button>
          )}

          {session && onExitSession && (
            <button
              onClick={onExitSession}
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '6px',
                padding: '6px 12px',
                borderRadius: '6px',
                backgroundColor: '#1e293b',
                border: '1px solid #334155',
                fontSize: '12px',
                color: '#f87171',
                cursor: 'pointer',
              }}
            >
              Sair da Sala
            </button>
          )}

          <button
            onClick={onResetCharacter}
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: '6px',
              padding: '6px 12px',
              borderRadius: '6px',
              backgroundColor: '#161a22',
              border: '1px solid var(--border-color)',
              fontSize: '12px',
              color: 'var(--text-muted)',
              cursor: 'pointer',
            }}
          >
            <RefreshCw size={13} /> Trocar Personagem
          </button>
        </div>
      </div>

      {/* BANDEJA DE HERÓIS DA MESA / HOT-SEAT ROSTER TRAY */}
      {session && (
        <div
          data-testid="party-roster-tray"
          style={{
            backgroundColor: '#0c121e',
            border: isSingleTerminal ? '1px solid rgba(234, 179, 8, 0.35)' : '1px solid rgba(56, 189, 248, 0.3)',
            borderRadius: '12px',
            padding: '12px 18px',
            display: 'flex',
            flexDirection: 'column',
            gap: '10px',
            boxShadow: '0 8px 24px rgba(0, 0, 0, 0.45)',
          }}
        >
          <div
            style={{
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
              flexWrap: 'wrap',
              gap: '10px',
              borderBottom: '1px solid rgba(255, 255, 255, 0.07)',
              paddingBottom: '8px',
            }}
          >
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <span
                style={{
                  fontSize: '13px',
                  fontWeight: 800,
                  color: '#facc15',
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  textTransform: 'uppercase',
                  letterSpacing: '0.04em',
                }}
              >
                <Users size={16} color="#facc15" /> Comitiva de Heróis na Mesa ({party && party.length > 0 ? party.length : (playerSlots.length || 1)}/4)
              </span>
              <span
                style={{
                  fontSize: '11px',
                  color: '#94a3b8',
                  backgroundColor: 'rgba(255, 255, 255, 0.05)',
                  padding: '2px 8px',
                  borderRadius: '6px',
                }}
              >
                💡 Clique em qualquer herói ou use os números <strong>[1]</strong>, <strong>[2]</strong>, <strong>[3]</strong>, <strong>[4]</strong> do teclado para alternar o turno ativo.
              </span>
            </div>

            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <span style={{ fontSize: '11.5px', color: '#64748b' }}>
                Código da Sala:
              </span>
              <button
                type="button"
                onClick={() => {
                  navigator.clipboard.writeText(session.room_code);
                  setCopiedUuid(true);
                  setTimeout(() => setCopiedUuid(false), 2000);
                  onShowToast?.(`Código #${session.room_code} copiado!`, 'info');
                }}
                style={{
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '4px',
                  padding: '2px 8px',
                  borderRadius: '6px',
                  backgroundColor: 'rgba(56, 189, 248, 0.12)',
                  border: '1px solid rgba(56, 189, 248, 0.35)',
                  color: '#38bdf8',
                  fontSize: '11px',
                  fontWeight: 700,
                  cursor: 'pointer',
                }}
              >
                {copiedUuid ? <Check size={12} color="#34d399" /> : <Copy size={12} />}
                #{session.room_code}
              </button>
            </div>
          </div>

          {/* Cards dos Heróis da Comitiva */}
          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fit, minmax(220px, 1fr))',
              gap: '10px',
            }}
          >
            {(party && party.length > 0
              ? party
              : [character]
            ).map((hero, idx) => {
              const isCurrentActive =
                character.id === hero.id ||
                character.name.toLowerCase() === hero.name.toLowerCase();
              const hpPct = Math.round((hero.vitals.hpCurrent / Math.max(hero.vitals.hpMax, 1)) * 100);
              const hpColor = hpPct > 50 ? '#4ade80' : hpPct > 25 ? '#facc15' : '#f87171';
              const pSlot = playerSlots[idx];

              return (
                <div
                  key={hero.id || idx}
                  onClick={() => {
                    if (onSelectPartyMember) {
                      onSelectPartyMember(hero);
                      setCharacter(hero);
                      onShowToast?.(`Turno assumido por [${idx + 1}] ${hero.name}!`, 'info');
                    }
                  }}
                  data-testid={`party-member-card-${idx + 1}`}
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: '10px',
                    padding: '8px 12px',
                    borderRadius: '10px',
                    backgroundColor: isCurrentActive
                      ? 'rgba(234, 179, 8, 0.14)'
                      : 'rgba(15, 23, 42, 0.65)',
                    border: isCurrentActive
                      ? '2px solid #eab308'
                      : '1px solid rgba(255, 255, 255, 0.1)',
                    boxShadow: isCurrentActive
                      ? '0 0 16px rgba(234, 179, 8, 0.35)'
                      : 'none',
                    cursor: 'pointer',
                    transition: 'all 0.18s ease',
                    position: 'relative',
                  }}
                >
                  {/* Badge de Tecla de Atalho */}
                  <div
                    style={{
                      width: '28px',
                      height: '28px',
                      borderRadius: '8px',
                      backgroundColor: isCurrentActive ? '#eab308' : 'rgba(255, 255, 255, 0.08)',
                      color: isCurrentActive ? '#000000' : '#38bdf8',
                      fontWeight: 900,
                      fontSize: '13px',
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'center',
                      flexShrink: 0,
                      fontFamily: 'monospace',
                      boxShadow: isCurrentActive ? '0 0 8px #eab308' : 'none',
                    }}
                    title={`Pressione tecla [${idx + 1}] para alternar o turno`}
                  >
                    {idx + 1}
                  </div>

                  {/* Detalhes do Herói */}
                  <div style={{ flex: 1, minWidth: 0 }}>
                    <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: '4px' }}>
                      <span
                        style={{
                          fontSize: '12.5px',
                          fontWeight: 800,
                          color: isCurrentActive ? '#fef08a' : '#f1f5f9',
                          overflow: 'hidden',
                          textOverflow: 'ellipsis',
                          whiteSpace: 'nowrap',
                        }}
                      >
                        {hero.name}
                      </span>
                      {isCurrentActive && (
                        <span
                          style={{
                            fontSize: '9.5px',
                            fontWeight: 800,
                            color: '#000000',
                            backgroundColor: '#eab308',
                            padding: '1px 6px',
                            borderRadius: '4px',
                            textTransform: 'uppercase',
                            letterSpacing: '0.04em',
                            flexShrink: 0,
                          }}
                        >
                          TURNO ATIVO
                        </span>
                      )}
                    </div>

                    <div style={{ fontSize: '11px', color: '#94a3b8', display: 'flex', alignItems: 'center', gap: '6px' }}>
                      <span>{hero.race} {hero.className} Nv.{hero.level}</span>
                      {pSlot && (
                        <span
                          style={{
                            fontSize: '9px',
                            fontFamily: 'monospace',
                            backgroundColor: 'rgba(56, 189, 248, 0.15)',
                            color: '#38bdf8',
                            padding: '0 4px',
                            borderRadius: '3px',
                          }}
                          title={`Código pessoal de login: ${pSlot.player_code}`}
                        >
                          [{pSlot.player_code}]
                        </span>
                      )}
                    </div>

                    {/* Barra de HP e AC */}
                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginTop: '3px' }}>
                      <div
                        style={{
                          flex: 1,
                          height: '5px',
                          backgroundColor: 'rgba(255, 255, 255, 0.1)',
                          borderRadius: '3px',
                          overflow: 'hidden',
                        }}
                      >
                        <div
                          style={{
                            width: `${Math.min(hpPct, 100)}%`,
                            height: '100%',
                            backgroundColor: hpColor,
                            transition: 'width 0.2s ease',
                          }}
                        />
                      </div>
                      <span style={{ fontSize: '10.5px', fontWeight: 700, color: hpColor, fontFamily: 'monospace' }}>
                        {hero.vitals.hpCurrent}/{hero.vitals.hpMax} HP
                      </span>
                      <span style={{ fontSize: '10.5px', color: '#cbd5e1', fontWeight: 700 }}>
                        🛡️ {hero.vitals.armorClass}
                      </span>
                    </div>
                  </div>
                </div>
              );
            })}

            {/* Botão de Adicionar Mais um Herói (se < 4) */}
            {party && party.length < 4 && onRequestAddPartyMember && (
              <button
                type="button"
                onClick={onRequestAddPartyMember}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  gap: '6px',
                  padding: '8px 12px',
                  borderRadius: '10px',
                  backgroundColor: 'rgba(56, 189, 248, 0.08)',
                  border: '1px dashed rgba(56, 189, 248, 0.4)',
                  color: '#7dd3fc',
                  fontSize: '12px',
                  fontWeight: 700,
                  cursor: 'pointer',
                  transition: 'all 0.15s ease',
                }}
                title="Criar novo integrante e adicionar à comitiva"
              >
                <Plus size={15} /> + Adicionar Herói ({party.length + 1}/4)
              </button>
            )}
          </div>
        </div>
      )}

      {/* NOVO LAYOUT HIERÁRQUICO: HISTÓRIA E CENA VIVA NO CENTRO (DOMINANTE), COCKPIT E STATS SECUNDÁRIOS */}
      <style>{`
        @media (max-width: 1024px) {
          .rpg-hud-main-grid {
            grid-template-columns: 1fr !important;
          }
        }
      `}</style>
      <div
        className="rpg-hud-main-grid"
        style={{
          display: 'grid',
          gridTemplateColumns: 'minmax(0, 1fr) 350px',
          gap: '20px',
          alignItems: 'start',
          paddingBottom: '80px',
        }}
      >
        {/* COLUNA CENTRAL E DOMINANTE (História, Cena Viva e Console de Ações 3 Vias) */}
        <div style={{ display: 'flex', flexDirection: 'column', gap: '16px', minWidth: 0 }}>
          <WorldViewPanel
            beat={activeBeat}
            livingScene={livingScene}
            chronicleEntries={chronicleEntries}
            worldPrologue={worldPrologue}
            isPrologueModalOpen={isPrologueModalOpen}
            onTogglePrologueModal={setIsPrologueModalOpen}
            onStartLivingScene={handleStartLivingScene}
            isStartingScene={isStartingScene}
            onClaimItem={handleClaimItem}
            onCellClick={handleCellClick}
            playerPos={playerPos}
            onResolveScene={handleResolveScene}
            onToggleCombatMode={handleToggleCombatMode}
            isCombatMode={isCombatMode}
            party={party}
            activeCharacter={character}
            selectedTargetId={selectedTargetId}
            onSelectTarget={(t) => setSelectedTargetId(t ? t.id : null)}
            onMoveParticipant={handleMoveParticipant}
            onAttackTarget={handleAttackTargetFromCanvas}
            onEndTurn={handleEndTurn}
            onFleeBattle={handleFleeBattle}
          />

          <TurnPossibilitiesConsole
            budget={budget}
            sheetActions={sheetActions}
            scenarioActions={scenarioActions}
            onExecuteSheetAction={(action, target) => handleExecuteSheetAction(action, target || undefined)}
            onExecuteScenarioAction={handleExecuteScenarioAction}
            onSubmitFreeformAction={handleSubmitFreeformAction}
            onResetTurnBudget={handleResetTurnBudget}
            recentSubmission={recentSubmission}
            actionLog={actionLog}
            isCombatMode={isCombatMode}
            hasHostileTargets={hasHostiles}
            hostileNames={livingScene?.active_npcs || []}
            participants={livingScene?.participants || []}
            selectedTargetId={selectedTargetId}
            onSelectTarget={(t) => setSelectedTargetId(t ? t.id : null)}
            onToggleCombatMode={handleToggleCombatMode}
            onResolveScene={handleResolveScene}
            onEndTurn={handleEndTurn}
            onFleeBattle={handleFleeBattle}
          />
        </div>

        {/* COLUNA LATERAL SECUNDÁRIA (HP, Atributos, Inventário e Cockpit da Classe) */}
        <div style={{ display: 'flex', flexDirection: 'column', gap: '16px', minWidth: 0 }}>
          {/* Vitals, Atributos e Inventário PRIMEIRO */}
          <VitalsAndAttributesPanel
            character={character}
            onUpdateHp={handleUpdateHp}
            onToggleSpellSlot={handleToggleSpellSlot}
          />

          {/* Painel Específico da Classe (Fúria, Ki, Magias, Surto de Ação) SEGUNDO */}
          {classSpecificPanel || (
            <ClassHUDDispatcher
              character={character}
              onGrantExtraAction={handleGrantExtraAction}
              onHealSecondWind={handleUpdateHp}
              onConsumeAction={handleConsumeAction}
              onUpdatePreparedSpells={handleUpdatePreparedSpells}
              onRestoreSpellSlots={handleRestoreSpellSlots}
              onNotify={(msg, type) => onShowToast?.(msg, type)}
            />
          )}
        </div>
      </div>

      {/* Chat da Party Retrátil no Canto da Tela */}
      {session && (
        <PartyChatWidget
          campaignId={session.campaign_id}
          roomCode={session.room_code}
          currentUserId={character.id}
          currentUserName={character.name}
        />
      )}
    </div>
  );
};
