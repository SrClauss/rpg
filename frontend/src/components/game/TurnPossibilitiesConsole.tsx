import { useState } from 'react';
import type { FC } from 'react';
import type {
  ActionEconomyBudget,
  SheetActionOption,
  ContextualScenarioOption,
  FreeformTextActionSubmission,
} from '../../types/game';
import {
  Sword,
  Bot,
  Send,
  CheckCircle2,
  AlertTriangle,
  RotateCcw,
  Compass,
} from 'lucide-react';

interface TurnPossibilitiesConsoleProps {
  budget: ActionEconomyBudget;
  sheetActions: SheetActionOption[];
  scenarioActions: ContextualScenarioOption[];
  onExecuteSheetAction: (action: SheetActionOption) => void;
  onExecuteScenarioAction: (scenario: ContextualScenarioOption) => void;
  onSubmitFreeformAction: (text: string) => void;
  onResetTurnBudget: () => void;
  recentSubmission?: FreeformTextActionSubmission | null;
  actionLog: string[];
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
}) => {
  const [activeVia, setActiveVia] = useState<'via1' | 'via2' | 'via3'>('via1');
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

  return (
    <div
      style={{
        backgroundColor: 'var(--bg-card)',
        border: '1px solid var(--border-color)',
        borderRadius: '12px',
        padding: '16px',
        display: 'flex',
        flexDirection: 'column',
        gap: '14px',
      }}
    >
      {/* Turn Budget Bar (Action, Bonus Action, Reaction, Movement) */}
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          flexWrap: 'wrap',
          gap: '10px',
          backgroundColor: '#101318',
          border: '1px solid var(--border-color)',
          borderRadius: '8px',
          padding: '10px 14px',
        }}
      >
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <span className="rpg-title" style={{ fontSize: '13px', color: 'var(--accent-gold)' }}>
            Orçamento do Turno:
          </span>

          <span
            style={{
              padding: '3px 8px',
              borderRadius: '4px',
              fontSize: '11px',
              fontWeight: 700,
              backgroundColor: budget.actionAvailable ? 'rgba(16, 185, 129, 0.2)' : '#261b1e',
              border: budget.actionAvailable ? '1px solid var(--accent-emerald)' : '1px solid #4a2128',
              color: budget.actionAvailable ? 'var(--accent-emerald)' : 'var(--accent-crimson)',
            }}
          >
            Ação: {budget.actionAvailable ? 'DISPONÍVEL' : 'GASTA'}
          </span>

          <span
            style={{
              padding: '3px 8px',
              borderRadius: '4px',
              fontSize: '11px',
              fontWeight: 700,
              backgroundColor: budget.bonusActionAvailable ? 'rgba(2, 132, 199, 0.2)' : '#261b1e',
              border: budget.bonusActionAvailable ? '1px solid var(--accent-sky)' : '1px solid #4a2128',
              color: budget.bonusActionAvailable ? 'var(--accent-sky)' : 'var(--accent-crimson)',
            }}
          >
            Ação Bônus: {budget.bonusActionAvailable ? 'DISPONÍVEL' : 'GASTA'}
          </span>

          <span
            style={{
              padding: '3px 8px',
              borderRadius: '4px',
              fontSize: '11px',
              fontWeight: 700,
              backgroundColor: budget.reactionAvailable ? 'rgba(234, 179, 8, 0.2)' : '#261b1e',
              border: budget.reactionAvailable ? '1px solid var(--accent-gold)' : '1px solid #4a2128',
              color: budget.reactionAvailable ? 'var(--accent-gold)' : 'var(--accent-crimson)',
            }}
          >
            Reação: {budget.reactionAvailable ? 'DISPONÍVEL' : 'GASTA'}
          </span>

          <span
            style={{
              padding: '3px 8px',
              borderRadius: '4px',
              fontSize: '11px',
              fontWeight: 700,
              backgroundColor: '#1b2230',
              border: '1px solid #2e3a50',
              color: '#93c5fd',
            }}
          >
            Movimento: {budget.movementRemainingFeet}/{budget.movementTotalFeet} pés
          </span>
        </div>

        <button
          onClick={onResetTurnBudget}
          style={{
            display: 'inline-flex',
            alignItems: 'center',
            gap: '6px',
            padding: '6px 12px',
            borderRadius: '6px',
            backgroundColor: '#1c222d',
            border: '1px solid var(--border-color)',
            fontSize: '11px',
            color: 'var(--text-muted)',
            fontWeight: 600,
          }}
          title="Reiniciar Orçamento de Turno"
        >
          <RotateCcw size={12} /> Novo Turno
        </button>
      </div>

      {/* Tabs for the 3 VIAS */}
      <div style={{ display: 'flex', gap: '8px' }}>
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
          <Sword size={16} /> Via 1: Mecânicas da Ficha
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
          <Compass size={16} /> Via 2: Sugestões do Cenário
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
          <Bot size={16} /> Via 3: Interação Livre (IA)
        </button>
      </div>

      {/* VIA 1: SHEET MECHANICS */}
      {activeVia === 'via1' && (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)' }}>
            Ações habilitadas pelo estado atual da sua ficha, armas equipadas e espaços de magia restantes:
          </div>

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
                      <span style={{ fontWeight: 'bold', fontSize: '14px', color: '#fff' }}>
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
                              ? 'rgba(2, 132, 199, 0.2)'
                              : 'rgba(234, 179, 8, 0.2)',
                          color:
                            action.type === 'action'
                              ? 'var(--accent-emerald)'
                              : action.type === 'bonus_action'
                              ? 'var(--accent-sky)'
                              : 'var(--accent-gold)',
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

                    <p style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '8px' }}>
                      {action.description}
                    </p>

                    <div style={{ display: 'flex', gap: '8px', fontSize: '11px', color: 'var(--text-faint)' }}>
                      {action.hitBonusOrDc && <span>Acerto: {action.hitBonusOrDc}</span>}
                      {action.damageFormula && <span>Dano: {action.damageFormula}</span>}
                      {action.rangeFeet && <span>Alcance: {action.rangeFeet}ft</span>}
                    </div>
                  </div>

                  <button
                    disabled={!isEnabled}
                    onClick={() => onExecuteSheetAction(action)}
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
                    }}
                  >
                    {isEnabled ? 'Executar no Turno' : action.unavailabilityReason || 'Sem Orçamento no Turno'}
                  </button>
                </div>
              );
            })}
          </div>
        </div>
      )}

      {/* VIA 2: CONTEXTUAL SCENARIO SUGGESTIONS */}
      {activeVia === 'via2' && (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)' }}>
            Possibilidades contextuais extraídas do ambiente do Beat atual:
          </div>

          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fill, minmax(280px, 1fr))',
              gap: '10px',
            }}
          >
            {scenarioActions.map((scen) => (
              <div
                key={scen.id}
                style={{
                  backgroundColor: '#101318',
                  border: '1px solid var(--border-color)',
                  borderRadius: '8px',
                  padding: '12px',
                  display: 'flex',
                  flexDirection: 'column',
                  justifyContent: 'space-between',
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
                    <span style={{ fontWeight: 'bold', fontSize: '14px', color: 'var(--accent-gold)' }}>
                      {scen.title}
                    </span>
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
                  </div>

                  <p style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '8px' }}>
                    {scen.description}
                  </p>

                  <div style={{ fontSize: '11px', color: 'var(--accent-emerald)', marginBottom: '6px' }}>
                    Benefício: {scen.tacticalBenefit}
                  </div>

                  {scen.skillCheckSuggested && (
                    <div style={{ fontSize: '11px', color: 'var(--accent-sky)' }}>
                      Teste Recomendado: {scen.skillCheckSuggested.skill}{' '}
                      {scen.skillCheckSuggested.dcEstimate ? `(CD ~${scen.skillCheckSuggested.dcEstimate})` : ''}
                    </div>
                  )}
                </div>

                <button
                  onClick={() => onExecuteScenarioAction(scen)}
                  style={{
                    marginTop: '10px',
                    padding: '6px 12px',
                    borderRadius: '6px',
                    backgroundColor: 'var(--accent-sky)',
                    color: '#fff',
                    fontWeight: 700,
                    fontSize: '12px',
                    cursor: 'pointer',
                  }}
                >
                  Interagir com o Cenário
                </button>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* VIA 3: FREEFORM TEXT ACTION (AI ARBITRATION) */}
      {activeVia === 'via3' && (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)' }}>
            Descreva qualquer ação criativa, fala ou manobra em linguagem natural. A IA avaliará se sua intenção é puramente narrativa ou se ativará as mecânicas determinísticas em Rust (`#regras`):
          </div>

          <form onSubmit={handleFreeformSubmit} style={{ display: 'flex', gap: '8px' }}>
            <input
              type="text"
              value={freeformInput}
              onChange={(e) => setFreeformInput(e.target.value)}
              placeholder="Ex: 'Chuto o braseiro aceso em direção aos pés do Orc para cegá-lo com as cinzas...'"
              style={{ flex: 1, fontSize: '14px', padding: '10px 14px' }}
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
