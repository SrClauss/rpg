export type CharacterClassType =
  | 'Barbarian'
  | 'Bard'
  | 'Cleric'
  | 'Druid'
  | 'Fighter'
  | 'Monk'
  | 'Paladin'
  | 'Ranger'
  | 'Rogue'
  | 'Sorcerer'
  | 'Warlock'
  | 'Wizard';

export type AbilityName =
  | 'strength'
  | 'dexterity'
  | 'constitution'
  | 'intelligence'
  | 'wisdom'
  | 'charisma';

export interface AbilityScores {
  strength: number;
  dexterity: number;
  constitution: number;
  intelligence: number;
  wisdom: number;
  charisma: number;
}

export interface SpellSlotLevel {
  level: number;
  total: number;
  used: number;
}

export interface SpellSlots {
  levels: Record<number, SpellSlotLevel>;
  pactMagic?: {
    slotLevel: number;
    total: number;
    used: number;
  };
}

export interface VitalStats {
  hpCurrent: number;
  hpMax: number;
  hpTemp: number;
  armorClass: number;
  initiativeModifier: number;
  speedFeet: number;
  hitDiceTotal: number;
  hitDiceCurrent: number;
  hitDiceType: string;
  deathSaves: {
    successes: number;
    failures: number;
  };
  conditions: string[];
}

export interface ClassSpecificResources {
  barbarian?: {
    rageUsesTotal: number;
    rageUsesCurrent: number;
    isRaging: boolean;
    rageDamageBonus: number;
    unarmoredDefenseFormula: string;
  };
  bard?: {
    inspirationDiceTotal: number;
    inspirationDiceCurrent: number;
    inspirationDieType: string;
    songOfRestDie: string;
  };
  cleric?: {
    channelDivinityTotal: number;
    channelDivinityCurrent: number;
    divineDomain: string;
    domainSpellsAlwaysPrepared: string[];
  };
  druid?: {
    wildShapeTotal: number;
    wildShapeCurrent: number;
    isWildShaped: boolean;
    currentBeastForm?: {
      name: string;
      hp: number;
      maxHp: number;
      armorClass: number;
      attacks: string[];
    };
  };
  fighter?: {
    secondWindAvailable: boolean;
    actionSurgeAvailable: boolean;
    fightingStyle: string;
    superiorityDiceTotal?: number;
    superiorityDiceCurrent?: number;
  };
  monk?: {
    kiPointsTotal: number;
    kiPointsCurrent: number;
    martialArtsDie: string;
    unarmoredDefenseFormula: string;
  };
  paladin?: {
    layOnHandsPoolTotal: number;
    layOnHandsPoolCurrent: number;
    divineSenseUsesTotal: number;
    divineSenseUsesCurrent: number;
    fightingStyle: string;
    divineSmiteUnlocked: boolean;
  };
  ranger?: {
    favoredEnemies: string[];
    naturalExplorers: string[];
    fightingStyle: string;
    huntersMarkActive: boolean;
  };
  rogue?: {
    sneakAttackDiceCount: number;
    cunningActionUnlocked: boolean;
    expertises: string[];
  };
  sorcerer?: {
    sorceryPointsTotal: number;
    sorceryPointsCurrent: number;
    metamagicKnown: string[];
    originName: string;
  };
  warlock?: {
    otherworldlyPatron: string;
    pactBoon: 'Chain' | 'Blade' | 'Tome';
    invocationsActive: string[];
  };
  wizard?: {
    arcaneRecoveryAvailable: boolean;
    spellbookSpells: string[];
    preparedSpellsMax: number;
    preparedSpellsActive: string[];
  };
}

export interface CharacterSheet {
  id: string;
  name: string;
  race: string;
  className: CharacterClassType;
  subclass?: string;
  level: number;
  background: string;
  alignment: string;
  proficiencyBonus: number;
  abilities: AbilityScores;
  savingThrowProficiencies: AbilityName[];
  skillProficiencies: string[];
  vitals: VitalStats;
  spellSlots: SpellSlots;
  spellsKnownOrPrepared: string[];
  classResources: ClassSpecificResources;
  features?: string[];
  attacksPerAction?: number;
  inventory: Array<{
    name: string;
    quantity: number;
    weightLbs: number;
    isEquipped?: boolean;
  }>;
}

export function calculateAbilityModifier(score: number): number {
  return Math.floor((score - 10) / 2);
}

export function formatModifier(mod: number): string {
  return mod >= 0 ? `+${mod}` : `${mod}`;
}
