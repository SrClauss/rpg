import { useState, useRef, useMemo } from 'react';
import type { FC, MouseEvent } from 'react';
import type { CharacterSheet } from '../../types/character';
import type {
  LivingSceneData,
  TurnParticipantData,
  SceneObstacle,
} from '../../types/session';
import {
  Sword,
  Footprints,
  Sparkles,
  SkipForward,
  Compass,
  Crosshair,
  Lock,
} from 'lucide-react';

export interface TacticalBattleCanvasProps {
  livingScene?: LivingSceneData | null;
  party?: CharacterSheet[];
  activeCharacter?: CharacterSheet | null;
  selectedTargetId?: string | null;
  onSelectTarget?: (target: TurnParticipantData) => void;
  onMoveParticipant?: (xFt: number, yFt: number, actorId?: string) => void;
  onAttackTarget?: (target: TurnParticipantData, distanceFt: number) => void;
  onEndTurn?: (actorId?: string) => void;
  onFleeBattle?: () => void;
  isCombatMode?: boolean;
}

const FIELD_WIDTH_FT = 60;
const FIELD_HEIGHT_FT = 40;
const SVG_WIDTH = 780;
const SVG_HEIGHT = 480;
const PAD_LEFT = 42;
const PAD_TOP = 34;
const PAD_RIGHT = 24;
const PAD_BOTTOM = 30;
const MAP_W = SVG_WIDTH - PAD_LEFT - PAD_RIGHT;
const MAP_H = SVG_HEIGHT - PAD_TOP - PAD_BOTTOM;

function ftToPxX(xFt: number): number {
  const clamped = Math.max(0, Math.min(FIELD_WIDTH_FT, xFt));
  return PAD_LEFT + (clamped / FIELD_WIDTH_FT) * MAP_W;
}

function ftToPxY(yFt: number): number {
  const clamped = Math.max(0, Math.min(FIELD_HEIGHT_FT, yFt));
  return PAD_TOP + (clamped / FIELD_HEIGHT_FT) * MAP_H;
}

function ftToPxDist(ft: number): number {
  return (ft / FIELD_WIDTH_FT) * MAP_W;
}

function pxToFt(pxX: number, pxY: number): [number, number] {
  const rawX = ((pxX - PAD_LEFT) / MAP_W) * FIELD_WIDTH_FT;
  const rawY = ((pxY - PAD_TOP) / MAP_H) * FIELD_HEIGHT_FT;
  return [
    Math.round(Math.max(2, Math.min(FIELD_WIDTH_FT - 2, rawX))),
    Math.round(Math.max(2, Math.min(FIELD_HEIGHT_FT - 2, rawY))),
  ];
}

function distanceFtBetween(a: [number, number], b: [number, number]): number {
  return Math.round(Math.hypot(b[0] - a[0], b[1] - a[1]));
}

function segmentIntersectsCircle(
  p1: [number, number],
  p2: [number, number],
  center: [number, number],
  radius: number
): boolean {
  const dx = p2[0] - p1[0];
  const dy = p2[1] - p1[1];
  const lenSq = dx * dx + dy * dy;
  if (lenSq === 0) return Math.hypot(center[0] - p1[0], center[1] - p1[1]) <= radius;
  let t = ((center[0] - p1[0]) * dx + (center[1] - p1[1]) * dy) / lenSq;
  t = Math.max(0, Math.min(1, t));
  const projX = p1[0] + t * dx;
  const projY = p1[1] + t * dy;
  return Math.hypot(center[0] - projX, center[1] - projY) <= radius;
}

const DEFAULT_OBSTACLES: SceneObstacle[] = [
  {
    id: 'obs-pillar-north',
    name: 'Pilares Antigos de Pedra',
    obstacle_type: 'cover',
    x: 25,
    y: 12,
    radius_ft: 4.5,
    mechanical_effect: '🛡️ Cobertura Parcial (+2 CA e +2 em Salvaguardas de DES)',
  },
  {
    id: 'obs-rubble-center',
    name: 'Escombros & Raízes Retorcidas',
    obstacle_type: 'difficult_terrain',
    x: 31,
    y: 26,
    radius_ft: 6,
    mechanical_effect: '🌿 Terreno Difícil (Cada 1 pé percorrido custa 2 pés de movimento)',
  },
  {
    id: 'obs-hazard-pit',
    name: 'Fosso de Estacas / Braseiro',
    obstacle_type: 'hazard',
    x: 40,
    y: 17,
    radius_ft: 4,
    mechanical_effect: '⚠️ Perigo Ambiental (Teste de DES CD 12 ou 1d6 de dano ao atravessar)',
  },
  {
    id: 'obs-rock-south',
    name: 'Rocha Fendida (Trincheira)',
    obstacle_type: 'cover',
    x: 20,
    y: 31,
    radius_ft: 4,
    mechanical_effect: '🛡️ Cobertura Tática (+2 CA contra ataques à distância)',
  },
];

export const TacticalBattleCanvas: FC<TacticalBattleCanvasProps> = ({
  livingScene,
  party = [],
  activeCharacter,
  selectedTargetId,
  onSelectTarget,
  onMoveParticipant,
  onAttackTarget,
  onEndTurn,
  onFleeBattle,
  isCombatMode = true,
}) => {
  const svgRef = useRef<SVGSVGElement | null>(null);
  const [hoverFt, setHoverFt] = useState<[number, number] | null>(null);
  const [hoveredParticipantId, setHoveredParticipantId] = useState<string | null>(null);
  const [hoveredObstacleId, setHoveredObstacleId] = useState<string | null>(null);

  // Normaliza obstáculos (usa os da cena ou fallback rico)
  const obstacles: SceneObstacle[] = useMemo(() => {
    if (livingScene?.obstacles && livingScene.obstacles.length > 0) {
      return livingScene.obstacles;
    }
    return DEFAULT_OBSTACLES;
  }, [livingScene?.obstacles]);

  // Normaliza participantes (garantindo que heróis e inimigos tenham coordenadas em pés 60x40, HP, CA e iniciativa detalhada)
  const normalizedParticipants: TurnParticipantData[] = useMemo(() => {
    const rawList = livingScene?.participants || [];
    const heroList = party.length > 0 ? party : activeCharacter ? [activeCharacter] : [];

    // Verifica se as posições vindas do backend estão em coordenadas pequenas legadas (0..5)
    const allSmallCoords =
      rawList.length > 0 &&
      rawList.every(
        (p) => !p.position || (p.position[0] <= 5 && p.position[1] <= 5)
      );

    let result: TurnParticipantData[] = [];

    if (rawList.length > 0) {
      let playerIdx = 0;
      let enemyIdx = 0;

      result = rawList.map((p) => {
        const matchedHero = p.is_player
          ? heroList.find(
              (h) =>
                h.id === p.id ||
                h.name.toLowerCase() === p.name.toLowerCase()
            )
          : undefined;

        const maxHp =
          p.max_hp ??
          matchedHero?.vitals.hpMax ??
          (p.is_player ? 12 : 12);
        const curHp =
          p.current_hp ??
          matchedHero?.vitals.hpCurrent ??
          maxHp;
        const ac =
          p.armor_class ??
          matchedHero?.vitals.armorClass ??
          (p.is_player ? 15 : 13);
        const initMod =
          p.initiative_modifier ??
          matchedHero?.vitals.initiativeModifier ??
          2;
        const initRoll =
          p.initiative_roll ??
          Math.max(1, Math.min(20, p.initiative - initMod));
        const isDead = Boolean(p.is_dead || curHp <= 0);
        const isHostile = p.is_player
          ? false
          : p.is_hostile !== undefined
          ? p.is_hostile
          : p.token_role !== 'ally';

        let pos: [number, number];
        if (p.position && !allSmallCoords) {
          pos = [
            Math.max(3, Math.min(FIELD_WIDTH_FT - 3, p.position[0])),
            Math.max(4, Math.min(FIELD_HEIGHT_FT - 4, p.position[1])),
          ];
        } else if (p.position && allSmallCoords) {
          if (p.is_player) {
            pos = [10 + (playerIdx % 2) * 5, 9 + playerIdx * 8];
          } else {
            pos = [44 + (enemyIdx % 2) * 6, 9 + enemyIdx * 9];
          }
        } else {
          if (p.is_player) {
            pos = [10 + (playerIdx % 2) * 5, 9 + playerIdx * 8];
          } else {
            pos = [44 + (enemyIdx % 2) * 6, 9 + enemyIdx * 9];
          }
        }

        if (p.is_player) playerIdx++;
        else enemyIdx++;

        return {
          ...p,
          is_hostile: isHostile,
          current_hp: Math.max(0, curHp),
          max_hp: Math.max(1, maxHp),
          armor_class: ac,
          initiative_modifier: initMod,
          initiative_roll: initRoll,
          is_dead: isDead,
          position: pos,
          actions_budget: p.actions_budget || {
            action_used: false,
            bonus_action_used: false,
            reaction_used: false,
            movement_remaining_ft: 30,
          },
        };
      });
    } else {
      // Fallback caso a cena ainda não tenha participantes sincronizados
      result = heroList.map((h, idx) => ({
        id: h.id,
        name: h.name,
        is_player: true,
        is_hostile: false,
        initiative: 14 + h.vitals.initiativeModifier - idx,
        initiative_roll: 14 - idx,
        initiative_modifier: h.vitals.initiativeModifier,
        current_hp: h.vitals.hpCurrent,
        max_hp: h.vitals.hpMax,
        armor_class: h.vitals.armorClass,
        is_dead: h.vitals.hpCurrent <= 0,
        position: [10 + (idx % 2) * 5, 9 + idx * 8] as [number, number],
        actions_budget: {
          action_used: false,
          bonus_action_used: false,
          reaction_used: false,
          movement_remaining_ft: h.vitals.speedFeet || 30,
        },
      }));
    }

    // Se não houver nenhum participante hostil na lista, mas active_npcs tiver nomes, adiciona tokens de referência
    const hasNonPlayers = result.some((p) => !p.is_player);
    if (!hasNonPlayers && livingScene?.active_npcs && livingScene.active_npcs.length > 0) {
      livingScene.active_npcs.forEach((npcName, idx) => {
        result.push({
          id: `npc-hostile-${idx + 1}`,
          name: npcName,
          is_player: false,
          is_hostile: true,
          initiative: 13 - idx * 2,
          initiative_roll: 11 - idx * 2,
          initiative_modifier: 2,
          current_hp: 12,
          max_hp: 12,
          armor_class: 13,
          is_dead: false,
          position: [45 + (idx % 2) * 6, 10 + idx * 10],
          actions_budget: {
            action_used: false,
            bonus_action_used: false,
            reaction_used: false,
            movement_remaining_ft: 30,
          },
        });
      });
    }

    return result;
  }, [livingScene?.participants, livingScene?.active_npcs, party, activeCharacter]);

  const currentIdx =
    normalizedParticipants.length > 0
      ? (livingScene?.current_participant_index ?? 0) % normalizedParticipants.length
      : 0;

  const currentTurnParticipant = normalizedParticipants[currentIdx] || normalizedParticipants[0];

  // Participante controlado pelo jogador para cálculo de distância e movimento
  const activeHeroParticipant = useMemo(() => {
    if (currentTurnParticipant?.is_player && !currentTurnParticipant.is_dead) {
      return currentTurnParticipant;
    }
    if (activeCharacter) {
      const found = normalizedParticipants.find(
        (p) =>
          p.is_player &&
          (p.id === activeCharacter.id ||
            p.name.toLowerCase() === activeCharacter.name.toLowerCase())
      );
      if (found) return found;
    }
    return normalizedParticipants.find((p) => p.is_player) || normalizedParticipants[0];
  }, [currentTurnParticipant, activeCharacter, normalizedParticipants]);

  const livingHostiles = useMemo(
    () => normalizedParticipants.filter((p) => !p.is_player && p.is_hostile !== false && !p.is_dead),
    [normalizedParticipants]
  );

  // Alvo selecionado efetivo (prioriza selectedTargetId se vivo, senão o inimigo vivo mais próximo)
  const effectiveTarget = useMemo(() => {
    if (selectedTargetId) {
      const match = normalizedParticipants.find((p) => p.id === selectedTargetId && !p.is_dead);
      if (match) return match;
    }
    if (livingHostiles.length === 0) return null;
    if (!activeHeroParticipant?.position) return livingHostiles[0];
    const heroPos = activeHeroParticipant.position;
    return [...livingHostiles].sort(
      (a, b) =>
        distanceFtBetween(heroPos, a.position || [45, 20]) -
        distanceFtBetween(heroPos, b.position || [45, 20])
    )[0];
  }, [selectedTargetId, normalizedParticipants, livingHostiles, activeHeroParticipant]);

  const heroPos: [number, number] = activeHeroParticipant?.position || [12, 20];
  const movementRemainingFt =
    activeHeroParticipant?.actions_budget?.movement_remaining_ft ?? 30;

  // Ponto alvo da régua de distância (hover do mouse ou alvo selecionado)
  const hoveredParticipant = useMemo(
    () => normalizedParticipants.find((p) => p.id === hoveredParticipantId) || null,
    [normalizedParticipants, hoveredParticipantId]
  );

  const hoveredObstacle = useMemo(
    () => obstacles.find((o) => o.id === hoveredObstacleId) || null,
    [obstacles, hoveredObstacleId]
  );

  const rulerTargetFt: [number, number] | null = useMemo(() => {
    if (hoveredParticipant?.position && hoveredParticipant.id !== activeHeroParticipant?.id) {
      return hoveredParticipant.position;
    }
    if (hoverFt) return hoverFt;
    if (effectiveTarget?.position) return effectiveTarget.position;
    return null;
  }, [hoveredParticipant, hoverFt, effectiveTarget, activeHeroParticipant]);

  const rulerDistanceFt = useMemo(() => {
    if (!rulerTargetFt) return 0;
    return distanceFtBetween(heroPos, rulerTargetFt);
  }, [heroPos, rulerTargetFt]);

  // Verifica se algum obstáculo de cobertura está na linha de visão
  const intersectingObstacle = useMemo(() => {
    if (!rulerTargetFt) return null;
    return (
      obstacles.find(
        (obs) =>
          (obs.obstacle_type === 'cover' || obs.obstacle_type === 'chokepoint') &&
          segmentIntersectsCircle(heroPos, rulerTargetFt, [obs.x, obs.y], obs.radius_ft)
      ) || null
    );
  }, [heroPos, rulerTargetFt, obstacles]);

  const handleSvgMouseMove = (e: MouseEvent<SVGSVGElement>) => {
    if (!svgRef.current) return;
    const rect = svgRef.current.getBoundingClientRect();
    const scaleX = SVG_WIDTH / rect.width;
    const scaleY = SVG_HEIGHT / rect.height;
    const pxX = (e.clientX - rect.left) * scaleX;
    const pxY = (e.clientY - rect.top) * scaleY;
    if (
      pxX >= PAD_LEFT &&
      pxX <= SVG_WIDTH - PAD_RIGHT &&
      pxY >= PAD_TOP &&
      pxY <= SVG_HEIGHT - PAD_BOTTOM
    ) {
      setHoverFt(pxToFt(pxX, pxY));
    } else {
      setHoverFt(null);
    }
  };

  const handleSvgClick = (e: MouseEvent<SVGSVGElement>) => {
    if (!svgRef.current || !onMoveParticipant) return;
    // Se clicou num token ou obstáculo que já tratou stopPropagation, ignora
    const rect = svgRef.current.getBoundingClientRect();
    const scaleX = SVG_WIDTH / rect.width;
    const scaleY = SVG_HEIGHT / rect.height;
    const pxX = (e.clientX - rect.left) * scaleX;
    const pxY = (e.clientY - rect.top) * scaleY;
    if (
      pxX < PAD_LEFT ||
      pxX > SVG_WIDTH - PAD_RIGHT ||
      pxY < PAD_TOP ||
      pxY > SVG_HEIGHT - PAD_BOTTOM
    ) {
      return;
    }
    const [targetX, targetY] = pxToFt(pxX, pxY);
    onMoveParticipant(targetX, targetY, activeHeroParticipant?.id);
  };

  const handleMoveAdjacentToTarget = (target: TurnParticipantData) => {
    if (!onMoveParticipant || !target.position) return;
    const [tx, ty] = target.position;
    const [hx, hy] = heroPos;
    const dx = tx - hx;
    const dy = ty - hy;
    const dist = Math.hypot(dx, dy);
    if (dist <= 5) return;
    // Ponto a 4.5 pés antes do alvo na direção do vetor
    const stopDist = Math.max(0, Math.min(movementRemainingFt, dist - 4.5));
    const ratio = stopDist / dist;
    const destX = Math.round(hx + dx * ratio);
    const destY = Math.round(hy + dy * ratio);
    onMoveParticipant(destX, destY, activeHeroParticipant?.id);
  };

  const roundNumber = livingScene?.round_number || 1;
  const inActiveCombat =
    (livingScene?.in_combat !== false || isCombatMode) && livingHostiles.length > 0;

  return (
    <div
      data-testid="tactical-battle-canvas"
      style={{
        display: 'flex',
        flexDirection: 'column',
        gap: '12px',
        width: '100%',
        backgroundColor: '#090c12',
        border: inActiveCombat
          ? '1px solid rgba(239, 68, 68, 0.45)'
          : '1px solid rgba(234, 179, 8, 0.3)',
        borderRadius: '12px',
        padding: '14px',
        boxShadow: '0 10px 30px rgba(0, 0, 0, 0.65)',
      }}
    >
      {/* =====================================================================
          1. BARRA DE ORDEM DE INICIATIVA REAL (INITIATIVE TRACKER)
         ===================================================================== */}
      <div
        style={{
          backgroundColor: '#0e131d',
          border: '1px solid #1e293b',
          borderRadius: '10px',
          padding: '10px 12px',
          display: 'flex',
          flexDirection: 'column',
          gap: '10px',
        }}
      >
        <div
          style={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            flexWrap: 'wrap',
            gap: '8px',
            borderBottom: '1px solid rgba(255, 255, 255, 0.07)',
            paddingBottom: '8px',
          }}
        >
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', flexWrap: 'wrap' }}>
            <span
              style={{
                backgroundColor: inActiveCombat
                  ? 'rgba(239, 68, 68, 0.2)'
                  : 'rgba(16, 185, 129, 0.2)',
                border: inActiveCombat ? '1px solid #ef4444' : '1px solid #10b981',
                color: inActiveCombat ? '#fca5a5' : '#6ee7b7',
                padding: '3px 9px',
                borderRadius: '6px',
                fontSize: '11px',
                fontWeight: 800,
                display: 'inline-flex',
                alignItems: 'center',
                gap: '5px',
                letterSpacing: '0.04em',
              }}
            >
              {inActiveCombat ? (
                <>
                  <Lock size={12} color="#ef4444" /> 🔒 EM BATALHA • RODADA #{roundNumber}
                </>
              ) : (
                <>
                  <Sparkles size={12} color="#10b981" /> 🏆 CAMPO DE BATALHA • RODADA #{roundNumber}
                </>
              )}
            </span>

            <span style={{ fontSize: '12px', color: '#cbd5e1', fontWeight: 700 }}>
              Ordem de Iniciativa (d20 + Modificador de Destreza):
            </span>
          </div>

          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', flexWrap: 'wrap' }}>
            {activeHeroParticipant && (
              <div
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  fontSize: '11px',
                  backgroundColor: '#131b28',
                  padding: '4px 10px',
                  borderRadius: '6px',
                  border: '1px solid #26354d',
                }}
              >
                <span style={{ color: '#94a3b8' }}>Orçamento ({activeHeroParticipant.name}):</span>
                <span
                  style={{
                    color: activeHeroParticipant.actions_budget.action_used ? '#f87171' : '#34d399',
                    fontWeight: 800,
                  }}
                >
                  Ação: {activeHeroParticipant.actions_budget.action_used ? 'Gasta' : 'Disp.'}
                </span>
                <span style={{ color: '#475569' }}>•</span>
                <span
                  style={{
                    color: activeHeroParticipant.actions_budget.bonus_action_used ? '#f87171' : '#fb923c',
                    fontWeight: 700,
                  }}
                >
                  Bônus: {activeHeroParticipant.actions_budget.bonus_action_used ? 'Gasta' : 'Disp.'}
                </span>
                <span style={{ color: '#475569' }}>•</span>
                <span style={{ color: '#38bdf8', fontWeight: 700 }}>
                  Mov: {movementRemainingFt}/30ft
                </span>
              </div>
            )}

            {onEndTurn && (
              <button
                type="button"
                onClick={() => onEndTurn(activeHeroParticipant?.id)}
                data-testid="canvas-end-turn-btn"
                style={{
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '5px',
                  padding: '5px 12px',
                  borderRadius: '6px',
                  backgroundColor: '#eab308',
                  color: '#000',
                  border: 'none',
                  fontSize: '11.5px',
                  fontWeight: 800,
                  cursor: 'pointer',
                  boxShadow: '0 0 12px rgba(234, 179, 8, 0.35)',
                }}
                title="Encerrar turno atual, processar ações de inimigos e avançar na fila de iniciativa"
              >
                <SkipForward size={13} /> ⏭️ Passar / Encerrar Turno
              </button>
            )}
          </div>
        </div>

        {/* Cards da Fila de Iniciativa */}
        <div
          style={{
            display: 'grid',
            gridTemplateColumns: 'repeat(auto-fill, minmax(175px, 1fr))',
            gap: '8px',
          }}
        >
          {normalizedParticipants.map((p, idx) => {
            const isCurrentTurn = idx === currentIdx && !p.is_dead;
            const isSelectedTarget = effectiveTarget?.id === p.id && !p.is_player && !p.is_dead;
            const curHp = p.current_hp ?? 12;
            const maxHp = Math.max(1, p.max_hp ?? 12);
            const hpPct = Math.max(0, Math.min(100, Math.round((curHp / maxHp) * 100)));
            const hpBarColor = p.is_dead
              ? '#475569'
              : hpPct > 50
              ? p.is_player
                ? '#10b981'
                : '#ef4444'
              : hpPct > 25
              ? '#f59e0b'
              : '#dc2626';

            const rollVal = p.initiative_roll ?? Math.max(1, p.initiative - (p.initiative_modifier ?? 2));
            const modVal = p.initiative_modifier ?? 2;
            const modStr = modVal >= 0 ? `+${modVal}` : `${modVal}`;

            return (
              <div
                key={p.id || idx}
                onClick={() => {
                  if (!p.is_player && !p.is_dead && onSelectTarget) {
                    onSelectTarget(p);
                  }
                }}
                data-testid={`initiative-card-${idx}`}
                style={{
                  backgroundColor: p.is_dead
                    ? 'rgba(15, 23, 42, 0.4)'
                    : isCurrentTurn
                    ? p.is_player
                      ? 'rgba(234, 179, 8, 0.16)'
                      : 'rgba(239, 68, 68, 0.18)'
                    : isSelectedTarget
                    ? 'rgba(225, 29, 72, 0.14)'
                    : '#131924',
                  border: p.is_dead
                    ? '1px dashed #334155'
                    : isCurrentTurn
                    ? p.is_player
                      ? '2px solid #facc15'
                      : '2px solid #ef4444'
                    : isSelectedTarget
                    ? '1.5px solid #f43f5e'
                    : '1px solid #1e293b',
                  borderRadius: '8px',
                  padding: '8px 10px',
                  opacity: p.is_dead ? 0.52 : 1,
                  cursor: !p.is_player && !p.is_dead ? 'pointer' : 'default',
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '4px',
                  position: 'relative',
                  boxShadow: isCurrentTurn
                    ? '0 0 14px rgba(234, 179, 8, 0.25)'
                    : 'none',
                  transition: 'all 0.15s ease',
                }}
                title={
                  !p.is_player && !p.is_dead
                    ? `Clique para selecionar ${p.name} como Alvo Prioritário`
                    : `${p.name} (Iniciativa ${p.initiative})`
                }
              >
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', gap: '4px' }}>
                  <div style={{ display: 'flex', alignItems: 'center', gap: '5px', minWidth: 0 }}>
                    <span
                      style={{
                        fontSize: '10px',
                        fontWeight: 900,
                        padding: '1px 5px',
                        borderRadius: '4px',
                        backgroundColor: isCurrentTurn ? '#facc15' : '#1e293b',
                        color: isCurrentTurn ? '#000' : '#94a3b8',
                        fontFamily: 'monospace',
                      }}
                    >
                      {idx + 1}º
                    </span>
                    <span
                      style={{
                        fontSize: '12px',
                        fontWeight: 800,
                        color: p.is_dead
                          ? '#64748b'
                          : p.is_player
                          ? '#fef08a'
                          : '#fca5a5',
                        textDecoration: p.is_dead ? 'line-through' : 'none',
                        overflow: 'hidden',
                        textOverflow: 'ellipsis',
                        whiteSpace: 'nowrap',
                      }}
                    >
                      {p.name}
                    </span>
                  </div>

                  {p.is_dead ? (
                    <span
                      style={{
                        fontSize: '9.5px',
                        fontWeight: 800,
                        color: '#f87171',
                        backgroundColor: 'rgba(127, 29, 29, 0.4)',
                        padding: '1px 5px',
                        borderRadius: '4px',
                      }}
                    >
                      💀 ABATIDO
                    </span>
                  ) : isCurrentTurn ? (
                    <span
                      style={{
                        fontSize: '9px',
                        fontWeight: 900,
                        color: '#000',
                        backgroundColor: '#facc15',
                        padding: '1px 5px',
                        borderRadius: '4px',
                      }}
                    >
                      ⚡ VEZ
                    </span>
                  ) : isSelectedTarget ? (
                    <span
                      style={{
                        fontSize: '9px',
                        fontWeight: 800,
                        color: '#fff',
                        backgroundColor: '#e11d48',
                        padding: '1px 5px',
                        borderRadius: '4px',
                      }}
                    >
                      🎯 ALVO
                    </span>
                  ) : null}
                </div>

                {/* Cálculo explícito da Iniciativa */}
                <div
                  style={{
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'center',
                    fontSize: '10.5px',
                    color: '#94a3b8',
                    fontFamily: 'monospace',
                  }}
                >
                  <span>
                    🎲 Inic: <strong style={{ color: '#e2e8f0' }}>{p.initiative}</strong> ({rollVal}
                    {modStr})
                  </span>
                  <span style={{ color: '#cbd5e1', fontWeight: 700 }}>🛡️ CA {p.armor_class ?? 13}</span>
                </div>

                {/* Barra de PV Real */}
                <div style={{ display: 'flex', alignItems: 'center', gap: '6px', marginTop: '2px' }}>
                  <div
                    style={{
                      flex: 1,
                      height: '6px',
                      backgroundColor: '#090d14',
                      borderRadius: '3px',
                      overflow: 'hidden',
                      border: '1px solid rgba(255,255,255,0.07)',
                    }}
                  >
                    <div
                      style={{
                        width: `${hpPct}%`,
                        height: '100%',
                        backgroundColor: hpBarColor,
                        transition: 'width 0.25s ease',
                      }}
                    />
                  </div>
                  <span
                    style={{
                      fontSize: '10.5px',
                      fontWeight: 800,
                      fontFamily: 'monospace',
                      color: p.is_dead ? '#64748b' : '#f1f5f9',
                    }}
                  >
                    {curHp}/{maxHp} PV
                  </span>
                </div>
              </div>
            );
          })}
        </div>
      </div>

      {/* =====================================================================
          2. CAMPO DE BATALHA INTERATIVO (CANVAS SVG 60ft x 40ft)
         ===================================================================== */}
      <div
        style={{
          position: 'relative',
          width: '100%',
          backgroundColor: '#070a0f',
          borderRadius: '10px',
          border: '1px solid #1e293b',
          overflow: 'hidden',
        }}
      >
        {/* Banner flutuante superior de telemetria de distância (Altura Fixa Estrita - Sem Deslocamento de Layout) */}
        <div
          style={{
            height: '38px',
            minHeight: '38px',
            maxHeight: '38px',
            boxSizing: 'border-box',
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            flexWrap: 'nowrap',
            overflow: 'hidden',
            whiteSpace: 'nowrap',
            gap: '8px',
            padding: '0 14px',
            backgroundColor: '#0c111a',
            borderBottom: '1px solid #1e293b',
            fontSize: '11.5px',
          }}
        >
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              flexWrap: 'nowrap',
              overflow: 'hidden',
              minWidth: 0,
              flexShrink: 1,
            }}
          >
            <span
              style={{
                color: '#facc15',
                fontWeight: 700,
                display: 'inline-flex',
                alignItems: 'center',
                gap: '5px',
                whiteSpace: 'nowrap',
                flexShrink: 0,
              }}
            >
              <Compass size={13} /> {activeHeroParticipant?.name || 'Herói'}: ({heroPos[0]}ft, {heroPos[1]}ft)
            </span>
            {rulerTargetFt && (
              <span
                style={{
                  backgroundColor:
                    rulerDistanceFt <= 5
                      ? 'rgba(16, 185, 129, 0.16)'
                      : rulerDistanceFt <= movementRemainingFt
                      ? 'rgba(56, 189, 248, 0.16)'
                      : 'rgba(245, 158, 11, 0.16)',
                  border:
                    rulerDistanceFt <= 5
                      ? '1px solid #10b981'
                      : rulerDistanceFt <= movementRemainingFt
                      ? '1px solid #38bdf8'
                      : '1px solid #f59e0b',
                  color:
                    rulerDistanceFt <= 5
                      ? '#6ee7b7'
                      : rulerDistanceFt <= movementRemainingFt
                      ? '#7dd3fc'
                      : '#fde047',
                  padding: '2px 8px',
                  borderRadius: '5px',
                  fontWeight: 700,
                  fontFamily: 'monospace',
                  whiteSpace: 'nowrap',
                  overflow: 'hidden',
                  textOverflow: 'ellipsis',
                  flexShrink: 1,
                }}
              >
                📏 {rulerDistanceFt} pés ({(rulerDistanceFt * 0.3).toFixed(1)}m) •{' '}
                {rulerDistanceFt <= 5
                  ? '⚔️ Corpo-a-Corpo (5ft)'
                  : rulerDistanceFt <= movementRemainingFt
                  ? `🏃 No Alcance (${movementRemainingFt}ft)`
                  : `🏹 À Distância (${movementRemainingFt}ft)`}
              </span>
            )}
            {intersectingObstacle && (
              <span
                style={{
                  backgroundColor: 'rgba(56, 189, 248, 0.15)',
                  border: '1px solid #38bdf8',
                  color: '#bae6fd',
                  padding: '2px 8px',
                  borderRadius: '5px',
                  fontWeight: 700,
                  fontSize: '11px',
                  whiteSpace: 'nowrap',
                  overflow: 'hidden',
                  textOverflow: 'ellipsis',
                  flexShrink: 1,
                }}
              >
                🛡️ Linha cruza {intersectingObstacle.name} (+2 CA)
              </span>
            )}
          </div>

          <span
            style={{
              color: '#94a3b8',
              fontSize: '11px',
              whiteSpace: 'nowrap',
              overflow: 'hidden',
              textOverflow: 'ellipsis',
              flexShrink: 0,
            }}
          >
            💡 <strong>Clique no chão</strong> p/ mover ou <strong>duplo-clique no inimigo</strong> p/ atacar
          </span>
        </div>

        {/* Estilos CSS Inline para Animações Táticas */}
        <style>{`
          @keyframes tokenDefeated {
            0% { transform: scale(1.2); }
            50% { transform: scale(0.9); }
            100% { transform: scale(1); }
          }
        `}</style>

        {/* SVG Tático Responsivo (60ft x 40ft) */}
        <svg
          ref={svgRef}
          viewBox={`0 0 ${SVG_WIDTH} ${SVG_HEIGHT}`}
          onMouseMove={handleSvgMouseMove}
          onMouseLeave={() => setHoverFt(null)}
          onClick={handleSvgClick}
          style={{
            width: '100%',
            height: 'auto',
            maxHeight: '460px',
            display: 'block',
            cursor: 'crosshair',
            userSelect: 'none',
          }}
        >
          <defs>
            <radialGradient id="battlefieldGlow" cx="50%" cy="50%" r="60%">
              <stop offset="0%" stopColor="#131c2e" />
              <stop offset="70%" stopColor="#0b101b" />
              <stop offset="100%" stopColor="#06080d" />
            </radialGradient>
            <pattern id="hazardStripes" width="8" height="8" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
              <line x1="0" y1="0" x2="0" y2="8" stroke="rgba(239, 68, 68, 0.35)" strokeWidth="4" />
            </pattern>
            {/* Iluminação de Tocha / Visão no Escuro (60ft) */}
            <radialGradient
              id="torchlightVision"
              gradientUnits="userSpaceOnUse"
              cx={ftToPxX(heroPos[0])}
              cy={ftToPxY(heroPos[1])}
              r={ftToPxDist(60)}
            >
              <stop offset="0%" stopColor="#05070c" stopOpacity="0" />
              <stop offset="60%" stopColor="#05070c" stopOpacity="0.04" />
              <stop offset="85%" stopColor="#05070c" stopOpacity="0.32" />
              <stop offset="100%" stopColor="#05070c" stopOpacity="0.55" />
            </radialGradient>
          </defs>

          {/* Fundo da Arena */}
          <rect
            x={PAD_LEFT}
            y={PAD_TOP}
            width={MAP_W}
            height={MAP_H}
            rx={8}
            fill="url(#battlefieldGlow)"
            stroke="#26334d"
            strokeWidth={2}
          />

          {/* Zonas Táticas de Fundo */}
          <rect
            x={PAD_LEFT}
            y={PAD_TOP}
            width={MAP_W * 0.3}
            height={MAP_H}
            fill="rgba(56, 189, 248, 0.03)"
          />
          <rect
            x={PAD_LEFT + MAP_W * 0.65}
            y={PAD_TOP}
            width={MAP_W * 0.35}
            height={MAP_H}
            fill="rgba(239, 68, 68, 0.035)"
          />

          <text
            x={PAD_LEFT + MAP_W * 0.15}
            y={PAD_TOP + 18}
            fill="rgba(148, 163, 184, 0.25)"
            fontSize="10"
            fontWeight="800"
            textAnchor="middle"
            letterSpacing="1.5"
          >
            VANGUARDA DOS HERÓIS
          </text>
          <text
            x={PAD_LEFT + MAP_W * 0.48}
            y={PAD_TOP + 18}
            fill="rgba(148, 163, 184, 0.25)"
            fontSize="10"
            fontWeight="800"
            textAnchor="middle"
            letterSpacing="1.5"
          >
            ZONA DE ENGAJAMENTO CENTRAL
          </text>
          <text
            x={PAD_LEFT + MAP_W * 0.82}
            y={PAD_TOP + 18}
            fill="rgba(248, 113, 113, 0.28)"
            fontSize="10"
            fontWeight="800"
            textAnchor="middle"
            letterSpacing="1.5"
          >
            TERRITÓRIO HOSTIL
          </text>

          {/* Linhas de Referência de 5 em 5 pés (12 colunas x 8 linhas) */}
          {Array.from({ length: 13 }).map((_, i) => {
            const ft = i * 5;
            const x = ftToPxX(ft);
            return (
              <g key={`vgrid-${ft}`}>
                <line
                  x1={x}
                  y1={PAD_TOP}
                  x2={x}
                  y2={PAD_TOP + MAP_H}
                  stroke={ft % 15 === 0 ? 'rgba(148, 163, 184, 0.16)' : 'rgba(148, 163, 184, 0.06)'}
                  strokeWidth={ft % 15 === 0 ? 1.2 : 0.7}
                  strokeDasharray={ft % 15 === 0 ? undefined : '3,3'}
                />
                {ft % 10 === 0 && (
                  <text
                    x={x}
                    y={PAD_TOP - 8}
                    fill="#64748b"
                    fontSize="10"
                    fontFamily="monospace"
                    textAnchor="middle"
                  >
                    {ft}ft
                  </text>
                )}
              </g>
            );
          })}

          {Array.from({ length: 9 }).map((_, j) => {
            const ft = j * 5;
            const y = ftToPxY(ft);
            return (
              <g key={`hgrid-${ft}`}>
                <line
                  x1={PAD_LEFT}
                  y1={y}
                  x2={PAD_LEFT + MAP_W}
                  y2={y}
                  stroke={ft % 10 === 0 ? 'rgba(148, 163, 184, 0.16)' : 'rgba(148, 163, 184, 0.06)'}
                  strokeWidth={ft % 10 === 0 ? 1.2 : 0.7}
                  strokeDasharray={ft % 10 === 0 ? undefined : '3,3'}
                />
                {ft % 10 === 0 && (
                  <text
                    x={PAD_LEFT - 8}
                    y={y + 3}
                    fill="#64748b"
                    fontSize="10"
                    fontFamily="monospace"
                    textAnchor="end"
                  >
                    {ft}ft
                  </text>
                )}
              </g>
            );
          })}

          {/* Círculo de Alcance de Movimento Restante do Herói Ativo */}
          {activeHeroParticipant && movementRemainingFt > 0 && (
            <g>
              <ellipse
                cx={ftToPxX(heroPos[0])}
                cy={ftToPxY(heroPos[1])}
                rx={(movementRemainingFt / FIELD_WIDTH_FT) * MAP_W}
                ry={(movementRemainingFt / FIELD_HEIGHT_FT) * MAP_H}
                fill="rgba(56, 189, 248, 0.05)"
                stroke="rgba(56, 189, 248, 0.35)"
                strokeWidth={1.5}
                strokeDasharray="6,4"
              />
              {/* Anel de Alcance Corpo-a-Corpo (5ft) */}
              <ellipse
                cx={ftToPxX(heroPos[0])}
                cy={ftToPxY(heroPos[1])}
                rx={(5 / FIELD_WIDTH_FT) * MAP_W}
                ry={(5 / FIELD_HEIGHT_FT) * MAP_H}
                fill="rgba(234, 179, 8, 0.05)"
                stroke="rgba(234, 179, 8, 0.28)"
                strokeWidth={1}
                strokeDasharray="2,3"
              />
            </g>
          )}

          {/* Camada de Obstáculos Táticos e Terreno */}
          {obstacles.map((obs) => {
            const cx = ftToPxX(obs.x);
            const cy = ftToPxY(obs.y);
            const rx = (obs.radius_ft / FIELD_WIDTH_FT) * MAP_W;
            const ry = (obs.radius_ft / FIELD_HEIGHT_FT) * MAP_H;
            const isHovered = hoveredObstacleId === obs.id;

            const isCover = obs.obstacle_type === 'cover';
            const isHazard = obs.obstacle_type === 'hazard';
            const isDiff = obs.obstacle_type === 'difficult_terrain';

            const fill = isHazard
              ? 'url(#hazardStripes)'
              : isCover
              ? 'rgba(51, 65, 85, 0.72)'
              : isDiff
              ? 'rgba(16, 185, 129, 0.14)'
              : 'rgba(120, 53, 15, 0.35)';

            const stroke = isHazard
              ? '#ef4444'
              : isCover
              ? '#94a3b8'
              : isDiff
              ? '#10b981'
              : '#f59e0b';

            const iconLabel = isHazard ? '⚠️' : isCover ? '🛡️' : isDiff ? '🌿' : '🚧';

            return (
              <g
                key={obs.id}
                onMouseEnter={() => setHoveredObstacleId(obs.id)}
                onMouseLeave={() => setHoveredObstacleId(null)}
                style={{ cursor: 'help' }}
              >
                <ellipse
                  cx={cx}
                  cy={cy}
                  rx={rx}
                  ry={ry}
                  fill={fill}
                  stroke={stroke}
                  strokeWidth={isHovered ? 2.5 : 1.5}
                  strokeDasharray={isDiff ? '4,3' : undefined}
                />
                <text
                  x={cx}
                  y={cy + 3}
                  fontSize="13"
                  textAnchor="middle"
                >
                  {iconLabel}
                </text>
                <text
                  x={cx}
                  y={cy + ry + 12}
                  fill="#cbd5e1"
                  fontSize="9.5"
                  fontWeight="700"
                  textAnchor="middle"
                >
                  {obs.name}
                </text>
              </g>
            );
          })}

          {/* Ambiance de Masmorra: Fog of War / Tocha & Visão no Escuro (60 pés) */}
          <rect
            x={PAD_LEFT}
            y={PAD_TOP}
            width={MAP_W}
            height={MAP_H}
            rx={8}
            fill="url(#torchlightVision)"
            pointerEvents="none"
          />

          {/* Linha Tracejada Tática de Distância & Linha de Visão */}
          {rulerTargetFt && (
            <g pointerEvents="none">
              <line
                x1={ftToPxX(heroPos[0])}
                y1={ftToPxY(heroPos[1])}
                x2={ftToPxX(rulerTargetFt[0])}
                y2={ftToPxY(rulerTargetFt[1])}
                stroke={
                  rulerDistanceFt <= 5
                    ? '#10b981'
                    : rulerDistanceFt <= movementRemainingFt
                    ? '#38bdf8'
                    : '#f59e0b'
                }
                strokeWidth={2}
                strokeDasharray="6,4"
              />
              {/* Marcador no destino do cursor se não estiver sobre um participante */}
              {!hoveredParticipant && hoverFt && (
                <circle
                  cx={ftToPxX(rulerTargetFt[0])}
                  cy={ftToPxY(rulerTargetFt[1])}
                  r={5}
                  fill={rulerDistanceFt <= movementRemainingFt ? '#38bdf8' : '#f59e0b'}
                  opacity={0.8}
                />
              )}
              {/* Etiqueta de Distância no Ponto Médio da Linha */}
              {rulerDistanceFt > 0 && (
                <g
                  transform={`translate(${
                    (ftToPxX(heroPos[0]) + ftToPxX(rulerTargetFt[0])) / 2
                  }, ${(ftToPxY(heroPos[1]) + ftToPxY(rulerTargetFt[1])) / 2 - 10})`}
                >
                  <rect
                    x={-44}
                    y={-11}
                    width={88}
                    height={18}
                    rx={4}
                    fill="rgba(9, 13, 22, 0.92)"
                    stroke={
                      rulerDistanceFt <= 5
                        ? '#10b981'
                        : rulerDistanceFt <= movementRemainingFt
                        ? '#38bdf8'
                        : '#f59e0b'
                    }
                    strokeWidth={1}
                  />
                  <text
                    x={0}
                    y={2}
                    fill="#f8fafc"
                    fontSize="10"
                    fontWeight="800"
                    fontFamily="monospace"
                    textAnchor="middle"
                  >
                    {rulerDistanceFt}ft ({(rulerDistanceFt * 0.3).toFixed(1)}m)
                  </text>
                </g>
              )}
            </g>
          )}

          {/* Camada de Tokens: Participantes (Mortos primeiro, Vivos por cima) */}
          {[...normalizedParticipants]
            .sort((a, b) => {
              const aDead = Boolean(a.is_dead || (a.current_hp ?? 0) <= 0);
              const bDead = Boolean(b.is_dead || (b.current_hp ?? 0) <= 0);
              return aDead === bDead ? 0 : aDead ? -1 : 1;
            })
            .map((p) => {
              const pos = p.position || [20, 20];
              const cx = ftToPxX(pos[0]);
              const cy = ftToPxY(pos[1]);
              const isDead = Boolean(p.is_dead || (p.current_hp ?? 0) <= 0);
              const isCurrent = p.id === currentTurnParticipant?.id && !isDead;
              const isActiveHero = p.id === activeHeroParticipant?.id;
              const isTarget = effectiveTarget?.id === p.id && !p.is_player && !isDead;

              const curHp = Math.max(0, p.current_hp ?? 0);
              const maxHp = Math.max(1, p.max_hp ?? 12);
              const hpPct = Math.max(0, Math.min(1, curHp / maxHp));

              const tokenFill = isDead
                ? '#1e293b'
                : p.is_player
                ? isActiveHero
                  ? '#d97706'
                  : '#0284c7'
                : p.is_hostile === false
                ? '#059669'
                : '#be123c';

              const tokenStroke = isDead
                ? '#475569'
                : isCurrent
                ? '#facc15'
                : isTarget
                ? '#fda4af'
                : p.is_player
                ? '#fde047'
                : '#f87171';

              return (
                <g
                  key={p.id}
                  data-testid={`canvas-token-${p.id}`}
                  onMouseEnter={() => setHoveredParticipantId(p.id)}
                  onMouseLeave={() => setHoveredParticipantId(null)}
                  onClick={(e) => {
                    e.stopPropagation();
                    if (!p.is_player && !isDead && onSelectTarget) {
                      onSelectTarget(p);
                    }
                  }}
                  onDoubleClick={(e) => {
                    e.stopPropagation();
                    if (!p.is_player && !isDead) {
                      if (onSelectTarget) {
                        onSelectTarget(p);
                      }
                      if (onAttackTarget) {
                        const dist = distanceFtBetween(heroPos, p.position || [45, 20]);
                        onAttackTarget(p, dist);
                      }
                    }
                  }}
                  style={{
                    cursor: !p.is_player && !isDead ? 'pointer' : 'default',
                    opacity: isDead ? 0.4 : 1,
                    filter: isDead ? 'grayscale(100%)' : undefined,
                    transformOrigin: `${cx}px ${cy}px`,
                    animation: isDead ? 'tokenDefeated 0.45s ease-out' : undefined,
                    transition: 'opacity 0.25s ease, filter 0.25s ease',
                  }}
                >
                  {/* Halo de Turno Ativo ou Alvo Selecionado */}
                  {(isCurrent || isTarget) && !isDead && (
                    <circle
                      cx={cx}
                      cy={cy}
                      r={24}
                      fill="none"
                      stroke={isCurrent ? '#facc15' : '#f43f5e'}
                      strokeWidth={2}
                      strokeDasharray={isTarget ? '4,3' : undefined}
                    />
                  )}

                  {/* Anel Indicador de Morte / Derrota */}
                  {isDead && (
                    <circle
                      cx={cx}
                      cy={cy}
                      r={22}
                      fill="none"
                      stroke="#64748b"
                      strokeWidth={1.5}
                      strokeDasharray="3,3"
                      opacity={0.7}
                    />
                  )}

                  {/* Corpo do Token */}
                  <circle
                    cx={cx}
                    cy={cy}
                    r={17}
                    fill={tokenFill}
                    stroke={tokenStroke}
                    strokeWidth={isCurrent || isTarget ? 3 : 2}
                  />

                  {/* Iniciais ou Ícone de Caveira se Morto */}
                  <text
                    x={cx}
                    y={cy + 4}
                    fill="#ffffff"
                    fontSize={isDead ? '13' : '11'}
                    fontWeight="900"
                    textAnchor="middle"
                  >
                    {isDead
                      ? '💀'
                      : p.name
                          .split(' ')
                          .map((w) => w[0])
                          .join('')
                          .slice(0, 2)
                          .toUpperCase()}
                  </text>

                  {/* Mini-Badge de CA no canto do token */}
                  {!isDead && (
                    <g transform={`translate(${cx + 13}, ${cy - 13})`}>
                      <circle r={8} fill="#0f172a" stroke="#475569" strokeWidth={1} />
                      <text
                        x={0}
                        y={3}
                        fill="#e2e8f0"
                        fontSize="8.5"
                        fontWeight="800"
                        textAnchor="middle"
                      >
                        {p.armor_class ?? 13}
                      </text>
                    </g>
                  )}

                  {/* Barra de PV acima do Token */}
                  <g transform={`translate(${cx - 22}, ${cy - 31})`}>
                    <rect
                      x={0}
                      y={0}
                      width={44}
                      height={6}
                      rx={3}
                      fill="#090d14"
                      stroke="#334155"
                      strokeWidth={0.8}
                    />
                    <rect
                      x={0}
                      y={0}
                      width={44 * hpPct}
                      height={6}
                      rx={3}
                      fill={
                        isDead
                          ? '#475569'
                          : p.is_player
                          ? '#22c55e'
                          : '#ef4444'
                      }
                    />
                  </g>

                  {/* Nome e PV abaixo do Token */}
                  <g transform={`translate(${cx}, ${cy + 30})`}>
                    <rect
                      x={-48}
                      y={-10}
                      width={96}
                      height={15}
                      rx={4}
                      fill="rgba(9, 13, 22, 0.86)"
                    />
                    <text
                      x={0}
                      y={1}
                      fill={
                        isDead
                          ? '#94a3b8'
                          : p.is_player
                          ? '#fef08a'
                          : '#fecdd3'
                      }
                      fontSize="9.5"
                      fontWeight="800"
                      textAnchor="middle"
                    >
                      {p.name.length > 14 ? `${p.name.slice(0, 13)}…` : p.name} ({curHp}/{maxHp})
                    </text>
                  </g>
                </g>
              );
            })}
        </svg>

        {/* HUD Card Overlay Flutuante: Tooltip de Participante / Obstáculo (Sem Deslocamento de Layout) */}
        {(hoveredParticipant || hoveredObstacle) && (
          <div
            data-testid="canvas-hud-overlay"
            style={{
              position: 'absolute',
              bottom: '12px',
              left: '12px',
              maxWidth: '380px',
              backgroundColor: 'rgba(11, 16, 27, 0.94)',
              border: hoveredParticipant
                ? (hoveredParticipant.is_dead || (hoveredParticipant.current_hp ?? 0) <= 0)
                  ? '1px solid #475569'
                  : hoveredParticipant.is_player
                  ? '1px solid #38bdf8'
                  : '1px solid #f43f5e'
                : hoveredObstacle
                ? '1px solid #38bdf8'
                : '1px solid transparent',
              borderRadius: '8px',
              padding: '8px 12px',
              boxShadow: '0 6px 20px rgba(0, 0, 0, 0.65)',
              backdropFilter: 'blur(6px)',
              pointerEvents: 'none',
              zIndex: 10,
              opacity: hoveredParticipant || hoveredObstacle ? 1 : 0,
              transform: hoveredParticipant || hoveredObstacle ? 'translateY(0)' : 'translateY(6px)',
              transition: 'opacity 0.2s ease, transform 0.2s ease, border-color 0.2s ease',
            }}
          >
            {hoveredParticipant ? (
              <div>
                <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: '8px' }}>
                  <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                    <span style={{ fontSize: '13px' }}>
                      {hoveredParticipant.is_dead || (hoveredParticipant.current_hp ?? 0) <= 0
                        ? '💀'
                        : hoveredParticipant.is_player
                        ? '🛡️'
                        : '⚔️'}
                    </span>
                    <strong style={{ color: '#f8fafc', fontSize: '12px' }}>
                      {hoveredParticipant.name}
                    </strong>
                  </div>
                  <span
                    style={{
                      fontSize: '10px',
                      padding: '1px 6px',
                      borderRadius: '4px',
                      fontWeight: 700,
                      backgroundColor:
                        hoveredParticipant.is_dead || (hoveredParticipant.current_hp ?? 0) <= 0
                          ? 'rgba(100, 116, 139, 0.25)'
                          : hoveredParticipant.is_player
                          ? 'rgba(56, 189, 248, 0.2)'
                          : 'rgba(244, 63, 94, 0.2)',
                      color:
                        hoveredParticipant.is_dead || (hoveredParticipant.current_hp ?? 0) <= 0
                          ? '#94a3b8'
                          : hoveredParticipant.is_player
                          ? '#7dd3fc'
                          : '#fda4af',
                    }}
                  >
                    {hoveredParticipant.is_dead || (hoveredParticipant.current_hp ?? 0) <= 0
                      ? 'Derrotado'
                      : hoveredParticipant.id === currentTurnParticipant?.id
                      ? 'Turno Ativo'
                      : hoveredParticipant.is_player
                      ? 'Herói'
                      : 'Inimigo'}
                  </span>
                </div>

                {/* Status e Atributos: PV, CA, Distância */}
                <div
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: '10px',
                    marginTop: '5px',
                    fontSize: '11px',
                    fontFamily: 'monospace',
                    color: '#cbd5e1',
                  }}
                >
                  <span
                    style={{
                      color:
                        hoveredParticipant.is_dead || (hoveredParticipant.current_hp ?? 0) <= 0
                          ? '#94a3b8'
                          : (hoveredParticipant.current_hp ?? 0) <= (hoveredParticipant.max_hp ?? 1) * 0.25
                          ? '#ef4444'
                          : '#4ade80',
                      fontWeight: 700,
                    }}
                  >
                    ❤️ {Math.max(0, hoveredParticipant.current_hp ?? 0)}/{Math.max(1, hoveredParticipant.max_hp ?? 1)} PV
                  </span>
                  <span style={{ fontWeight: 700 }}>
                    🛡️ CA {hoveredParticipant.armor_class ?? 13}
                  </span>
                  <span style={{ color: '#facc15', fontWeight: 700 }}>
                    📏 {distanceFtBetween(heroPos, hoveredParticipant.position || [45, 20])}ft
                  </span>
                </div>

                {/* Dica de Ação Tática */}
                {!hoveredParticipant.is_player &&
                  !(hoveredParticipant.is_dead || (hoveredParticipant.current_hp ?? 0) <= 0) && (
                    <div style={{ color: '#94a3b8', fontSize: '10px', marginTop: '4px', fontStyle: 'italic' }}>
                      💡 Clique para selecionar alvo • Duplo-clique para atacar
                    </div>
                  )}
              </div>
            ) : hoveredObstacle ? (
              <div>
                <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                  <span style={{ fontSize: '13px' }}>
                    {hoveredObstacle.obstacle_type === 'hazard'
                      ? '⚠️'
                      : hoveredObstacle.obstacle_type === 'cover'
                      ? '🛡️'
                      : hoveredObstacle.obstacle_type === 'difficult_terrain'
                      ? '🌿'
                      : '🚧'}
                  </span>
                  <strong style={{ color: '#38bdf8', fontSize: '12px' }}>
                    {hoveredObstacle.name}
                  </strong>
                  <span style={{ fontSize: '10px', color: '#94a3b8', marginLeft: 'auto' }}>
                    ({hoveredObstacle.x}ft, {hoveredObstacle.y}ft)
                  </span>
                </div>
                <div style={{ color: '#cbd5e1', fontSize: '11px', marginTop: '3px' }}>
                  {hoveredObstacle.mechanical_effect}
                </div>
              </div>
            ) : null}
          </div>
        )}
      </div>

      {/* =====================================================================
          3. PAINEL INFERIOR DE COMANDO TÁTICO DIRETO (ALVO & AÇÕES DE MAPA)
         ===================================================================== */}
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          flexWrap: 'wrap',
          gap: '10px',
          backgroundColor: '#0e131d',
          border: '1px solid #1e293b',
          borderRadius: '10px',
          padding: '10px 14px',
        }}
      >
        {effectiveTarget ? (
          <>
            <div style={{ display: 'flex', alignItems: 'center', gap: '10px', flexWrap: 'wrap' }}>
              <span
                style={{
                  backgroundColor: 'rgba(225, 29, 72, 0.2)',
                  border: '1px solid #f43f5e',
                  color: '#fda4af',
                  padding: '4px 10px',
                  borderRadius: '6px',
                  fontSize: '11.5px',
                  fontWeight: 800,
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '5px',
                }}
              >
                <Crosshair size={13} /> 🎯 Alvo Selecionado: {effectiveTarget.name}
              </span>
              <span style={{ fontSize: '12px', color: '#f1f5f9', fontWeight: 700, fontFamily: 'monospace' }}>
                ❤️ {effectiveTarget.current_hp}/{effectiveTarget.max_hp} PV • 🛡️ CA{' '}
                {effectiveTarget.armor_class ?? 13} • 📏 Distância:{' '}
                {distanceFtBetween(heroPos, effectiveTarget.position || [45, 20])} ft
              </span>
            </div>

            <div style={{ display: 'flex', alignItems: 'center', gap: '8px', flexWrap: 'wrap' }}>
              {distanceFtBetween(heroPos, effectiveTarget.position || [45, 20]) > 5 &&
                movementRemainingFt > 0 && (
                  <button
                    type="button"
                    onClick={() => handleMoveAdjacentToTarget(effectiveTarget)}
                    style={{
                      display: 'inline-flex',
                      alignItems: 'center',
                      gap: '5px',
                      padding: '6px 12px',
                      borderRadius: '6px',
                      backgroundColor: 'rgba(56, 189, 248, 0.16)',
                      border: '1px solid #38bdf8',
                      color: '#7dd3fc',
                      fontSize: '11.5px',
                      fontWeight: 700,
                      cursor: 'pointer',
                    }}
                  >
                    <Footprints size={13} /> 🏃 Aproximar p/ Corpo-a-Corpo (5ft)
                  </button>
                )}

              {onAttackTarget && (
                <button
                  type="button"
                  disabled={Boolean(activeHeroParticipant?.actions_budget.action_used)}
                  onClick={() =>
                    onAttackTarget(
                      effectiveTarget,
                      distanceFtBetween(heroPos, effectiveTarget.position || [45, 20])
                    )
                  }
                  data-testid="canvas-attack-target-btn"
                  style={{
                    display: 'inline-flex',
                    alignItems: 'center',
                    gap: '6px',
                    padding: '6px 14px',
                    borderRadius: '6px',
                    backgroundColor: activeHeroParticipant?.actions_budget.action_used
                      ? '#1e293b'
                      : '#e11d48',
                    color: activeHeroParticipant?.actions_budget.action_used ? '#64748b' : '#fff',
                    border: 'none',
                    fontSize: '12px',
                    fontWeight: 800,
                    cursor: activeHeroParticipant?.actions_budget.action_used
                      ? 'not-allowed'
                      : 'pointer',
                    boxShadow: activeHeroParticipant?.actions_budget.action_used
                      ? 'none'
                      : '0 0 12px rgba(225, 29, 72, 0.4)',
                  }}
                >
                  <Sword size={13} />{' '}
                  {activeHeroParticipant?.actions_budget.action_used
                    ? 'Ação Principal Já Usada'
                    : `⚔️ Atacar ${effectiveTarget.name} (${distanceFtBetween(
                        heroPos,
                        effectiveTarget.position || [45, 20]
                      )}ft)`}
                </button>
              )}

              {inActiveCombat && onFleeBattle && (
                <button
                  type="button"
                  onClick={onFleeBattle}
                  style={{
                    display: 'inline-flex',
                    alignItems: 'center',
                    gap: '5px',
                    padding: '6px 12px',
                    borderRadius: '6px',
                    backgroundColor: 'rgba(245, 158, 11, 0.14)',
                    border: '1px solid #f59e0b',
                    color: '#fde047',
                    fontSize: '11.5px',
                    fontWeight: 700,
                    cursor: 'pointer',
                  }}
                  title="Tentar Retirada Tática (Teste CD 12) para desengajar e encerrar a batalha"
                >
                  🏃‍♂️ Retirada Tática (CD 12)
                </button>
              )}
            </div>
          </>
        ) : (
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', color: '#6ee7b7', fontSize: '12px', fontWeight: 700 }}>
            <Sparkles size={15} /> 🏆 Todos os inimigos hostis no mapa foram derrotados! Você pode saquear o ambiente ou concluir a cena.
          </div>
        )}
      </div>
    </div>
  );
};
