import { useState, useEffect, useMemo } from 'react';
import type { FC } from 'react';
import type {
  ActionEconomyBudget,
  SheetActionOption,
  ContextualScenarioOption,
  FreeformTextActionSubmission,
} from '../../types/game';
import type { TurnParticipantData } from '../../types/session';
import {
  Sword,
  Bot,
  Send,
  CheckCircle2,
  AlertTriangle,
  RotateCcw,
  Compass,
  Shield,
  Zap,
  Footprints,
  Target,
  Sparkles,
  Clock,
  Lock,
  SkipForward,
  Crosshair,
} from 'lucide-react';

export interface TurnPossibilitiesConsoleProps {
  budget: ActionEconomyBudget;
  sheetActions: SheetActionOption[];
  scenarioActions: ContextualScenarioOption[];
  onExecuteSheetAction: (action: SheetActionOption, target?: TurnParticipantData | null) => void;
  onExecuteScenarioAction: (scenario: ContextualScenarioOption) => void;
  onSubmitFreeformAction: (text: string) => void;
  onResetTurnBudget: () => void;
  recentSubmission?: FreeformTextActionSubmission | null;
  actionLog: string[];
  isCombatMode?: boolean;
  hasHostileTargets?: boolean;
  hostileNames?: string[];
  onToggleCombatMode?: (combat: boolean) => void;
  onResolveScene?: () => void;
  participants?: TurnParticipantData[];
  selectedTargetId?: string | null;
  onSelectTarget?: (target: TurnParticipantData) => void;
  onEndTurn?: () => void;
  onFleeBattle?: () => void;
}

export const TurnPossibilitiesConsole: FC<TurnPossibilitiesConsoleProps> = ({
  budget,
  sheetActions,
  scenarioActions,
  onExecuteSheetAction,
  onExecuteScenarioAction,
  onSubmitFreeformAction,
  onResetTurnBudget,
  recentSubmission,
  actionLog,
  isCombatMode: externalCombatMode,
  hasHostileTargets = false,
  hostileNames = [],
  onToggleCombatMode,
  onResolveScene,
  participants = [],
  selectedTargetId = null,
  onSelectTarget,
  onEndTurn,
  onFleeBattle,
}) => {
  // Controle de Modo: Combate Tático vs Exploração Livre
  const [internalCombatMode, setInternalCombatMode] = useState<boolean>(externalCombatMode ?? false);
  const combatMode = externalCombatMode !== undefined ? externalCombatMode : internalCombatMode;

  const hostileParticipants = useMemo(
    () => participants.filter((p) => !p.is_player && p.is_hostile !== false),
    [participants]
  );

  const livingHostiles = useMemo(
    () =>
      hostileParticipants.filter(
        (p) => !p.is_dead && (p.current_hp === undefined || p.current_hp > 0)
      ),
    [hostileParticipants]
  );

  const effectiveTarget: TurnParticipantData | null = useMemo(() => {
    if (selectedTargetId) {
      const found = livingHostiles.find((p) => p.id === selectedTargetId);
      if (found) return found;
    }
    return livingHostiles[0] || null;
  }, [selectedTargetId, livingHostiles]);

  // Trava de Batalha: se está em combate e existem inimigos vivos, não permite sair livremente
  const isBattleLocked =
    combatMode && (livingHostiles.length > 0 || (hostileParticipants.length === 0 && hasHostileTargets));

  const setCombatMode = (active: boolean) => {
    if (!active && isBattleLocked) {
      return;
    }
    setInternalCombatMode(active);
    if (active) {
      onResetTurnBudget();
    }
    if (onToggleCombatMode) {
      onToggleCombatMode(active);
    }
  };

  const handleEndTurnClick = () => {
    if (onEndTurn) {
      onEndTurn();
    } else {
      onResetTurnBudget();
    }
  };

  const handleFleeClick = () => {
    if (onFleeBattle) {
      onFleeBattle();
    } else {
      onExecuteSheetAction({
        id: 'flee',
        name: 'Retirada Tática: Tentar fugir e desengajar do combate',
        type: 'action',
        category: 'basic',
        description: 'Tenta fugir e desengajar da batalha ativa (Teste de Destreza/Atletismo CD 12).',
        hitBonusOrDc: 'CD 12',
        isAvailable: true,
      });
    }
  };

  // No modo Exploração: Apenas Via 2 (Cenário) e Via 3 (IA Livre). Ficha não existe aqui.
  // No modo Combate: Via 1 (Combate/Ficha), Via 2 (Táticas do Cenário) e Via 3 (Ação Livre).
  const [activeVia, setActiveVia] = useState<'via1' | 'via2' | 'via3'>(
    combatMode ? 'via1' : 'via2'
  );

  // Sincroniza a aba ativa ao alternar entre Exploração e Combate
  useEffect(() => {
    if (combatMode) {
      setActiveVia('via1');
    } else {
      // No modo exploração, se estivesse na via1, obrigatoriamente vai para via2
      setActiveVia('via2');
    }
  }, [combatMode]);

  const [freeformInput, setFreeformInput] = useState<string>('');
  const [isEvaluating, setIsEvaluating] = useState<boolean>(false);

  const handleFreeformSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!freeformInput.trim()) return;

    setIsEvaluating(true);
    setTimeout(() => {
      onSubmitFreeformAction(freeformInput);
      setFreeformInput('');
      setIsEvaluating(false);
    }, 600);
  };

  const handleQuickPrompt = (promptText: string) => {
    setFreeformInput(promptText);
  };

  const movementPercentage = Math.max(
    0,
    Math.min(100, (budget.movementRemainingFeet / Math.max(1, budget.movementTotalFeet)) * 100)
  );

  const isTurnBudgetSpent =
    !budget.actionAvailable ||
    !budget.bonusActionAvailable ||
    budget.movementRemainingFeet < budget.movementTotalFeet;

  return (
    <div
      style={{
        backgroundColor: 'var(--bg-card)',
        border: combatMode ? '1px solid rgba(220, 38, 38, 0.4)' : '1px solid var(--border-color)',
        borderRadius: '12px',
        padding: '16px',
        display: 'flex',
        flexDirection: 'column',
        gap: '14px',
        boxShadow: combatMode ? '0 0 20px rgba(220, 38, 38, 0.15)' : 'none',
        transition: 'all 0.3s ease',
      }}
    >
      {/* ========================================================
          BARRA SUPERIOR (MODO DE JOGO & ECONOMIA DE RECURSOS)
         ======================================================== */}
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          flexWrap: 'wrap',
          gap: '12px',
          backgroundColor: '#0c0f14',
          border: '1px solid var(--border-color)',
          borderRadius: '10px',
          padding: '10px 14px',
        }}
      >
        {/* Lado Esquerdo: Indicador de Modo e Trava de Batalha */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px', flexWrap: 'wrap' }}>
          <div
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: '6px',
              padding: '4px 10px',
              borderRadius: '20px',
              fontSize: '11px',
              fontWeight: 800,
              letterSpacing: '0.5px',
              textTransform: 'uppercase',
              backgroundColor: combatMode ? 'rgba(239, 68, 68, 0.15)' : 'rgba(56, 189, 248, 0.15)',
              border: combatMode ? '1px solid #ef4444' : '1px solid #38bdf8',
              color: combatMode ? '#fca5a5' : '#7dd3fc',
            }}
          >
            {combatMode ? (
              <>
                <Zap size={13} style={{ color: '#ef4444' }} /> Modo Combate (Por Turnos)
              </>
            ) : (
              <>
                <Compass size={13} style={{ color: '#38bdf8' }} /> Modo Exploração Livre
              </>
            )}
          </div>

          {/* Trava de Batalha Ativa OU Botão de Alternância quando não há inimigos vivos */}
          {isBattleLocked ? (
            <span
              data-testid="combat-locked-badge"
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '6px',
                padding: '4px 10px',
                borderRadius: '6px',
                backgroundColor: 'rgba(127, 29, 29, 0.35)',
                border: '1px solid rgba(239, 68, 68, 0.5)',
                color: '#fecaca',
                fontSize: '11px',
                fontWeight: 700,
              }}
              title="Não é possível sair de uma batalha inconclusiva sem derrotar os inimigos ou executar Retirada Tática (CD 12)"
            >
              <Lock size={12} color="#f87171" />
              🔒 EM BATALHA ATIVA (Derrote os inimigos ou execute Retirada Tática CD 12)
            </span>
          ) : (
            <button
              onClick={() => setCombatMode(!combatMode)}
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '6px',
                padding: '4px 10px',
                borderRadius: '6px',
                backgroundColor: '#161c26',
                border: '1px solid #2d3748',
                color: 'var(--text-muted)',
                fontSize: '11px',
                fontWeight: 600,
                cursor: 'pointer',
                transition: 'all 0.2s',
              }}
              title={combatMode ? 'Todos os inimigos foram derrotados — retornar à exploração livre' : 'Iniciar modo de combate tático por turnos'}
            >
              <Clock size={12} />
              {combatMode ? '🕊️ Combate Concluído: Voltar à Exploração' : '⚔️ Entrar em Combate por Turnos'}
            </button>
          )}
        </div>

        {/* Lado Direito: Economia de Ações (Apenas em Combate) ou Informação Narrativa (Exploração) */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '14px', flexWrap: 'wrap' }}>
          {combatMode ? (
            /* Em Combate: Orçamento Tático com Indicadores Visuais de Ação */
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px', flexWrap: 'wrap' }}>
              {/* Pip: Ação Principal */}
              <div
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '5px',
                  padding: '3px 8px',
                  borderRadius: '6px',
                  backgroundColor: budget.actionAvailable ? 'rgba(16, 185, 129, 0.15)' : '#1e1618',
                  border: budget.actionAvailable ? '1px solid #10b981' : '1px solid #451a20',
                  color: budget.actionAvailable ? '#34d399' : '#f87171',
                  fontSize: '11px',
                  fontWeight: 700,
                }}
                title="Ação Principal D&D 5e"
              >
                <span
                  style={{
                    display: 'inline-block',
                    width: '8px',
                    height: '8px',
                    borderRadius: '50%',
                    backgroundColor: budget.actionAvailable ? '#10b981' : '#ef4444',
                    boxShadow: budget.actionAvailable ? '0 0 6px #10b981' : 'none',
                  }}
                />
                Ação: {budget.actionAvailable ? 'Disponível' : 'Gasta'}
              </div>

              {/* Pip: Ação Bônus */}
              <div
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '5px',
                  padding: '3px 8px',
                  borderRadius: '6px',
                  backgroundColor: budget.bonusActionAvailable ? 'rgba(249, 115, 22, 0.15)' : '#1e1618',
                  border: budget.bonusActionAvailable ? '1px solid #f97316' : '1px solid #451a20',
                  color: budget.bonusActionAvailable ? '#fb923c' : '#f87171',
                  fontSize: '11px',
                  fontWeight: 700,
                }}
                title="Ação Bônus D&D 5e"
              >
                <span
                  style={{
                    display: 'inline-block',
                    width: 0,
                    height: 0,
                    borderLeft: '4px solid transparent',
                    borderRight: '4px solid transparent',
                    borderBottom: budget.bonusActionAvailable ? '8px solid #f97316' : '8px solid #7f1d1d',
                    filter: budget.bonusActionAvailable ? 'drop-shadow(0 0 4px #f97316)' : 'none',
                  }}
                />
                Bônus: {budget.bonusActionAvailable ? 'Disp.' : 'Gasta'}
              </div>

              {/* Pip: Reação */}
              <div
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '5px',
                  padding: '3px 8px',
                  borderRadius: '6px',
                  backgroundColor: budget.reactionAvailable ? 'rgba(168, 85, 247, 0.15)' : '#1e1618',
                  border: budget.reactionAvailable ? '1px solid #a855f7' : '1px solid #451a20',
                  color: budget.reactionAvailable ? '#c084fc' : '#f87171',
                  fontSize: '11px',
                  fontWeight: 700,
                }}
                title="Reação D&D 5e"
              >
                <Shield size={11} style={{ color: budget.reactionAvailable ? '#a855f7' : '#7f1d1d' }} />
                Reação: {budget.reactionAvailable ? 'Disp.' : 'Gasta'}
              </div>

              {/* Barra de Movimento */}
              <div
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  padding: '3px 8px',
                  borderRadius: '6px',
                  backgroundColor: '#131b26',
                  border: '1px solid #233147',
                  fontSize: '11px',
                  color: '#93c5fd',
                  fontWeight: 600,
                }}
                title={`Movimento: ${budget.movementRemainingFeet}/${budget.movementTotalFeet} pés`}
              >
                <Footprints size={12} style={{ color: '#60a5fa' }} />
                <span>Mov: {budget.movementRemainingFeet}/{budget.movementTotalFeet} ft</span>
                <div
                  style={{
                    width: '36px',
                    height: '5px',
                    backgroundColor: '#1e293b',
                    borderRadius: '3px',
                    overflow: 'hidden',
                  }}
                >
                  <div
                    style={{
                      width: `${movementPercentage}%`,
                      height: '100%',
                      backgroundColor: '#38bdf8',
                      transition: 'width 0.2s',
                    }}
                  />
                </div>
              </div>
            </div>
          ) : (
            /* Em Exploração: Sem Poluição Tática de Orçamento */
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px', fontSize: '12px', color: '#94a3b8' }}>
              <Sparkles size={14} style={{ color: '#eab308' }} />
              <span>Interações contextuais e diálogo com o mundo</span>
            </div>
          )}

          {/* Botão de Encerrar Turno (Passa a vez na Iniciativa e executa reação dos inimigos) */}
          {combatMode ? (
            <button
              onClick={handleEndTurnClick}
              data-testid="console-end-turn-btn"
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '6px',
                padding: '6px 14px',
                borderRadius: '6px',
                backgroundColor: isTurnBudgetSpent ? 'var(--accent-gold)' : '#1c222d',
                border: isTurnBudgetSpent ? '1px solid var(--accent-gold)' : '1px solid var(--border-color)',
                fontSize: '12px',
                color: isTurnBudgetSpent ? '#000' : '#fef08a',
                fontWeight: 800,
                boxShadow: isTurnBudgetSpent ? '0 0 12px rgba(234, 179, 8, 0.45)' : 'none',
                cursor: 'pointer',
                transition: 'all 0.2s',
              }}
              title="Encerrar seu turno e passar para o próximo na Ordem de Iniciativa"
            >
              <SkipForward size={13} />{' '}
              {isTurnBudgetSpent
                ? '⏭️ Encerrar Turno -> Próximo na Iniciativa'
                : '⏭️ Passar / Encerrar Turno'}
            </button>
          ) : (
            <button
              onClick={onResetTurnBudget}
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '6px',
                padding: '6px 12px',
                borderRadius: '6px',
                backgroundColor: '#161c26',
                border: '1px solid #2d3748',
                fontSize: '12px',
                color: 'var(--text-muted)',
                fontWeight: 600,
                cursor: 'pointer',
              }}
              title="Limpar estado e atualizar cena"
            >
              <RotateCcw size={12} /> Atualizar
            </button>
          )}
        </div>
      </div>

      {/* ========================================================
          NAVEGAÇÃO DAS ABAS
          Exploração: APENAS 2 ABAS (Via 2: Cenário & Via 3: IA Livre)
          Combate:    3 ABAS (Via 1: Ficha, Via 2: Cenário, Via 3: IA Livre)
         ======================================================== */}
      <div style={{ display: 'flex', gap: '8px' }}>
        {combatMode ? (
          /* MODO COMBATE: 3 Vias ativas, com Ficha/Combate em 1º Lugar */
          <>
            <button
              onClick={() => setActiveVia('via1')}
              style={{
                flex: 1,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                gap: '8px',
                padding: '10px',
                borderRadius: '8px',
                fontSize: '13px',
                fontWeight: 700,
                backgroundColor: activeVia === 'via1' ? 'var(--accent-gold)' : '#101318',
                color: activeVia === 'via1' ? '#000' : 'var(--text-muted)',
                border: activeVia === 'via1' ? '1px solid var(--accent-gold)' : '1px solid var(--border-color)',
                transition: 'all 0.2s',
              }}
            >
              <Sword size={16} /> 1. Mecânicas de Combate (Ficha)
            </button>

            <button
              onClick={() => setActiveVia('via2')}
              style={{
                flex: 1,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                gap: '8px',
                padding: '10px',
                borderRadius: '8px',
                fontSize: '13px',
                fontWeight: 700,
                backgroundColor: activeVia === 'via2' ? 'var(--accent-gold)' : '#101318',
                color: activeVia === 'via2' ? '#000' : 'var(--text-muted)',
                border: activeVia === 'via2' ? '1px solid var(--accent-gold)' : '1px solid var(--border-color)',
                transition: 'all 0.2s',
              }}
            >
              <Compass size={16} /> 2. Táticas do Cenário
            </button>

            <button
              onClick={() => setActiveVia('via3')}
              style={{
                flex: 1,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                gap: '8px',
                padding: '10px',
                borderRadius: '8px',
                fontSize: '13px',
                fontWeight: 700,
                backgroundColor: activeVia === 'via3' ? 'var(--accent-gold)' : '#101318',
                color: activeVia === 'via3' ? '#000' : 'var(--text-muted)',
                border: activeVia === 'via3' ? '1px solid var(--accent-gold)' : '1px solid var(--border-color)',
                transition: 'all 0.2s',
              }}
            >
              <Bot size={16} /> 3. Ação Criativa Livre (IA)
            </button>
          </>
        ) : (
          /* MODO EXPLORAÇÃO: Mecânicas da Ficha REMOVIDAS. Apenas Cenário e Texto Livre */
          <>
            <button
              onClick={() => setActiveVia('via2')}
              style={{
                flex: 1,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                gap: '8px',
                padding: '11px',
                borderRadius: '8px',
                fontSize: '13px',
                fontWeight: 700,
                backgroundColor: activeVia === 'via2' ? 'var(--accent-gold)' : '#101318',
                color: activeVia === 'via2' ? '#000' : 'var(--text-muted)',
                border: activeVia === 'via2' ? '1px solid var(--accent-gold)' : '1px solid var(--border-color)',
                transition: 'all 0.2s',
              }}
            >
              <Compass size={16} /> 1. Sugestões do Cenário (IA)
            </button>

            <button
              onClick={() => setActiveVia('via3')}
              style={{
                flex: 1,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                gap: '8px',
                padding: '11px',
                borderRadius: '8px',
                fontSize: '13px',
                fontWeight: 700,
                backgroundColor: activeVia === 'via3' ? 'var(--accent-gold)' : '#101318',
                color: activeVia === 'via3' ? '#000' : 'var(--text-muted)',
                border: activeVia === 'via3' ? '1px solid var(--accent-gold)' : '1px solid var(--border-color)',
                transition: 'all 0.2s',
              }}
            >
              <Bot size={16} /> 2. Interação Livre com o Mundo (IA)
            </button>
          </>
        )}
      </div>

      {/* ========================================================
          VIA 1: MECÂNICAS DA FICHA (EXCLUSIVA DO MODO COMBATE)
         ======================================================== */}
      {combatMode && activeVia === 'via1' && (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexWrap: 'wrap', gap: '8px' }}>
            <span style={{ fontSize: '12px', color: 'var(--text-muted)' }}>
              Ações táticas habilitadas pelo orçamento de combate e armas equipadas:
            </span>
            {isBattleLocked && (
              <button
                type="button"
                onClick={handleFleeClick}
                disabled={!budget.actionAvailable}
                data-testid="via1-flee-quick-btn"
                style={{
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '6px',
                  padding: '5px 12px',
                  borderRadius: '6px',
                  backgroundColor: budget.actionAvailable ? 'rgba(245, 158, 11, 0.16)' : '#1e2430',
                  border: budget.actionAvailable ? '1px solid #f59e0b' : '1px solid #334155',
                  color: budget.actionAvailable ? '#fde047' : '#64748b',
                  fontSize: '11.5px',
                  fontWeight: 800,
                  cursor: budget.actionAvailable ? 'pointer' : 'not-allowed',
                }}
                title="Usa sua Ação Principal para tentar fugir e desengajar do combate (Teste CD 12)"
              >
                🏃‍♂️ Retirada Tática / Fugir da Batalha (Ação • Teste CD 12)
              </button>
            )}
          </div>

          {/* SELETOR CLARO DE ALVO DO ATAQUE (COM PV REAL E CA) */}
          {hostileParticipants.length > 0 ? (
            <div
              data-testid="combat-target-selector"
              style={{
                backgroundColor: 'rgba(15, 23, 42, 0.85)',
                border: '1px solid rgba(239, 68, 68, 0.4)',
                borderRadius: '8px',
                padding: '10px 12px',
                display: 'flex',
                flexDirection: 'column',
                gap: '8px',
              }}
            >
              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexWrap: 'wrap', gap: '6px' }}>
                <span style={{ fontSize: '12px', fontWeight: 800, color: '#fca5a5', display: 'inline-flex', alignItems: 'center', gap: '6px' }}>
                  <Crosshair size={14} color="#ef4444" /> 🎯 Selecione o Alvo do Ataque ({livingHostiles.length} vivo(s) / {hostileParticipants.length} total):
                </span>
                {effectiveTarget && (
                  <span style={{ fontSize: '11.5px', color: '#fef08a', fontWeight: 700, fontFamily: 'monospace' }}>
                    Alvo Atual: {effectiveTarget.name} ({effectiveTarget.current_hp ?? 12}/{effectiveTarget.max_hp ?? 12} PV • CA {effectiveTarget.armor_class ?? 13})
                  </span>
                )}
              </div>

              <div style={{ display: 'flex', flexWrap: 'wrap', gap: '8px' }}>
                {hostileParticipants.map((enemy) => {
                  const maxHp = Math.max(1, enemy.max_hp ?? 12);
                  const curHp = Math.max(0, enemy.current_hp ?? maxHp);
                  const ac = enemy.armor_class ?? 13;
                  const isDead = Boolean(enemy.is_dead || curHp <= 0);
                  const isSelected = !isDead && effectiveTarget?.id === enemy.id;

                  return (
                    <button
                      key={enemy.id}
                      type="button"
                      disabled={isDead}
                      onClick={() => onSelectTarget?.(enemy)}
                      data-testid={`target-option-${enemy.id}`}
                      style={{
                        display: 'inline-flex',
                        alignItems: 'center',
                        gap: '6px',
                        padding: '6px 12px',
                        borderRadius: '6px',
                        backgroundColor: isDead
                          ? 'rgba(15, 23, 42, 0.4)'
                          : isSelected
                          ? 'rgba(225, 29, 72, 0.25)'
                          : '#131924',
                        border: isDead
                          ? '1px dashed #334155'
                          : isSelected
                          ? '2px solid #f43f5e'
                          : '1px solid #2d3748',
                        color: isDead ? '#64748b' : isSelected ? '#fff' : '#cbd5e1',
                        fontSize: '11.5px',
                        fontWeight: isSelected ? 800 : 600,
                        cursor: isDead ? 'not-allowed' : 'pointer',
                        textDecoration: isDead ? 'line-through' : 'none',
                        boxShadow: isSelected ? '0 0 10px rgba(244, 63, 94, 0.35)' : 'none',
                        transition: 'all 0.15s ease',
                      }}
                    >
                      <span>{isDead ? '💀' : isSelected ? '🎯' : '⚔️'}</span>
                      <span>{enemy.name}</span>
                      <span
                        style={{
                          fontFamily: 'monospace',
                          fontSize: '11px',
                          color: isDead ? '#ef4444' : '#fca5a5',
                          fontWeight: 800,
                        }}
                      >
                        ({curHp}/{maxHp} PV • 🛡️ CA {ac})
                      </span>
                    </button>
                  );
                })}
              </div>
            </div>
          ) : hasHostileTargets ? (
            <div
              style={{
                backgroundColor: 'rgba(239, 68, 68, 0.12)',
                border: '1px solid rgba(239, 68, 68, 0.35)',
                borderRadius: '6px',
                padding: '8px 12px',
                fontSize: '12px',
                color: '#fca5a5',
                display: 'flex',
                alignItems: 'center',
                gap: '8px',
              }}
            >
              <Sword size={15} color="#ef4444" />
              <span>
                <strong>Inimigos Hostis no Combate:</strong>{' '}
                {hostileNames && hostileNames.length > 0
                  ? hostileNames.join(', ')
                  : 'Ameaças e Patrulhas da Região'}{' '}
                — Selecione uma ação marcial ou magia abaixo para atacar ou manobrar contra eles.
              </span>
            </div>
          ) : (
            <div
              style={{
                backgroundColor: 'rgba(16, 185, 129, 0.1)',
                border: '1px solid rgba(16, 185, 129, 0.35)',
                borderRadius: '6px',
                padding: '8px 12px',
                fontSize: '12px',
                color: '#6ee7b7',
                display: 'flex',
                alignItems: 'center',
                gap: '8px',
              }}
            >
              <CheckCircle2 size={15} color="#10b981" />
              Todos os inimigos hostis foram derrotados! Você pode encerrar o combate ou concluir a cena.
            </div>
          )}

          {/* Feedback de Progressão do Turno quando a Ação Principal foi gasta */}
          {!budget.actionAvailable && (
            <div
              data-testid="turn-progression-banner"
              style={{
                backgroundColor: 'rgba(234, 179, 8, 0.14)',
                border: '1px solid rgba(234, 179, 8, 0.45)',
                borderRadius: '8px',
                padding: '10px 14px',
                fontSize: '12px',
                color: '#fef08a',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                flexWrap: 'wrap',
                gap: '8px',
              }}
            >
              <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                <SkipForward size={15} color="#facc15" />
                <span>
                  <strong>Ação Principal Consumida!</strong> Use sua Ação Bônus/Movimento restante ou passe imediatamente para o próximo participante na Iniciativa:
                </span>
              </div>
              <button
                type="button"
                onClick={handleEndTurnClick}
                style={{
                  backgroundColor: '#eab308',
                  color: '#000',
                  border: 'none',
                  borderRadius: '6px',
                  padding: '6px 14px',
                  fontWeight: 800,
                  fontSize: '12px',
                  cursor: 'pointer',
                  whiteSpace: 'nowrap',
                  boxShadow: '0 0 12px rgba(234, 179, 8, 0.4)',
                }}
              >
                ⏭️ Encerrar Turno -&gt; Próximo na Iniciativa
              </button>
            </div>
          )}

          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fill, minmax(260px, 1fr))',
              gap: '10px',
            }}
          >
            {sheetActions.map((action) => {
              const canAfford =
                action.type === 'action'
                  ? budget.actionAvailable
                  : action.type === 'bonus_action'
                  ? budget.bonusActionAvailable
                  : action.type === 'reaction'
                  ? budget.reactionAvailable
                  : true;

              const isEnabled = action.isAvailable && canAfford;
              const isOffensive = action.category === 'attack' || action.category === 'spell';

              return (
                <div
                  key={action.id}
                  style={{
                    backgroundColor: '#101318',
                    border: isEnabled
                      ? '1px solid #28303e'
                      : '1px solid #1a1e26',
                    borderRadius: '8px',
                    padding: '12px',
                    display: 'flex',
                    flexDirection: 'column',
                    justifyContent: 'space-between',
                    opacity: isEnabled ? 1 : 0.6,
                    position: 'relative',
                  }}
                >
                  <div>
                    <div
                      style={{
                        display: 'flex',
                        justifyContent: 'space-between',
                        alignItems: 'center',
                        marginBottom: '4px',
                      }}
                    >
                      <span style={{ fontWeight: 'bold', fontSize: '13px', color: '#fff' }}>
                        {action.name}
                      </span>
                      <span
                        style={{
                          fontSize: '10px',
                          padding: '2px 6px',
                          borderRadius: '4px',
                          backgroundColor:
                            action.type === 'action'
                              ? 'rgba(16, 185, 129, 0.2)'
                              : action.type === 'bonus_action'
                              ? 'rgba(249, 115, 22, 0.2)'
                              : 'rgba(168, 85, 247, 0.2)',
                          color:
                            action.type === 'action'
                              ? 'var(--accent-emerald)'
                              : action.type === 'bonus_action'
                              ? '#fb923c'
                              : '#c084fc',
                          fontWeight: 700,
                        }}
                      >
                        {action.type === 'action'
                          ? 'Ação'
                          : action.type === 'bonus_action'
                          ? 'Ação Bônus'
                          : 'Reação'}
                      </span>
                    </div>

                    <p style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '8px', lineHeight: '1.4' }}>
                      {action.description}
                    </p>

                    <div style={{ display: 'flex', flexWrap: 'wrap', gap: '8px', fontSize: '11px', color: 'var(--text-faint)' }}>
                      {action.hitBonusOrDc && <span>Acerto: {action.hitBonusOrDc}</span>}
                      {action.damageFormula && <span>Dano: {action.damageFormula}</span>}
                      {action.rangeFeet && <span>Alcance: {action.rangeFeet}ft</span>}
                    </div>
                  </div>

                  <button
                    disabled={!isEnabled}
                    onClick={() => onExecuteSheetAction(action, isOffensive ? effectiveTarget : null)}
                    style={{
                      marginTop: '10px',
                      padding: '6px 12px',
                      borderRadius: '6px',
                      backgroundColor: isEnabled ? 'var(--accent-gold)' : '#1f242d',
                      color: isEnabled ? '#000' : 'var(--text-faint)',
                      fontWeight: 700,
                      fontSize: '12px',
                      cursor: isEnabled ? 'pointer' : 'not-allowed',
                      textAlign: 'center',
                      border: 'none',
                      transition: 'all 0.2s',
                    }}
                  >
                    {isEnabled
                      ? isOffensive && effectiveTarget
                        ? `🎯 Executar em ${effectiveTarget.name}`
                        : 'Executar no Turno'
                      : action.unavailabilityReason || 'Sem Orçamento no Turno'}
                  </button>
                </div>
              );
            })}

            {/* Card Mecânico Explícito de Retirada Tática / Fuga da Batalha */}
            <div
              style={{
                backgroundColor: '#121016',
                border: budget.actionAvailable
                  ? '1px solid rgba(245, 158, 11, 0.45)'
                  : '1px solid #1a1e26',
                borderRadius: '8px',
                padding: '12px',
                display: 'flex',
                flexDirection: 'column',
                justifyContent: 'space-between',
                opacity: budget.actionAvailable ? 1 : 0.6,
              }}
            >
              <div>
                <div
                  style={{
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'center',
                    marginBottom: '4px',
                  }}
                >
                  <span style={{ fontWeight: 'bold', fontSize: '13px', color: '#fde047' }}>
                    🏃‍♂️ Retirada Tática / Fugir da Batalha
                  </span>
                  <span
                    style={{
                      fontSize: '10px',
                      padding: '2px 6px',
                      borderRadius: '4px',
                      backgroundColor: 'rgba(245, 158, 11, 0.2)',
                      color: '#fbbf24',
                      fontWeight: 700,
                    }}
                  >
                    Ação • CD 12
                  </span>
                </div>
                <p style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '8px', lineHeight: '1.4' }}>
                  Manobra mecânica para desengajar das linhas hostis e bater em retirada coordenada (Teste de Destreza ou Atletismo CD 12).
                </p>
                <div style={{ fontSize: '11px', color: '#94a3b8' }}>
                  Mecânica oficial para encerrar um combate sem abater todos os inimigos.
                </div>
              </div>

              <button
                type="button"
                disabled={!budget.actionAvailable}
                onClick={handleFleeClick}
                data-testid="flee-action-card-btn"
                style={{
                  marginTop: '10px',
                  padding: '6px 12px',
                  borderRadius: '6px',
                  backgroundColor: budget.actionAvailable ? '#f59e0b' : '#1f242d',
                  color: budget.actionAvailable ? '#000' : 'var(--text-faint)',
                  fontWeight: 800,
                  fontSize: '12px',
                  cursor: budget.actionAvailable ? 'pointer' : 'not-allowed',
                  border: 'none',
                }}
              >
                {budget.actionAvailable
                  ? '🏃‍♂️ Executar Retirada Tática (CD 12)'
                  : 'Sem Ação Principal no Turno'}
              </button>
            </div>
          </div>
        </div>
      )}

      {/* ========================================================
          VIA 2: SUGESTÕES DO CENÁRIO (IA / MUNDO)
          (Aba Primária no Modo Exploração)
         ======================================================== */}
      {activeVia === 'via2' && (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <span style={{ fontSize: '12px', color: 'var(--text-muted)' }}>
              {combatMode
                ? 'Oportunidades táticas do terreno, cobertura e interação durante a batalha:'
                : 'Possibilidades situacionais extraídas do ambiente, pistas e NPCs da cena:'}
            </span>
            <span
              style={{
                fontSize: '11px',
                color: '#38bdf8',
                backgroundColor: 'rgba(56, 189, 248, 0.1)',
                padding: '2px 8px',
                borderRadius: '4px',
                border: '1px solid rgba(56, 189, 248, 0.25)',
              }}
            >
              Sugestões da IA baseadas no D&D 5e (#regras)
            </span>
          </div>

          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fill, minmax(280px, 1fr))',
              gap: '10px',
            }}
          >
            {scenarioActions.map((scen) => {
              const isConsumed = Boolean(scen.consumed);
              const canAfford =
                !isConsumed &&
                (!combatMode ||
                (scen.actionCost === 'action'
                  ? budget.actionAvailable
                  : scen.actionCost === 'bonus_action'
                  ? budget.bonusActionAvailable
                  : true));

              return (
                <div
                  key={scen.id}
                  style={{
                    backgroundColor: isConsumed ? '#080a0e' : '#101318',
                    border: isConsumed
                      ? '1px dashed rgba(239, 68, 68, 0.35)'
                      : canAfford
                      ? '1px solid var(--border-color)'
                      : '1px solid #1f2530',
                    borderRadius: '8px',
                    padding: '12px',
                    display: 'flex',
                    flexDirection: 'column',
                    justifyContent: 'space-between',
                    opacity: isConsumed ? 0.45 : canAfford ? 1 : 0.6,
                  }}
                >
                  <div>
                    <div
                      style={{
                        display: 'flex',
                        justifyContent: 'space-between',
                        alignItems: 'center',
                        marginBottom: '4px',
                      }}
                    >
                      <span
                        style={{
                          fontWeight: 'bold',
                          fontSize: '14px',
                          color: isConsumed
                            ? '#64748b'
                            : canAfford
                            ? 'var(--accent-gold)'
                            : 'var(--text-muted)',
                          textDecoration: isConsumed ? 'line-through' : 'none',
                        }}
                      >
                        {scen.title}
                      </span>
                      {isConsumed ? (
                        <span
                          style={{
                            fontSize: '10px',
                            padding: '2px 6px',
                            borderRadius: '4px',
                            backgroundColor: 'rgba(239, 68, 68, 0.15)',
                            color: '#f87171',
                            fontWeight: 700,
                          }}
                        >
                          Esgotada
                        </span>
                      ) : combatMode && (
                        <span
                          style={{
                            fontSize: '10px',
                            padding: '2px 6px',
                            borderRadius: '4px',
                            backgroundColor: '#1e2430',
                            color: 'var(--text-muted)',
                          }}
                        >
                          {scen.actionCost === 'action' ? 'Custa Ação' : 'Ação Bônus'}
                        </span>
                      )}
                    </div>

                    <p style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '8px', lineHeight: '1.4' }}>
                      {scen.description}
                    </p>

                    <div style={{ fontSize: '11px', color: isConsumed ? '#64748b' : 'var(--accent-emerald)', marginBottom: '6px' }}>
                      Benefício: {scen.tacticalBenefit}
                    </div>

                    {scen.skillCheckSuggested && (
                      <div
                        style={{
                          fontSize: '11px',
                          color: isConsumed ? '#64748b' : '#38bdf8',
                          display: 'inline-flex',
                          alignItems: 'center',
                          gap: '4px',
                          backgroundColor: isConsumed ? 'rgba(255,255,255,0.03)' : 'rgba(56, 189, 248, 0.1)',
                          padding: '2px 6px',
                          borderRadius: '4px',
                        }}
                      >
                        <Target size={11} /> Teste: {scen.skillCheckSuggested.skill}{' '}
                        {scen.skillCheckSuggested.dcEstimate ? `(CD ~${scen.skillCheckSuggested.dcEstimate})` : ''}
                      </div>
                    )}
                  </div>

                  <button
                    disabled={!canAfford || isConsumed}
                    onClick={() => onExecuteScenarioAction(scen)}
                    style={{
                      marginTop: '10px',
                      padding: '8px 12px',
                      borderRadius: '6px',
                      backgroundColor: isConsumed
                        ? '#141822'
                        : canAfford
                        ? 'var(--accent-sky)'
                        : '#1c212a',
                      color: isConsumed ? '#64748b' : canAfford ? '#fff' : 'var(--text-faint)',
                      fontWeight: 700,
                      fontSize: '12px',
                      cursor: canAfford && !isConsumed ? 'pointer' : 'not-allowed',
                      border: isConsumed
                        ? '1px dashed #334155'
                        : canAfford
                        ? 'none'
                        : '1px solid #2d3748',
                      transition: 'all 0.2s',
                    }}
                  >
                    {isConsumed
                      ? '❌ Ação Já Executada nesta Cena'
                      : canAfford
                      ? 'Interagir com o Cenário'
                      : 'Ação Indisponível'}
                  </button>
                </div>
              );
            })}
          </div>

          {/* Barra de Avanço da História e Confronto com o Chefe */}
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'space-between',
              backgroundColor: 'rgba(234, 179, 8, 0.08)',
              border: '1px solid rgba(234, 179, 8, 0.3)',
              borderRadius: '8px',
              padding: '12px 16px',
              marginTop: '12px',
              flexWrap: 'wrap',
              gap: '12px',
            }}
          >
            <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
              <Sparkles size={18} color="#facc15" />
              <div>
                <div style={{ fontSize: '13px', fontWeight: 800, color: '#fef08a' }}>
                  Avanço da Crônica &amp; Confronto da Região
                </div>
                <div style={{ fontSize: '11.5px', color: '#cbd5e1' }}>
                  {isBattleLocked
                    ? 'Há inimigos hostis vivos em combate! Derrote-os ou execute Retirada Tática (CD 12) antes de avançar.'
                    : 'Após vasculhar pistas e interagir com o cenário, avance para a próxima cena ou inicie o combate tático.'}
                </div>
              </div>
            </div>

            <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap' }}>
              {!combatMode && (
                <button
                  type="button"
                  onClick={() => setCombatMode(true)}
                  style={{
                    display: 'inline-flex',
                    alignItems: 'center',
                    gap: '6px',
                    padding: '8px 14px',
                    borderRadius: '6px',
                    backgroundColor: 'rgba(239, 68, 68, 0.2)',
                    border: '1px solid #ef4444',
                    color: '#fca5a5',
                    fontSize: '12px',
                    fontWeight: 700,
                    cursor: 'pointer',
                    transition: 'all 0.15s',
                  }}
                  title="Rolar iniciativa e entrar no modo de combate tático D&D 5e contra os inimigos da cena"
                >
                  <Sword size={14} /> ⚔️ Iniciar Combate Tático
                </button>
              )}

              {isBattleLocked ? (
                <button
                  type="button"
                  onClick={handleFleeClick}
                  style={{
                    display: 'inline-flex',
                    alignItems: 'center',
                    gap: '6px',
                    padding: '8px 14px',
                    borderRadius: '6px',
                    backgroundColor: 'rgba(245, 158, 11, 0.2)',
                    border: '1px solid #f59e0b',
                    color: '#fde047',
                    fontSize: '12px',
                    fontWeight: 800,
                    cursor: 'pointer',
                  }}
                >
                  🏃‍♂️ Tentar Retirada Tática / Fuga (CD 12)
                </button>
              ) : (
                onResolveScene && (
                  <button
                    type="button"
                    onClick={onResolveScene}
                    data-testid="advance-scene-btn"
                    style={{
                      display: 'inline-flex',
                      alignItems: 'center',
                      gap: '6px',
                      padding: '8px 16px',
                      borderRadius: '6px',
                      backgroundColor: 'rgba(16, 185, 129, 0.2)',
                      border: '1px solid #10b981',
                      color: '#6ee7b7',
                      fontSize: '12px',
                      fontWeight: 800,
                      cursor: 'pointer',
                      transition: 'all 0.15s',
                    }}
                    title="Consolidar esta cena na Crônica e avançar para o próximo ato"
                  >
                    <CheckCircle2 size={14} /> 📜 Concluir Cena &amp; Avançar História
                  </button>
                )
              )}
            </div>
          </div>
        </div>
      )}

      {/* ========================================================
          VIA 3: INTERAÇÃO LIVRE COM IA (ARBITRAGEM DE REGRAS)
          (Aba Secundária no Modo Exploração)
         ======================================================== */}
      {activeVia === 'via3' && (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', lineHeight: '1.4' }}>
            Descreva qualquer ação criativa, diálogo com NPCs, investigação minuciosa ou manobra em linguagem natural. A IA avaliará sua intenção e acionará o motor determinístico em Rust quando houver regras ou testes necessários (`#regras`):
          </div>

          {/* Atalhos Rápidos de Ação Criativa (Prompt Helpers) */}
          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px' }}>
            <button
              type="button"
              onClick={() => handleQuickPrompt('Examinar atentamente as paredes, o chão e os cantos escuros da sala em busca de armadilhas, passagens secretas ou pistas...')}
              style={{
                fontSize: '11px',
                padding: '3px 8px',
                borderRadius: '4px',
                backgroundColor: '#161d28',
                border: '1px solid #233145',
                color: '#93c5fd',
                cursor: 'pointer',
              }}
            >
              🔍 Investigar Ambiente
            </button>
            <button
              type="button"
              onClick={() => handleQuickPrompt('Aproximo-me com calma e pergunto amigavelmente sobre o que aconteceu neste lugar recentemente...')}
              style={{
                fontSize: '11px',
                padding: '3px 8px',
                borderRadius: '4px',
                backgroundColor: '#161d28',
                border: '1px solid #233145',
                color: '#93c5fd',
                cursor: 'pointer',
              }}
            >
              💬 Dialogar com Calma
            </button>
            <button
              type="button"
              onClick={() => handleQuickPrompt('Tento me mover pelas sombras nas bordas da sala sem fazer ruído algum...')}
              style={{
                fontSize: '11px',
                padding: '3px 8px',
                borderRadius: '4px',
                backgroundColor: '#161d28',
                border: '1px solid #233145',
                color: '#93c5fd',
                cursor: 'pointer',
              }}
            >
              👥 Esgueirar-se em Furtividade
            </button>
          </div>

          <form onSubmit={handleFreeformSubmit} style={{ display: 'flex', gap: '8px' }}>
            <input
              type="text"
              value={freeformInput}
              onChange={(e) => setFreeformInput(e.target.value)}
              placeholder="Ex: 'Acendo uma tocha e tento inspecionar o símbolo entalhado na porta de ferro...'"
              style={{
                flex: 1,
                fontSize: '13px',
                padding: '10px 14px',
                backgroundColor: '#0c0f14',
                border: '1px solid var(--border-color)',
                borderRadius: '8px',
                color: '#fff',
              }}
            />
            <button
              type="submit"
              disabled={isEvaluating || !freeformInput.trim()}
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '8px',
                padding: '10px 18px',
                borderRadius: '8px',
                backgroundColor: isEvaluating ? '#333' : 'var(--accent-gold)',
                color: isEvaluating ? '#888' : '#000',
                fontWeight: 700,
                fontSize: '13px',
                cursor: isEvaluating ? 'not-allowed' : 'pointer',
                border: 'none',
                transition: 'all 0.2s',
              }}
            >
              <Send size={15} /> {isEvaluating ? 'Avaliando IA...' : 'Enviar Ação'}
            </button>
          </form>

          {/* AI Arbitration Outcome Feedback */}
          {recentSubmission && (
            <div
              style={{
                backgroundColor: '#101318',
                border: recentSubmission.requiresRulesMechanic
                  ? '1px solid var(--accent-gold)'
                  : '1px solid var(--accent-emerald)',
                borderRadius: '8px',
                padding: '12px',
                marginTop: '4px',
              }}
            >
              <div
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '8px',
                  marginBottom: '6px',
                  fontSize: '12px',
                  fontWeight: 700,
                  color: recentSubmission.requiresRulesMechanic
                    ? 'var(--accent-gold)'
                    : 'var(--accent-emerald)',
                }}
              >
                {recentSubmission.requiresRulesMechanic ? (
                  <>
                    <AlertTriangle size={15} /> Parecer da IA: Mecânica do Jogo Acionada (#regras)
                  </>
                ) : (
                  <>
                    <CheckCircle2 size={15} /> Parecer da IA: Resolução Puramente Narrativa
                  </>
                )}
              </div>

              <div style={{ fontSize: '13px', color: '#fff', marginBottom: '4px' }}>
                &ldquo;{recentSubmission.playerText}&rdquo;
              </div>

              {recentSubmission.resolvedCheck && (
                <div style={{ fontSize: '12px', color: 'var(--accent-sky)' }}>
                  Rolagem Determinística: {recentSubmission.resolvedCheck.abilityOrSkill} (
                  {recentSubmission.resolvedCheck.rollFormula}) ={' '}
                  <strong>{recentSubmission.resolvedCheck.result}</strong> vs CD{' '}
                  {recentSubmission.resolvedCheck.targetDc} &rarr;{' '}
                  <span
                    style={{
                      color: recentSubmission.resolvedCheck.success
                        ? 'var(--accent-emerald)'
                        : 'var(--accent-crimson)',
                      fontWeight: 'bold',
                    }}
                  >
                    {recentSubmission.resolvedCheck.success ? 'SUCESSO!' : 'FALHA!'}
                  </span>
                </div>
              )}

              {recentSubmission.narrativeFeedback && (
                <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginTop: '4px' }}>
                  {recentSubmission.narrativeFeedback}
                </div>
              )}
            </div>
          )}
        </div>
      )}

      {/* Turn Action Log */}
      {actionLog.length > 0 && (
        <div
          style={{
            backgroundColor: '#0c0e12',
            borderRadius: '6px',
            padding: '8px 12px',
            border: '1px solid #1e2430',
          }}
        >
          <div style={{ fontSize: '11px', color: 'var(--text-faint)', fontWeight: 600, marginBottom: '4px' }}>
            Ações Executadas no Turno Atual:
          </div>
          <div style={{ display: 'flex', flexDirection: 'column', gap: '2px' }}>
            {actionLog.map((log, idx) => (
              <div key={idx} style={{ fontSize: '12px', color: 'var(--accent-emerald)' }}>
                &bull; {log}
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};
