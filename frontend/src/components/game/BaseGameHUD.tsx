import { useState } from 'react';
import type { FC, ReactNode } from 'react';
import type { CharacterSheet } from '../../types/character';
import type {
  ActionEconomyBudget,
  NarrativeBeat,
  SheetActionOption,
  ContextualScenarioOption,
  FreeformTextActionSubmission,
} from '../../types/game';
import { VitalsAndAttributesPanel } from './VitalsAndAttributesPanel';
import { WorldViewPanel } from './WorldViewPanel';
import { TurnPossibilitiesConsole } from './TurnPossibilitiesConsole';
import { DND_CLASSES } from '../character-creation/constants';
import { calculateAbilityModifier } from '../../types/character';
import { RefreshCw } from 'lucide-react';
import { submitTurnActionToBackend } from '../../services/api';

interface BaseGameHUDProps {
  character: CharacterSheet;
  onResetCharacter: () => void;
  classSpecificPanel?: ReactNode;
}

export const BaseGameHUD: FC<BaseGameHUDProps> = ({
  character: initialCharacter,
  onResetCharacter,
  classSpecificPanel,
}) => {
  const [character, setCharacter] = useState<CharacterSheet>(initialCharacter);

  // Turn budget state
  const [budget, setBudget] = useState<ActionEconomyBudget>({
    actionAvailable: true,
    bonusActionAvailable: true,
    reactionAvailable: true,
    movementTotalFeet: character.vitals.speedFeet,
    movementRemainingFeet: character.vitals.speedFeet,
  });

  // Action log for the turn
  const [actionLog, setActionLog] = useState<string[]>([]);

  // Recent freeform submission
  const [recentSubmission, setRecentSubmission] =
    useState<FreeformTextActionSubmission | null>(null);

  // Active Beat data
  const [activeBeat] = useState<NarrativeBeat>({
    id: 'beat-1',
    beatIndex: 1,
    title: 'A Cripta dos Reis Esquecidos',
    location: 'Salão do Sarcófago Rúnico — Masmorra Profunda',
    proseContent:
      'O ar úmido cheira a musgo centenário e terra revolvida. À sua frente, entre as colunas partidas de pedra calcária, ergue-se um bruto orc com uma maça cravada de cravos oxidados. No centro do salão, o sarcófago rúnico emite um zumbido violeta rítmico, como um coração arcano pulsando no escuro. As sombras nas paredes dançam ao sabor da sua tocha trêmula.',
    activeNpcs: ['Guerreiro Orc Brutal', 'Espírito Ancestral Silencioso'],
    tacticalGridEngaged: true,
  });

  const strMod = calculateAbilityModifier(character.abilities.strength);
  const dexMod = calculateAbilityModifier(character.abilities.dexterity);
  const prof = character.proficiencyBonus;

  // Build Sheet Actions dynamically based on character capabilities
  const sheetActions: SheetActionOption[] = [
    {
      id: 'attack-weapon',
      name: 'Ataque Marcial com Arma',
      type: 'action',
      category: 'attack',
      description: 'Ataca o inimigo mais próximo no alcance corpo a corpo ou à distância.',
      rangeFeet: 5,
      hitBonusOrDc: `+${Math.max(strMod, dexMod) + prof} para acertar`,
      damageFormula: `1d8 + ${Math.max(strMod, dexMod)} cortante/perfurante`,
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
  ];

  // Add spell actions if spellcaster
  if (
    character.spellSlots.pactMagic ||
    (character.spellSlots.levels[1] && character.spellSlots.levels[1].total > 0)
  ) {
    const hasSlots = character.spellSlots.pactMagic
      ? character.spellSlots.pactMagic.used < character.spellSlots.pactMagic.total
      : (character.spellSlots.levels[1]?.used || 0) < (character.spellSlots.levels[1]?.total || 0);

    sheetActions.unshift({
      id: 'cast-spell-level1',
      name: 'Conjurar Magia de 1º Círculo',
      type: 'action',
      category: 'spell',
      description: 'Canaliza o poder arcano ou divino para lançar uma magia preparada.',
      rangeFeet: 60,
      hitBonusOrDc: 'CD 13 Salvaguarda',
      damageFormula: '3d4 + 3 de Força ou Efeito',
      spellLevelRequired: 1,
      isAvailable: hasSlots,
      unavailabilityReason: hasSlots ? undefined : 'Sem Espaços de Magia disponíveis!',
    });
  }

  // Contextual Scenario Options
  const scenarioActions: ContextualScenarioOption[] = [
    {
      id: 'scen-sarcophagus',
      title: 'Examinar o Sarcófago Rúnico',
      description: 'Aproxima-se para decifrar as runas violetas esculpidas na tampa de pedra.',
      actionCost: 'action',
      targetObjectName: 'Sarcófago Rúnico',
      skillCheckSuggested: { skill: 'Arcanismo ou Religião', dcEstimate: 13 },
      tacticalBenefit: 'Pode revelar fraqueza do inimigo ou desarmar armadilha de pulso.',
    },
    {
      id: 'scen-pillar',
      title: 'Empurrar Pilar Rachado sobre o Orc',
      description: 'Usa a alavancagem de uma das colunas abaladas para derrubá-la no inimigo.',
      actionCost: 'action',
      targetObjectName: 'Pilar de Pedra Calcária',
      skillCheckSuggested: { skill: 'Atletismo (FOR)', dcEstimate: 14 },
      tacticalBenefit: 'Causa 2d10 de dano contundente e derruba o orc prostrado se falhar.',
    },
    {
      id: 'scen-intimidate',
      title: 'Rugir Ordem de Rendição',
      description: 'Aproveita a postura combativa e o eco da câmara para abalar a moral do orc.',
      actionCost: 'bonus_action',
      skillCheckSuggested: { skill: 'Intimidação (CAR)', dcEstimate: 12 },
      tacticalBenefit: 'Inimigo recebe desvantagem no próximo ataque por medo.',
    },
  ];

  // HP manipulation
  const handleUpdateHp = (delta: number) => {
    setCharacter((prev) => {
      const nextHp = Math.min(prev.vitals.hpMax, Math.max(0, prev.vitals.hpCurrent + delta));
      return {
        ...prev,
        vitals: { ...prev.vitals, hpCurrent: nextHp },
      };
    });
  };

  // Toggle Spell Slot
  const handleToggleSpellSlot = (level: number, isPact?: boolean) => {
    setCharacter((prev) => {
      if (isPact && prev.spellSlots.pactMagic) {
        const cur = prev.spellSlots.pactMagic.used;
        const tot = prev.spellSlots.pactMagic.total;
        const nextUsed = cur >= tot ? 0 : cur + 1;
        return {
          ...prev,
          spellSlots: {
            ...prev.spellSlots,
            pactMagic: { ...prev.spellSlots.pactMagic, used: nextUsed },
          },
        };
      } else {
        const slot = prev.spellSlots.levels[level];
        if (!slot) return prev;
        const nextUsed = slot.used >= slot.total ? 0 : slot.used + 1;
        return {
          ...prev,
          spellSlots: {
            ...prev.spellSlots,
            levels: {
              ...prev.spellSlots.levels,
              [level]: { ...slot, used: nextUsed },
            },
          },
        };
      }
    });
  };

  // Execute Sheet Action
  const handleExecuteSheetAction = (action: SheetActionOption) => {
    if (action.type === 'action') {
      setBudget((b) => ({ ...b, actionAvailable: false }));
    } else if (action.type === 'bonus_action') {
      setBudget((b) => ({ ...b, bonusActionAvailable: false }));
    } else if (action.type === 'reaction') {
      setBudget((b) => ({ ...b, reactionAvailable: false }));
    }

    // Deduct slot if spell
    if (action.category === 'spell' && action.spellLevelRequired) {
      handleToggleSpellSlot(
        action.spellLevelRequired,
        Boolean(character.spellSlots.pactMagic)
      );
    }

    setActionLog((prev) => [
      ...prev,
      `[${action.type.toUpperCase()}] ${action.name} executado com sucesso. (${action.damageFormula || action.description})`,
    ]);
  };

  // Execute Scenario Action
  const handleExecuteScenarioAction = (scen: ContextualScenarioOption) => {
    if (scen.actionCost === 'action') {
      setBudget((b) => ({ ...b, actionAvailable: false }));
    } else if (scen.actionCost === 'bonus_action') {
      setBudget((b) => ({ ...b, bonusActionAvailable: false }));
    }

    const d20 = Math.floor(Math.random() * 20) + 1;
    const total = d20 + prof + strMod;
    const isSuccess = scen.skillCheckSuggested?.dcEstimate
      ? total >= scen.skillCheckSuggested.dcEstimate
      : d20 > 10;

    setActionLog((prev) => [
      ...prev,
      `[CENÁRIO] ${scen.title} -> Teste D20 (${d20}) + Bônus = ${total} vs CD ${
        scen.skillCheckSuggested?.dcEstimate || 12
      }: ${isSuccess ? 'SUCESSO! ' + scen.tacticalBenefit : 'FALHA! Ação consumida.'}`,
    ]);
  };

  // Freeform Text Action (AI Evaluation via Backend / Resolver)
  const handleSubmitFreeformAction = async (text: string) => {
    const sub = await submitTurnActionToBackend('via3', character, text);

    if (sub.requiresRulesMechanic) {
      setBudget((b) => ({ ...b, actionAvailable: false }));
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
    <div style={{ maxWidth: '1280px', margin: '0 auto', padding: '16px', display: 'flex', flexDirection: 'column', gap: '16px' }}>
      {/* Top Bar with Character Header */}
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          backgroundColor: 'var(--bg-card)',
          border: `1px solid ${classMeta?.color || 'var(--border-color)'}`,
          padding: '12px 20px',
          borderRadius: '12px',
        }}
      >
        <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
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
        </div>

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
          }}
        >
          <RefreshCw size={13} /> Trocar Personagem
        </button>
      </div>

      {/* Class Specific Resources Bar (Injected slot) */}
      {classSpecificPanel}

      {/* 2-Column Main Layout: Vitals/World on Top, Turn Console below */}
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(380px, 1fr))',
          gap: '16px',
        }}
      >
        <VitalsAndAttributesPanel
          character={character}
          onUpdateHp={handleUpdateHp}
          onToggleSpellSlot={handleToggleSpellSlot}
        />

        <WorldViewPanel beat={activeBeat} />
      </div>

      {/* Turn Possibilities Console (3 Vias) */}
      <TurnPossibilitiesConsole
        budget={budget}
        sheetActions={sheetActions}
        scenarioActions={scenarioActions}
        onExecuteSheetAction={handleExecuteSheetAction}
        onExecuteScenarioAction={handleExecuteScenarioAction}
        onSubmitFreeformAction={handleSubmitFreeformAction}
        onResetTurnBudget={handleResetTurnBudget}
        recentSubmission={recentSubmission}
        actionLog={actionLog}
      />
    </div>
  );
};
