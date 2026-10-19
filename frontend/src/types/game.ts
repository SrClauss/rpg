export type ActionBudgetSlot = 'action' | 'bonus_action' | 'reaction' | 'movement';

export interface ActionEconomyBudget {
  actionAvailable: boolean;
  bonusActionAvailable: boolean;
  reactionAvailable: boolean;
  movementTotalFeet: number;
  movementRemainingFeet: number;
}

export interface SheetActionOption {
  id: string;
  name: string;
  type: ActionBudgetSlot;
  description: string;
  category: 'attack' | 'spell' | 'class_feature' | 'basic';
  rangeFeet?: number;
  spellLevelRequired?: number;
  resourceCost?: {
    resourceName: string;
    amount: number;
  };
  hitBonusOrDc?: string;
  damageFormula?: string;
  isAvailable: boolean;
  unavailabilityReason?: string;
}

export interface ContextualScenarioOption {
  id: string;
  title: string;
  description: string;
  actionCost: ActionBudgetSlot | 'free';
  targetObjectName?: string;
  skillCheckSuggested?: {
    skill: string;
    dcEstimate?: number;
  };
  tacticalBenefit: string;
}

export interface FreeformTextActionSubmission {
  playerText: string;
  submittedAt: string;
  status: 'pending' | 'evaluating_llm' | 'resolved';
  requiresRulesMechanic?: boolean;
  detectedIntent?: string;
  resolvedCheck?: {
    abilityOrSkill: string;
    rollFormula: string;
    targetDc?: number;
    result?: number;
    success?: boolean;
  };
  narrativeFeedback?: string;
}

export interface NarrativeBeat {
  id: string;
  beatIndex: number;
  title: string;
  proseContent: string;
  location: string;
  activeNpcs: string[];
  tacticalGridEngaged: boolean;
}

export interface GameState {
  currentTurnPlayerId: string;
  isPlayerTurn: boolean;
  roundNumber: number;
  actionBudget: ActionEconomyBudget;
  activeBeat: NarrativeBeat;
  sheetActions: SheetActionOption[];
  scenarioActions: ContextualScenarioOption[];
  freeformHistory: FreeformTextActionSubmission[];
}
