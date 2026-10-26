import { useState, useEffect } from 'react';
import type { FC } from 'react';
import type {
  AbilityName,
  CharacterClassType,
  CharacterSheet,
  AbilityScores,
  ClassSpecificResources,
} from '../../types/character';
import {
  calculateAbilityModifier,
  formatModifier,
} from '../../types/character';
import {
  DND_CLASSES,
  BACKGROUNDS,
  STANDARD_ARRAY,
} from './constants';
import type { ClassMetadata } from './constants';
import {
  POINT_BUY_TOTAL_BUDGET,
  POINT_BUY_MIN_SCORE,
  POINT_BUY_MAX_SCORE,
  POINT_BUY_COST_TABLE,
  calculatePointBuySpent,
  SRD_RACES,
  getProficiencyBonusForLevel,
  getFixedHpAverageForHitDie,
  isAsiLevel,
  getSubclassUnlockLevel,
  SRD_SUBCLASSES,
  getSpellSlotsForClassAndLevel,
  SRD_SPELLS_BY_CIRCLE,
  getClassFeaturesAtLevel,
} from './srd51';
import type {
  AbilityGenerationMethod,
  LevelUpStepChoice,
  RaceMetadata,
} from './srd51';
import {
  Shield,
  Heart,
  Flame,
  Zap,
  BookOpen,
  Sparkles,
  Music,
  Sun,
  Trees,
  Sword,
  Compass,
  Crosshair,
  Eye,
  Check,
  ChevronRight,
  ChevronLeft,
  Sparkle,
  Dices,
  TrendingUp,
  Award,
} from 'lucide-react';

import type { SubclassOverride, CustomSpell } from '../../services/api';

interface CharacterCreationWizardProps {
  onCharacterCreated: (sheet: CharacterSheet) => void;
  onCancel?: () => void;
  isAddingToParty?: boolean;
  nextPlayerNumber?: number;
  subclassOverrides?: SubclassOverride[];
  customSpells?: CustomSpell[];
  initialSheet?: CharacterSheet;
}

const CLASS_ICONS: Record<string, FC<{ size?: number; color?: string }>> = {
  Flame,
  Music,
  Sun,
  Trees,
  Shield,
  Zap,
  Sword,
  Compass,
  Crosshair,
  Sparkles,
  Eye,
  BookOpen,
};

const ALL_ABILITIES: { key: AbilityName; label: string; short: string }[] = [
  { key: 'strength', label: 'FORÇA (STR)', short: 'FOR' },
  { key: 'dexterity', label: 'DESTREZA (DEX)', short: 'DES' },
  { key: 'constitution', label: 'CONSTITUIÇÃO (CON)', short: 'CON' },
  { key: 'intelligence', label: 'INTELIGÊNCIA (INT)', short: 'INT' },
  { key: 'wisdom', label: 'SABEDORIA (WIS)', short: 'SAB' },
  { key: 'charisma', label: 'CARISMA (CHA)', short: 'CAR' },
];

interface Rolled4d6Entry {
  dice: [number, number, number, number];
  droppedIndex: number;
  total: number;
}

function rollSingle4d6DropLowest(): Rolled4d6Entry {
  const d1 = Math.floor(Math.random() * 6) + 1;
  const d2 = Math.floor(Math.random() * 6) + 1;
  const d3 = Math.floor(Math.random() * 6) + 1;
  const d4 = Math.floor(Math.random() * 6) + 1;
  const dice: [number, number, number, number] = [d1, d2, d3, d4];
  let minVal = dice[0];
  let minIdx = 0;
  for (let i = 1; i < 4; i++) {
    if (dice[i] < minVal) {
      minVal = dice[i];
      minIdx = i;
    }
  }
  const total = d1 + d2 + d3 + d4 - minVal;
  return { dice, droppedIndex: minIdx, total };
}

export const CharacterCreationWizard: FC<CharacterCreationWizardProps> = ({
  onCharacterCreated,
  isAddingToParty = false,
  nextPlayerNumber = 2,
  subclassOverrides = [],
  customSpells = [],
  initialSheet,
}) => {
  const isEditing = Boolean(initialSheet);
  const [step, setStep] = useState<number>(1);
  const [name, setName] = useState<string>(
    initialSheet?.name || (isAddingToParty ? `Herói #${nextPlayerNumber}` : 'Valério de Arton')
  );

  const resolveInitialRace = (): string => {
    if (!initialSheet?.race) return SRD_RACES[0].id;
    const match = SRD_RACES.find(
      (r) =>
        r.name.toLowerCase() === initialSheet.race.toLowerCase() ||
        r.id.toLowerCase().includes(initialSheet.race.toLowerCase())
    );
    return match ? match.id : SRD_RACES[0].id;
  };

  const [race, setRace] = useState<string>(resolveInitialRace);
  const [halfElfPlusOnes, setHalfElfPlusOnes] = useState<[AbilityName, AbilityName]>([
    'dexterity',
    'constitution',
  ]);
  const [background, setBackground] = useState<string>(
    initialSheet?.background || BACKGROUNDS[1].name
  );
  const [alignment, setAlignment] = useState<string>(
    initialSheet?.alignment || 'Neutro e Bom'
  );
  const [selectedClass, setSelectedClass] = useState<CharacterClassType>(
    (initialSheet?.className as CharacterClassType) || 'Wizard'
  );
  const [targetLevel, setTargetLevel] = useState<number>(initialSheet?.level || 1);

  // Ability score generation method (SRD 5.1)
  const [abilityMethod, setAbilityMethod] = useState<AbilityGenerationMethod>('point_buy');

  // Base abilities before racial bonuses & before ASIs
  const [baseAbilities, setBaseAbilities] = useState<AbilityScores>(() => {
    if (initialSheet?.abilities) {
      const matchRace = SRD_RACES.find(
        (r) =>
          r.name.toLowerCase() === initialSheet.race.toLowerCase() ||
          r.id.toLowerCase().includes(initialSheet.race.toLowerCase())
      );
      const getBonus = (ab: AbilityName) => matchRace?.bonuses?.[ab] || 0;
      return {
        strength: Math.max(8, Math.min(18, initialSheet.abilities.strength - getBonus('strength'))),
        dexterity: Math.max(8, Math.min(18, initialSheet.abilities.dexterity - getBonus('dexterity'))),
        constitution: Math.max(8, Math.min(18, initialSheet.abilities.constitution - getBonus('constitution'))),
        intelligence: Math.max(8, Math.min(18, initialSheet.abilities.intelligence - getBonus('intelligence'))),
        wisdom: Math.max(8, Math.min(18, initialSheet.abilities.wisdom - getBonus('wisdom'))),
        charisma: Math.max(8, Math.min(18, initialSheet.abilities.charisma - getBonus('charisma'))),
      };
    }
    return {
      strength: 8,
      dexterity: 14,
      constitution: 14,
      intelligence: 15,
      wisdom: 12,
      charisma: 8,
    };
  });

  // 4d6 rolled pool state
  const [rolledPool, setRolledPool] = useState<Rolled4d6Entry[]>(() =>
    Array.from({ length: 6 }, () => rollSingle4d6DropLowest())
  );
  // Maps each ability to an index 0..5 of rolledPool (or STANDARD_ARRAY)
  const [poolAssignment, setPoolAssignment] = useState<Record<AbilityName, number>>({
    intelligence: 0,
    constitution: 1,
    dexterity: 2,
    wisdom: 3,
    strength: 4,
    charisma: 5,
  });

  // Level-by-level evolution state (for levels 2..20)
  const [activeEvolutionLevel, setActiveEvolutionLevel] = useState<number>(2);
  const [levelChoices, setLevelChoices] = useState<Record<number, LevelUpStepChoice>>(() => {
    if (!initialSheet || initialSheet.level <= 1) return {};
    const choices: Record<number, LevelUpStepChoice> = {};
    const subLvl = getSubclassUnlockLevel((initialSheet.className as CharacterClassType) || 'Wizard');
    for (let lvl = 2; lvl <= initialSheet.level; lvl++) {
      choices[lvl] = {
        level: lvl,
        hpMode: 'fixed_average',
        hpDieRoll: 5,
        subclassSelected: lvl >= subLvl ? initialSheet.subclass : undefined,
      };
    }
    return choices;
  });
  const [level1Subclass, setLevel1Subclass] = useState<string>(initialSheet?.subclass || '');
  const [level1SpellsSelected, setLevel1SpellsSelected] = useState<string[]>(
    initialSheet?.spellsKnownOrPrepared || []
  );

  const currentClassMeta: ClassMetadata =
    DND_CLASSES.find((c) => c.id === selectedClass) || DND_CLASSES[0];

  const isCasterClass = ['Bard', 'Cleric', 'Druid', 'Sorcerer', 'Warlock', 'Wizard'].includes(selectedClass);

  const getEffectiveSpellsForCircle = (circle: number, cls: CharacterClassType) => {
    const baseSpells = (SRD_SPELLS_BY_CIRCLE[circle] || []).map((sp) => ({
      name: sp,
      isHomebrew: false,
      school: 'SRD 5.1',
      description: 'Magia oficial das regras do SRD 5.1.',
    }));

    const matchingHomebrew = customSpells.filter(
      (cs) =>
        cs.level === circle &&
        (cs.classes.length === 0 ||
          cs.classes.some(
            (c) => c.toLowerCase() === cls.toLowerCase() || c.toLowerCase() === 'all'
          ))
    );

    const homebrewMapped = matchingHomebrew.map((cs) => ({
      name: `${cs.name} (${circle}º Círculo)`,
      isHomebrew: true,
      school: cs.school || 'Homebrew',
      description: cs.description,
    }));

    return [...homebrewMapped, ...baseSpells];
  };

  const getEffectiveSubclasses = (cls: CharacterClassType) => {
    const baseList = SRD_SUBCLASSES[cls] || [];
    const overridesForClass = subclassOverrides.filter(
      (o) => o.parent_class.toLowerCase() === cls.toLowerCase()
    );

    if (overridesForClass.length === 0) {
      return baseList.map((s) => ({ ...s, isHomebrew: false }));
    }

    let result = [...baseList.map((s) => ({ ...s, isHomebrew: false }))];

    for (const ov of overridesForClass) {
      if (ov.replaces) {
        const replaceIdx = result.findIndex(
          (s) => s.name.toLowerCase().includes(ov.replaces!.toLowerCase())
        );
        if (replaceIdx !== -1) {
          result[replaceIdx] = {
            name: ov.name,
            description: ov.description,
            isHomebrew: true,
          };
        } else {
          result.push({
            name: ov.name,
            description: ov.description,
            isHomebrew: true,
          });
        }
      } else {
        result.push({
          name: ov.name,
          description: ov.description,
          isHomebrew: true,
        });
      }
    }

    return result;
  };

  const effectiveSubclasses = getEffectiveSubclasses(selectedClass);

  const currentRaceMeta: RaceMetadata =
    SRD_RACES.find((r) => r.id === race) || SRD_RACES[0];

  // Ensure default level choices exist whenever targetLevel or selectedClass changes
  useEffect(() => {
    const avgHp = getFixedHpAverageForHitDie(currentClassMeta.hitDieValue);
    const primaryStat = currentClassMeta.primaryAbilities[0] || 'strength';
    const secondaryStat = currentClassMeta.primaryAbilities[1] || 'constitution';
    const subUnlock = getSubclassUnlockLevel(selectedClass);
    const defaultSub = effectiveSubclasses[0]?.name || '';

    if (subUnlock === 1) {
      setLevel1Subclass(defaultSub);
    }

    if (isCasterClass) {
      const spellsCircle1 = getEffectiveSpellsForCircle(1, selectedClass);
      // Pega até 4 magias iniciais padrão (ou as que já estavam selecionadas se válidas)
      setLevel1SpellsSelected((prev) => {
        if (prev.length > 0) return prev;
        return spellsCircle1.slice(0, 4).map((s) => s.name);
      });
    } else {
      setLevel1SpellsSelected([]);
    }

    setLevelChoices((prev) => {
      const next: Record<number, LevelUpStepChoice> = {};
      for (let lvl = 2; lvl <= 20; lvl++) {
        const existing = prev[lvl];
        const unlockedCircle = Math.min(9, Math.ceil(lvl / 2));
        const defaultSpellForLvl =
          SRD_SPELLS_BY_CIRCLE[unlockedCircle]?.[lvl % (SRD_SPELLS_BY_CIRCLE[unlockedCircle]?.length || 1)] ||
          SRD_SPELLS_BY_CIRCLE[1][0];

        next[lvl] = {
          level: lvl,
          hpMode: existing?.hpMode || 'fixed_average',
          hpDieRoll: existing?.hpMode === 'rolled' ? existing.hpDieRoll : avgHp,
          asiMode: existing?.asiMode || 'plus_two_one',
          asiPrimaryStat: existing?.asiPrimaryStat || primaryStat,
          asiSecondaryStat: existing?.asiSecondaryStat || secondaryStat,
          subclassSelected:
            lvl === subUnlock
              ? existing?.subclassSelected || defaultSub
              : undefined,
          newSpellAdded: existing?.newSpellAdded || defaultSpellForLvl,
        };
      }
      return next;
    });
  }, [selectedClass, currentClassMeta.hitDieValue, currentClassMeta.primaryAbilities]);

  // Compute racial bonus for a given ability
  const getRacialBonus = (ability: AbilityName): number => {
    let bonus = currentRaceMeta.bonuses[ability] || 0;
    if (currentRaceMeta.customPlusOneCount && ability !== 'charisma') {
      if (halfElfPlusOnes[0] === ability) bonus += 1;
      if (halfElfPlusOnes[1] === ability && halfElfPlusOnes[1] !== halfElfPlusOnes[0]) bonus += 1;
    }
    return bonus;
  };

  // Compute Level 1 abilities (Base + Racial)
  const level1Abilities: AbilityScores = {
    strength: baseAbilities.strength + getRacialBonus('strength'),
    dexterity: baseAbilities.dexterity + getRacialBonus('dexterity'),
    constitution: baseAbilities.constitution + getRacialBonus('constitution'),
    intelligence: baseAbilities.intelligence + getRacialBonus('intelligence'),
    wisdom: baseAbilities.wisdom + getRacialBonus('wisdom'),
    charisma: baseAbilities.charisma + getRacialBonus('charisma'),
  };

  // Compute cumulative abilities up to a given level (including ASIs up to `upToLevel`, capped at 20 per SRD 5.1)
  const getCumulativeAbilitiesAtLevel = (upToLevel: number): AbilityScores => {
    const result: AbilityScores = { ...level1Abilities };
    for (let lvl = 2; lvl <= upToLevel; lvl++) {
      if (isAsiLevel(selectedClass, lvl)) {
        const choice = levelChoices[lvl];
        if (!choice) continue;
        if (choice.asiMode === 'plus_two_one' && choice.asiPrimaryStat) {
          result[choice.asiPrimaryStat] = Math.min(20, result[choice.asiPrimaryStat] + 2);
        } else if (choice.asiMode === 'plus_one_two') {
          if (choice.asiPrimaryStat) {
            result[choice.asiPrimaryStat] = Math.min(20, result[choice.asiPrimaryStat] + 1);
          }
          if (choice.asiSecondaryStat) {
            result[choice.asiSecondaryStat] = Math.min(20, result[choice.asiSecondaryStat] + 1);
          }
        }
      }
    }
    return result;
  };

  const finalAbilities = getCumulativeAbilitiesAtLevel(targetLevel);
  const finalDexMod = calculateAbilityModifier(finalAbilities.dexterity);
  const finalConMod = calculateAbilityModifier(finalAbilities.constitution);
  const finalIntMod = calculateAbilityModifier(finalAbilities.intelligence);
  const finalWisMod = calculateAbilityModifier(finalAbilities.wisdom);
  const finalChaMod = calculateAbilityModifier(finalAbilities.charisma);
  const finalProficiency = getProficiencyBonusForLevel(targetLevel);

  // Calculate cumulative HP at any level `upToLevel` using official SRD 5.1 retroactive CON rule
  const getCumulativeHpAtLevel = (upToLevel: number): number => {
    const abilitiesAtLvl = getCumulativeAbilitiesAtLevel(upToLevel);
    const conModAtLvl = calculateAbilityModifier(abilitiesAtLvl.constitution);
    const hillDwarfBonus = currentRaceMeta.hpBonusPerLevel || 0;
    const draconicSorcererBonus =
      selectedClass === 'Sorcerer' &&
      (level1Subclass.includes('Dracônica') || !level1Subclass)
        ? 1
        : 0;

    // Level 1 HP: Max Hit Die + CON mod
    let totalHp = Math.max(
      1,
      currentClassMeta.hitDieValue + conModAtLvl + hillDwarfBonus + draconicSorcererBonus
    );

    // Levels 2..upToLevel
    const fixedAvg = getFixedHpAverageForHitDie(currentClassMeta.hitDieValue);
    for (let lvl = 2; lvl <= upToLevel; lvl++) {
      const choice = levelChoices[lvl];
      const dieVal =
        choice?.hpMode === 'rolled' ? choice.hpDieRoll : fixedAvg;
      const gained = Math.max(1, dieVal + conModAtLvl) + hillDwarfBonus + draconicSorcererBonus;
      totalHp += gained;
    }

    return totalHp;
  };

  const calculatedHp = getCumulativeHpAtLevel(targetLevel);

  // AC calculation
  let calculatedAc = 10 + finalDexMod;
  if (selectedClass === 'Barbarian') {
    calculatedAc = 10 + finalDexMod + finalConMod;
  } else if (selectedClass === 'Monk') {
    calculatedAc = 10 + finalDexMod + finalWisMod;
  } else if (
    selectedClass === 'Sorcerer' &&
    (level1Subclass.includes('Dracônica') || !level1Subclass)
  ) {
    calculatedAc = 13 + finalDexMod;
  }

  // Point buy spent
  const pointBuySpent = calculatePointBuySpent(baseAbilities);
  const pointBuyRemaining = POINT_BUY_TOTAL_BUDGET - pointBuySpent;

  // Point Buy handler (+/- with SRD 5.1 validation)
  const handlePointBuyChange = (ability: AbilityName, delta: number) => {
    setBaseAbilities((prev) => {
      const currentVal = prev[ability];
      const nextVal = currentVal + delta;
      if (nextVal < POINT_BUY_MIN_SCORE || nextVal > POINT_BUY_MAX_SCORE) {
        return prev;
      }
      const nextScores = { ...prev, [ability]: nextVal };
      const nextSpent = calculatePointBuySpent(nextScores);
      if (nextSpent > POINT_BUY_TOTAL_BUDGET) {
        return prev;
      }
      return nextScores;
    });
  };

  // Auto-optimize 27 Point Buy for the current class
  const handleOptimizePointBuyForClass = () => {
    const primary = currentClassMeta.primaryAbilities[0] || 'strength';
    const secondary = currentClassMeta.primaryAbilities[1] || 'constitution';
    const optimized: AbilityScores = {
      strength: 8,
      dexterity: 10,
      constitution: 12,
      intelligence: 10,
      wisdom: 10,
      charisma: 8,
    };

    // Spend exactly 27 points: 15 (9) + 15 (9) + 13 (5) + 12 (4) + 8 (0) + 8 (0) = 27 pts!
    const allStats: AbilityName[] = [
      'strength',
      'dexterity',
      'constitution',
      'intelligence',
      'wisdom',
      'charisma',
    ];
    const thirdStat: AbilityName =
      primary !== 'constitution' && secondary !== 'constitution'
        ? 'constitution'
        : primary !== 'dexterity' && secondary !== 'dexterity'
        ? 'dexterity'
        : 'wisdom';

    const remaining = allStats.filter(
      (s) => s !== primary && s !== secondary && s !== thirdStat
    );

    optimized[primary] = 15; // 9 pts
    optimized[secondary] = 15; // 9 pts
    optimized[thirdStat] = 13; // 5 pts
    optimized[remaining[0]] = 12; // 4 pts
    optimized[remaining[1]] = 8; // 0 pts
    optimized[remaining[2]] = 8; // 0 pts

    setBaseAbilities(optimized);
  };

  // Apply Standard Array [15, 14, 13, 12, 10, 8] or Rolled Pool according to assignment
  const applyPoolToBaseAbilities = (
    values: number[],
    assignment: Record<AbilityName, number>
  ) => {
    const next: AbilityScores = {
      strength: values[assignment.strength] ?? 10,
      dexterity: values[assignment.dexterity] ?? 10,
      constitution: values[assignment.constitution] ?? 10,
      intelligence: values[assignment.intelligence] ?? 10,
      wisdom: values[assignment.wisdom] ?? 10,
      charisma: values[assignment.charisma] ?? 10,
    };
    setBaseAbilities(next);
  };

  const handleAssignPoolSlot = (ability: AbilityName, targetSlotIndex: number) => {
    const values =
      abilityMethod === 'standard_array'
        ? STANDARD_ARRAY
        : rolledPool.map((r) => r.total);

    setPoolAssignment((prev) => {
      const currentSlotOfAbility = prev[ability];
      // Find which ability currently holds targetSlotIndex and swap
      const otherAbility = (Object.keys(prev) as AbilityName[]).find(
        (k) => prev[k] === targetSlotIndex
      );
      const nextAssignment = { ...prev, [ability]: targetSlotIndex };
      if (otherAbility && otherAbility !== ability) {
        nextAssignment[otherAbility] = currentSlotOfAbility;
      }
      applyPoolToBaseAbilities(values, nextAssignment);
      return nextAssignment;
    });
  };

  const handleAutoAssignPoolForClass = (
    methodOverride?: AbilityGenerationMethod,
    customPool?: Rolled4d6Entry[]
  ) => {
    const activeMethod = methodOverride || abilityMethod;
    const rawValues =
      activeMethod === 'standard_array'
        ? [...STANDARD_ARRAY]
        : (customPool || rolledPool).map((r) => r.total);

    // Sort indices by value descending
    const indexed = rawValues.map((v, i) => ({ v, i })).sort((a, b) => b.v - a.v);

    const primary = currentClassMeta.primaryAbilities[0] || 'strength';
    const secondary = currentClassMeta.primaryAbilities[1] || 'constitution';
    const thirdStat: AbilityName =
      primary !== 'constitution' && secondary !== 'constitution'
        ? 'constitution'
        : primary !== 'dexterity' && secondary !== 'dexterity'
        ? 'dexterity'
        : 'wisdom';

    const allStats: AbilityName[] = [
      'strength',
      'dexterity',
      'constitution',
      'intelligence',
      'wisdom',
      'charisma',
    ];
    const others = allStats.filter(
      (s) => s !== primary && s !== secondary && s !== thirdStat
    );

    const nextAssignment: Record<AbilityName, number> = {
      [primary]: indexed[0].i,
      [secondary]: indexed[1].i,
      [thirdStat]: indexed[2].i,
      [others[0]]: indexed[3].i,
      [others[1]]: indexed[4].i,
      [others[2]]: indexed[5].i,
    } as Record<AbilityName, number>;

    setPoolAssignment(nextAssignment);
    applyPoolToBaseAbilities(rawValues, nextAssignment);
  };

  const handleReroll4d6 = () => {
    const newRolls = Array.from({ length: 6 }, () => rollSingle4d6DropLowest());
    setRolledPool(newRolls);
    handleAutoAssignPoolForClass('roll_4d6', newRolls);
  };

  const handleSwitchMethod = (newMethod: AbilityGenerationMethod) => {
    setAbilityMethod(newMethod);
    if (newMethod === 'point_buy') {
      handleOptimizePointBuyForClass();
    } else if (newMethod === 'standard_array') {
      handleAutoAssignPoolForClass('standard_array');
    } else if (newMethod === 'roll_4d6') {
      handleAutoAssignPoolForClass('roll_4d6', rolledPool);
    }
  };

  // Level-up step handlers
  const updateLevelChoice = (lvl: number, patch: Partial<LevelUpStepChoice>) => {
    setLevelChoices((prev) => ({
      ...prev,
      [lvl]: {
        ...prev[lvl],
        ...patch,
      },
    }));
  };

  const handleRollHpForLevel = (lvl: number) => {
    const rolled = Math.floor(Math.random() * currentClassMeta.hitDieValue) + 1;
    updateLevelChoice(lvl, {
      hpMode: 'rolled',
      hpDieRoll: rolled,
    });
  };

  const handleFixedHpForLevel = (lvl: number) => {
    const avg = getFixedHpAverageForHitDie(currentClassMeta.hitDieValue);
    updateLevelChoice(lvl, {
      hpMode: 'fixed_average',
      hpDieRoll: avg,
    });
  };

  const handleAutoRollAllLevels = () => {
    const avg = getFixedHpAverageForHitDie(currentClassMeta.hitDieValue);
    const primaryStat = currentClassMeta.primaryAbilities[0] || 'strength';
    const secondaryStat = currentClassMeta.primaryAbilities[1] || 'constitution';

    setLevelChoices((prev) => {
      const next = { ...prev };
      for (let lvl = 2; lvl <= targetLevel; lvl++) {
        next[lvl] = {
          ...next[lvl],
          level: lvl,
          hpMode: 'fixed_average',
          hpDieRoll: avg,
          asiMode: 'plus_two_one',
          asiPrimaryStat: primaryStat,
          asiSecondaryStat: secondaryStat,
        };
      }
      return next;
    });
  };

  // Gather all unlocked features from Level 1 to targetLevel
  const getAllUnlockedFeatures = (): string[] => {
    const list: string[] = [...currentRaceMeta.traits];
    for (let lvl = 1; lvl <= targetLevel; lvl++) {
      const feats = getClassFeaturesAtLevel(selectedClass, lvl);
      feats.forEach((f) => list.push(`Nível ${lvl}: ${f}`));
    }
    return list;
  };

  // Determine chosen subclass name
  const subUnlockLvl = getSubclassUnlockLevel(selectedClass);
  const chosenSubclass =
    subUnlockLvl === 1
      ? level1Subclass || effectiveSubclasses[0]?.name
      : targetLevel >= subUnlockLvl
      ? levelChoices[subUnlockLvl]?.subclassSelected || effectiveSubclasses[0]?.name
      : undefined;

  // Determine attacks per action based on class & level (SRD 5.1)
  const getAttacksPerAction = (): number => {
    if (selectedClass === 'Fighter') {
      if (targetLevel >= 20) return 4;
      if (targetLevel >= 11) return 3;
      if (targetLevel >= 5) return 2;
      return 1;
    }
    if (['Barbarian', 'Paladin', 'Ranger', 'Monk'].includes(selectedClass)) {
      return targetLevel >= 5 ? 2 : 1;
    }
    return 1;
  };

  const buildFinalCharacterSheet = (): CharacterSheet => {
    const computedSpellSlots = getSpellSlotsForClassAndLevel(selectedClass, targetLevel);

    // Collect spells learned across levels
    const collectedSpells: string[] = isCasterClass
      ? [...level1SpellsSelected]
      : [];
    for (let lvl = 2; lvl <= targetLevel; lvl++) {
      const sp = levelChoices[lvl]?.newSpellAdded;
      if (sp && !collectedSpells.includes(sp)) {
        collectedSpells.push(sp);
      }
    }

    const classResources: ClassSpecificResources = {};
    if (selectedClass === 'Barbarian') {
      const rages =
        targetLevel >= 20
          ? 99
          : targetLevel >= 17
          ? 6
          : targetLevel >= 12
          ? 5
          : targetLevel >= 6
          ? 4
          : targetLevel >= 3
          ? 3
          : 2;
      const rageDmg = targetLevel >= 16 ? 4 : targetLevel >= 9 ? 3 : 2;
      classResources.barbarian = {
        rageUsesTotal: rages,
        rageUsesCurrent: rages,
        isRaging: false,
        rageDamageBonus: rageDmg,
        unarmoredDefenseFormula: '10 + Mod. DES + Mod. CON',
      };
    } else if (selectedClass === 'Bard') {
      const die =
        targetLevel >= 15 ? 'd12' : targetLevel >= 10 ? 'd10' : targetLevel >= 5 ? 'd8' : 'd6';
      classResources.bard = {
        inspirationDiceTotal: Math.max(1, finalChaMod),
        inspirationDiceCurrent: Math.max(1, finalChaMod),
        inspirationDieType: die,
        songOfRestDie: targetLevel >= 17 ? 'd12' : targetLevel >= 13 ? 'd10' : targetLevel >= 9 ? 'd8' : 'd6',
      };
    } else if (selectedClass === 'Cleric') {
      const cdUses = targetLevel >= 18 ? 3 : targetLevel >= 6 ? 2 : 1;
      classResources.cleric = {
        channelDivinityTotal: cdUses,
        channelDivinityCurrent: cdUses,
        divineDomain: chosenSubclass || 'Domínio da Vida',
        domainSpellsAlwaysPrepared: ['Bênção', 'Curar Ferimentos', 'Arma Espiritual', 'Espíritos Guardiões'],
      };
    } else if (selectedClass === 'Druid') {
      classResources.druid = {
        wildShapeTotal: targetLevel >= 20 ? 99 : 2,
        wildShapeCurrent: targetLevel >= 20 ? 99 : 2,
        isWildShaped: false,
      };
    } else if (selectedClass === 'Fighter') {
      classResources.fighter = {
        secondWindAvailable: true,
        actionSurgeAvailable: targetLevel >= 2,
        fightingStyle: 'Duelo (+2 dano com 1 arma)',
        superiorityDiceTotal: targetLevel >= 15 ? 6 : targetLevel >= 7 ? 5 : 4,
        superiorityDiceCurrent: targetLevel >= 15 ? 6 : targetLevel >= 7 ? 5 : 4,
      };
    } else if (selectedClass === 'Monk') {
      const die =
        targetLevel >= 17 ? '1d10' : targetLevel >= 11 ? '1d8' : targetLevel >= 5 ? '1d6' : '1d4';
      classResources.monk = {
        kiPointsTotal: targetLevel >= 2 ? targetLevel : 0,
        kiPointsCurrent: targetLevel >= 2 ? targetLevel : 0,
        martialArtsDie: die,
        unarmoredDefenseFormula: '10 + Mod. DES + Mod. SAB',
      };
    } else if (selectedClass === 'Paladin') {
      classResources.paladin = {
        layOnHandsPoolTotal: targetLevel * 5,
        layOnHandsPoolCurrent: targetLevel * 5,
        divineSenseUsesTotal: Math.max(1, 1 + finalChaMod),
        divineSenseUsesCurrent: Math.max(1, 1 + finalChaMod),
        fightingStyle: 'Defesa (+1 CA)',
        divineSmiteUnlocked: targetLevel >= 2,
      };
    } else if (selectedClass === 'Ranger') {
      classResources.ranger = {
        favoredEnemies: ['Bestas selvagens', 'Goblinóides e Metamorfos'],
        naturalExplorers: ['Floresta', 'Montanha'],
        fightingStyle: 'Arquearia (+2 ataque a distância)',
        huntersMarkActive: false,
      };
    } else if (selectedClass === 'Rogue') {
      classResources.rogue = {
        sneakAttackDiceCount: Math.ceil(targetLevel / 2),
        cunningActionUnlocked: targetLevel >= 2,
        expertises: ['Furtividade', 'Prestidigitação', ...(targetLevel >= 6 ? ['Percepção', 'Investigação'] : [])],
      };
    } else if (selectedClass === 'Sorcerer') {
      classResources.sorcerer = {
        sorceryPointsTotal: targetLevel >= 2 ? targetLevel : 0,
        sorceryPointsCurrent: targetLevel >= 2 ? targetLevel : 0,
        metamagicKnown: targetLevel >= 3 ? ['Magia Acelerada', 'Magia Gêmea'] : [],
        originName: chosenSubclass || 'Linhagem Dracônica',
      };
    } else if (selectedClass === 'Warlock') {
      classResources.warlock = {
        otherworldlyPatron: chosenSubclass || 'O Corruptor (Fiend)',
        pactBoon: 'Tome',
        invocationsActive: ['Visão Diabólica', 'Rajada Agonizante'],
      };
    } else if (selectedClass === 'Wizard') {
      classResources.wizard = {
        arcaneRecoveryAvailable: true,
        spellbookSpells: collectedSpells,
        preparedSpellsMax: Math.max(1, finalIntMod + targetLevel),
        preparedSpellsActive: collectedSpells.slice(0, Math.max(2, finalIntMod + targetLevel)),
      };
    }

    const extraSpeed =
      (selectedClass === 'Barbarian' && targetLevel >= 5 ? 10 : 0) +
      (selectedClass === 'Monk' && targetLevel >= 2 ? Math.floor((targetLevel + 2) / 4) * 5 + 5 : 0);

    return {
      id: initialSheet?.id || `char-${Date.now()}`,
      name,
      race: currentRaceMeta.name,
      className: selectedClass,
      subclass: chosenSubclass,
      level: targetLevel,
      background,
      alignment,
      proficiencyBonus: finalProficiency,
      abilities: finalAbilities,
      savingThrowProficiencies:
        selectedClass === 'Monk' && targetLevel >= 14
          ? ['strength', 'dexterity', 'constitution', 'intelligence', 'wisdom', 'charisma']
          : selectedClass === 'Rogue' && targetLevel >= 15
          ? [...currentClassMeta.savingThrows, 'wisdom']
          : currentClassMeta.savingThrows,
      skillProficiencies: BACKGROUNDS.find((b) => b.name === background)?.skills || [
        'Percepção',
      ],
      vitals: {
        hpCurrent: calculatedHp,
        hpMax: calculatedHp,
        hpTemp: 0,
        armorClass: calculatedAc,
        initiativeModifier: finalDexMod,
        speedFeet: currentRaceMeta.speedFeet + extraSpeed,
        hitDiceTotal: targetLevel,
        hitDiceCurrent: targetLevel,
        hitDiceType: currentClassMeta.hitDie,
        deathSaves: { successes: 0, failures: 0 },
        conditions: [],
      },
      spellSlots: computedSpellSlots,
      spellsKnownOrPrepared: currentClassMeta.isSpellcaster ? collectedSpells : [],
      classResources,
      features: getAllUnlockedFeatures(),
      attacksPerAction: getAttacksPerAction(),
      inventory: initialSheet?.inventory?.length
        ? initialSheet.inventory
        : [
            { name: 'Arma Principal da Classe', quantity: 1, weightLbs: 3, isEquipped: true },
            { name: 'Mochila de Aventureiro', quantity: 1, weightLbs: 5 },
            { name: 'Poção de Cura (2d4+2)', quantity: Math.max(1, Math.floor(targetLevel / 3)), weightLbs: 1 },
            { name: 'Tochas', quantity: 5, weightLbs: 5 },
          ],
    };
  };

  const handleFinish = () => {
    const sheet = buildFinalCharacterSheet();
    onCharacterCreated(sheet);
  };

  const wizardSteps =
    targetLevel > 1
      ? [
          { num: 1, label: '1. Identidade, Raça & Nível' },
          { num: 2, label: '2. As 12 Classes Originais' },
          { num: 3, label: '3. Atributos Nível 1 (SRD 5.1)' },
          { num: 4, label: `4. Evolução Nível a Nível (1 → ${targetLevel})` },
          { num: 5, label: '5. Revisão & Entrada' },
        ]
      : [
          { num: 1, label: '1. Identidade, Raça & Nível' },
          { num: 2, label: '2. As 12 Classes Originais' },
          { num: 3, label: '3. Atributos Nível 1 (SRD 5.1)' },
          { num: 5, label: '4. Revisão & Entrada' },
        ];

  const activeEvoChoice = levelChoices[activeEvolutionLevel];
  const unlockedCircleAtActiveLvl = Math.min(9, Math.ceil(activeEvolutionLevel / 2));
  const subUnlockForSelected = getSubclassUnlockLevel(selectedClass);

  return (
    <div style={{ maxWidth: '1180px', margin: '0 auto', padding: '24px 16px' }}>
      {/* Header */}
      <div style={{ textAlign: 'center', marginBottom: '24px', position: 'relative' }}>
        <h1
          className="rpg-title"
          style={{
            fontSize: '30px',
            color: 'var(--accent-gold)',
            marginBottom: '6px',
            textShadow: '0 0 15px rgba(234, 179, 8, 0.3)',
          }}
        >
          {isEditing
            ? `Editar Herói: ${name} (Nv.${targetLevel} ${selectedClass})`
            : isAddingToParty
            ? `Adicionar Novo Herói à Party (Jogador #${nextPlayerNumber}) — D&D 5e`
            : 'Criação & Evolução de Personagem — D&D 5e (SRD 5.1)'}
        </h1>
        <p style={{ color: 'var(--text-muted)', fontSize: '14px' }}>
          Regras oficiais de atributos (Point Buy 27 pts, Matriz Padrão 72 pts ou 4d6 Drop 1) + Progressão Nível a Nível do 1 ao 20.
        </p>

        {/* Wizard Steps indicator */}
        <div
          style={{
            display: 'flex',
            justifyContent: 'center',
            flexWrap: 'wrap',
            gap: '10px',
            marginTop: '18px',
          }}
        >
          {wizardSteps.map((s) => (
            <button
              key={s.num}
              onClick={() => setStep(s.num)}
              data-testid={`wizard-step-tab-${s.num}`}
              style={{
                padding: '7px 14px',
                borderRadius: '20px',
                fontSize: '12px',
                fontWeight: 700,
                backgroundColor: step === s.num ? 'var(--accent-gold)' : 'var(--bg-card)',
                color: step === s.num ? '#000' : 'var(--text-muted)',
                border:
                  step === s.num
                    ? '1px solid var(--accent-gold)'
                    : '1px solid var(--border-color)',
                transition: 'all 0.2s',
                cursor: 'pointer',
              }}
            >
              {s.label}
            </button>
          ))}
        </div>
      </div>

      {/* STEP 1: IDENTITY, RACE & INITIAL LEVEL */}
      {step === 1 && (
        <div
          style={{
            backgroundColor: 'var(--bg-card)',
            border: '1px solid var(--border-color)',
            borderRadius: '12px',
            padding: '24px',
          }}
        >
          <h2
            className="rpg-title"
            style={{ fontSize: '20px', color: '#fff', marginBottom: '16px' }}
          >
            Passo 1: Identidade, Raça SRD 5.1 &amp; Nível Inicial da Campanha
          </h2>

          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fit, minmax(260px, 1fr))',
              gap: '20px',
            }}
          >
            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '13px',
                  color: 'var(--text-muted)',
                  marginBottom: '6px',
                }}
              >
                Nome do Personagem:
              </label>
              <input
                type="text"
                value={name}
                onChange={(e) => setName(e.target.value)}
                style={{ width: '100%', fontSize: '15px' }}
                placeholder="Ex: Valério de Arton"
                data-testid="wizard-hero-name"
              />
            </div>

            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '13px',
                  color: 'var(--text-muted)',
                  marginBottom: '6px',
                }}
              >
                Raça Oficial (SRD 5.1):
              </label>
              <select
                value={race}
                onChange={(e) => setRace(e.target.value)}
                style={{ width: '100%', fontSize: '15px' }}
              >
                {SRD_RACES.map((r) => (
                  <option key={r.id} value={r.id}>
                    {r.id}
                  </option>
                ))}
              </select>
            </div>

            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '13px',
                  color: 'var(--text-muted)',
                  marginBottom: '6px',
                }}
              >
                Antecedente (Background):
              </label>
              <select
                value={background}
                onChange={(e) => setBackground(e.target.value)}
                style={{ width: '100%', fontSize: '15px' }}
              >
                {BACKGROUNDS.map((b) => (
                  <option key={b.name} value={b.name}>
                    {b.name} (Perícias: {b.skills.join(', ')})
                  </option>
                ))}
              </select>
            </div>

            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '13px',
                  color: 'var(--text-muted)',
                  marginBottom: '6px',
                }}
              >
                Tendência (Alignment):
              </label>
              <select
                value={alignment}
                onChange={(e) => setAlignment(e.target.value)}
                style={{ width: '100%', fontSize: '15px' }}
              >
                <option value="Leal e Bom">Leal e Bom</option>
                <option value="Neutro e Bom">Neutro e Bom</option>
                <option value="Caótico e Bom">Caótico e Bom</option>
                <option value="Leal e Neutro">Leal e Neutro</option>
                <option value="Neutro Puro">Neutro Puro</option>
                <option value="Caótico e Neutro">Caótico e Neutro</option>
              </select>
            </div>
          </div>

          {/* Half-Elf +1/+1 selector */}
          {currentRaceMeta.customPlusOneCount && (
            <div
              style={{
                marginTop: '16px',
                padding: '14px',
                borderRadius: '8px',
                backgroundColor: '#111622',
                border: '1px solid #3b82f6',
              }}
            >
              <div style={{ fontSize: '13px', fontWeight: 700, color: '#93c5fd', marginBottom: '8px' }}>
                Meio-Elfo (SRD 5.1): Recebe +2 em Carisma e +1 em outros DOIS atributos à sua escolha:
              </div>
              <div style={{ display: 'flex', gap: '16px', flexWrap: 'wrap' }}>
                <div>
                  <span style={{ fontSize: '12px', color: 'var(--text-muted)', marginRight: '8px' }}>
                    1º Atributo (+1):
                  </span>
                  <select
                    value={halfElfPlusOnes[0]}
                    onChange={(e) =>
                      setHalfElfPlusOnes([e.target.value as AbilityName, halfElfPlusOnes[1]])
                    }
                  >
                    {ALL_ABILITIES.filter((a) => a.key !== 'charisma').map((a) => (
                      <option key={a.key} value={a.key}>
                        {a.label}
                      </option>
                    ))}
                  </select>
                </div>
                <div>
                  <span style={{ fontSize: '12px', color: 'var(--text-muted)', marginRight: '8px' }}>
                    2º Atributo (+1):
                  </span>
                  <select
                    value={halfElfPlusOnes[1]}
                    onChange={(e) =>
                      setHalfElfPlusOnes([halfElfPlusOnes[0], e.target.value as AbilityName])
                    }
                  >
                    {ALL_ABILITIES.filter(
                      (a) => a.key !== 'charisma' && a.key !== halfElfPlusOnes[0]
                    ).map((a) => (
                      <option key={a.key} value={a.key}>
                        {a.label}
                      </option>
                    ))}
                  </select>
                </div>
              </div>
            </div>
          )}

          {/* Racial traits preview */}
          <div
            style={{
              marginTop: '16px',
              padding: '12px 16px',
              borderRadius: '8px',
              backgroundColor: '#111419',
              border: '1px solid var(--border-color)',
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
              flexWrap: 'wrap',
              gap: '10px',
            }}
          >
            <div>
              <span style={{ fontSize: '12px', fontWeight: 700, color: 'var(--accent-gold)' }}>
                Traços Raciais ({currentRaceMeta.name}):{' '}
              </span>
              <span style={{ fontSize: '12px', color: 'var(--text-muted)' }}>
                {currentRaceMeta.traits.join(' • ')}
              </span>
            </div>
            <span
              style={{
                fontSize: '11px',
                padding: '3px 8px',
                borderRadius: '4px',
                backgroundColor: '#1e293b',
                color: '#38bdf8',
                fontWeight: 700,
              }}
            >
              Deslocamento Base: {currentRaceMeta.speedFeet} pés
            </span>
          </div>

          {/* TARGET LEVEL SELECTOR (1 TO 20) */}
          <div
            style={{
              marginTop: '20px',
              padding: '18px 20px',
              borderRadius: '10px',
              backgroundColor: 'rgba(234, 179, 8, 0.06)',
              border: '1px solid rgba(234, 179, 8, 0.35)',
            }}
          >
            <div
              style={{
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                flexWrap: 'wrap',
                gap: '12px',
                marginBottom: '10px',
              }}
            >
              <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                <TrendingUp size={18} color="var(--accent-gold)" />
                <span className="rpg-title" style={{ fontSize: '16px', color: 'var(--accent-gold)', fontWeight: 700 }}>
                  Nível Inicial da Campanha (1 ao 20 — Progressão SRD 5.1)
                </span>
              </div>
              <span
                style={{
                  fontSize: '14px',
                  fontWeight: 800,
                  padding: '4px 14px',
                  borderRadius: '20px',
                  backgroundColor: 'var(--accent-gold)',
                  color: '#000',
                }}
              >
                Nível Inicial Escolhido: Nível {targetLevel} (Proficiência +{getProficiencyBonusForLevel(targetLevel)})
              </span>
            </div>

            <p style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '12px' }}>
              Se você escolher um nível acima de 1 (ex: Nível 15), o sistema habilitará o{' '}
              <strong style={{ color: '#fff' }}>Passo 4: Evolução Nível a Nível</strong> para você evoluir cada nível individualmente (dados de vida, ASIs nos níveis 4/8/12/16/19, subclasse, características e círculos de magia até o 9º círculo).
            </p>

            <div style={{ display: 'flex', alignItems: 'center', gap: '14px', flexWrap: 'wrap' }}>
              <input
                type="range"
                min={1}
                max={20}
                value={targetLevel}
                onChange={(e) => {
                  const val = Number(e.target.value);
                  setTargetLevel(val);
                  if (activeEvolutionLevel > val && val >= 2) {
                    setActiveEvolutionLevel(val);
                  }
                }}
                style={{ flex: 1, minWidth: '220px', accentColor: 'var(--accent-gold)' }}
              />
              <div style={{ display: 'flex', gap: '6px', flexWrap: 'wrap' }}>
                {[1, 3, 5, 10, 15, 20].map((preset) => (
                  <button
                    key={preset}
                    type="button"
                    onClick={() => {
                      setTargetLevel(preset);
                      if (preset >= 2) setActiveEvolutionLevel(2);
                    }}
                    style={{
                      padding: '5px 10px',
                      borderRadius: '6px',
                      fontSize: '12px',
                      fontWeight: 700,
                      backgroundColor: targetLevel === preset ? 'var(--accent-gold)' : '#161a22',
                      color: targetLevel === preset ? '#000' : 'var(--text-muted)',
                      border: '1px solid var(--border-color)',
                      cursor: 'pointer',
                    }}
                  >
                    Nível {preset}
                  </button>
                ))}
              </div>
            </div>
          </div>

          <div style={{ display: 'flex', justifyContent: 'flex-end', marginTop: '24px' }}>
            <button
              onClick={() => setStep(2)}
              data-testid="wizard-step1-next-btn"
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '8px',
                padding: '10px 20px',
                borderRadius: '8px',
                backgroundColor: 'var(--accent-gold)',
                color: '#000',
                fontWeight: 'bold',
                cursor: 'pointer',
              }}
            >
              Avançar para Escolha de Classe <ChevronRight size={18} />
            </button>
          </div>
        </div>
      )}

      {/* STEP 2: THE 12 CLASSES */}
      {step === 2 && (
        <div>
          <div
            style={{
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
              marginBottom: '16px',
              flexWrap: 'wrap',
              gap: '10px',
            }}
          >
            <div>
              <h2 className="rpg-title" style={{ fontSize: '20px', color: '#fff' }}>
                Passo 2: Selecione sua Classe (1 das 12 Classes Oficiais SRD 5.1)
              </h2>
              <p style={{ color: 'var(--text-muted)', fontSize: '13px' }}>
                Cada classe possui um dado de vida, salvaguardas, progressão de níveis e cockpit tático exclusivo.
              </p>
            </div>
            <span
              style={{
                fontSize: '13px',
                padding: '4px 10px',
                borderRadius: '6px',
                backgroundColor: 'var(--accent-gold-bg)',
                color: 'var(--accent-gold)',
                fontWeight: 600,
              }}
            >
              Classe Selecionada: {currentClassMeta.namePt} ({currentClassMeta.hitDie})
            </span>
          </div>

          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fill, minmax(240px, 1fr))',
              gap: '14px',
            }}
          >
            {DND_CLASSES.map((cls) => {
              const isSelected = selectedClass === cls.id;
              const IconComp = CLASS_ICONS[cls.iconName] || Sword;

              return (
                <div
                  key={cls.id}
                  data-testid={`class-card-${cls.id}`}
                  onClick={() => setSelectedClass(cls.id)}
                  style={{
                    backgroundColor: isSelected ? 'var(--bg-card-hover)' : 'var(--bg-card)',
                    border: isSelected
                      ? `2px solid ${cls.color}`
                      : '1px solid var(--border-color)',
                    borderRadius: '10px',
                    padding: '16px',
                    cursor: 'pointer',
                    position: 'relative',
                    transition: 'all 0.15s ease-in-out',
                    boxShadow: isSelected ? `0 0 15px ${cls.color}33` : 'none',
                  }}
                >
                  <div
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'space-between',
                      marginBottom: '8px',
                    }}
                  >
                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                      <div
                        style={{
                          width: '32px',
                          height: '32px',
                          borderRadius: '6px',
                          backgroundColor: `${cls.color}22`,
                          display: 'flex',
                          alignItems: 'center',
                          justifyContent: 'center',
                        }}
                      >
                        <IconComp size={18} color={cls.color} />
                      </div>
                      <span
                        className="rpg-title"
                        style={{
                          fontWeight: 'bold',
                          fontSize: '15px',
                          color: isSelected ? cls.color : '#fff',
                        }}
                      >
                        {cls.namePt}
                      </span>
                    </div>

                    <span
                      style={{
                        fontSize: '11px',
                        padding: '2px 6px',
                        borderRadius: '4px',
                        backgroundColor: '#1f242d',
                        color: 'var(--text-muted)',
                      }}
                    >
                      Dado: {cls.hitDie}
                    </span>
                  </div>

                  <p
                    style={{
                      fontSize: '12px',
                      color: 'var(--text-muted)',
                      lineHeight: '1.4',
                      marginBottom: '10px',
                    }}
                  >
                    {cls.description}
                  </p>

                  <div
                    style={{
                      display: 'flex',
                      flexWrap: 'wrap',
                      gap: '4px',
                      fontSize: '10px',
                      color: 'var(--text-faint)',
                    }}
                  >
                    <span>Salvaguardas: {cls.savingThrows.join(', ')}</span>
                  </div>

                  {isSelected && (
                    <div
                      style={{
                        position: 'absolute',
                        top: '8px',
                        right: '8px',
                        width: '18px',
                        height: '18px',
                        borderRadius: '50%',
                        backgroundColor: cls.color,
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                      }}
                    >
                      <Check size={12} color="#000" />
                    </div>
                  )}
                </div>
              );
            })}
          </div>

          {/* If class unlocks subclass at Level 1 (Cleric, Sorcerer, Warlock), allow selecting here */}
          {subUnlockForSelected === 1 && (
            <div
              style={{
                marginTop: '18px',
                padding: '16px',
                borderRadius: '10px',
                backgroundColor: '#111622',
                border: `1px solid ${currentClassMeta.color}`,
              }}
            >
              <div style={{ fontSize: '14px', fontWeight: 700, color: '#fff', marginBottom: '8px' }}>
                Subclasse de Nível 1 ({currentClassMeta.namePt} escolhe sua origem já no 1º nível pelo SRD 5.1):
              </div>
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(260px, 1fr))', gap: '10px' }}>
                {effectiveSubclasses.map((sub) => {
                  const active = (level1Subclass || effectiveSubclasses[0]?.name) === sub.name;
                  return (
                    <div
                      key={sub.name}
                      onClick={() => setLevel1Subclass(sub.name)}
                      data-testid={`subclass-option-${sub.name}`}
                      style={{
                        padding: '10px 14px',
                        borderRadius: '8px',
                        backgroundColor: active
                          ? sub.isHomebrew
                            ? 'rgba(168, 85, 247, 0.2)'
                            : 'rgba(234, 179, 8, 0.12)'
                          : '#0d1017',
                        border: active
                          ? sub.isHomebrew
                            ? '1px solid #c084fc'
                            : '1px solid var(--accent-gold)'
                          : sub.isHomebrew
                          ? '1px dashed rgba(168, 85, 247, 0.45)'
                          : '1px solid var(--border-color)',
                        cursor: 'pointer',
                        transition: 'all 0.15s ease',
                      }}
                    >
                      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: '6px' }}>
                        <span style={{ fontSize: '13px', fontWeight: 700, color: active ? (sub.isHomebrew ? '#e9d5ff' : 'var(--accent-gold)') : '#fff' }}>
                          {sub.name}
                        </span>
                        {sub.isHomebrew && (
                          <span
                            style={{
                              fontSize: '10px',
                              fontWeight: 800,
                              padding: '2px 6px',
                              borderRadius: '4px',
                              backgroundColor: 'rgba(168, 85, 247, 0.25)',
                              border: '1px solid #c084fc',
                              color: '#d8b4fe',
                              textTransform: 'uppercase',
                              letterSpacing: '0.04em',
                            }}
                          >
                            👑 Override Homebrew
                          </span>
                        )}
                      </div>
                      <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginTop: '4px', lineHeight: 1.4 }}>
                        {sub.description}
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>
          )}

          {/* If class is a spellcaster (Wizard, Sorcerer, Cleric, Druid, Bard, Warlock), display Level 1 Spell selection */}
          {isCasterClass && (
            <div
              style={{
                marginTop: '18px',
                padding: '16px',
                borderRadius: '10px',
                backgroundColor: '#111622',
                border: '1px solid #7c3aed',
              }}
            >
              <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '8px' }}>
                <div style={{ fontSize: '14px', fontWeight: 700, color: '#fff', display: 'flex', alignItems: 'center', gap: '8px' }}>
                  <Sparkles size={16} color="#c084fc" />
                  Grimório Inicial (1º Nível): Escolha as magias de 1º Círculo ({currentClassMeta.namePt})
                </div>
                <div style={{ fontSize: '12px', color: '#a78bfa' }}>
                  {level1SpellsSelected.length} selecionada(s)
                </div>
              </div>
              <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '12px' }}>
                Inclui o repertório oficial SRD 5.1 e magias homebrews/personalizadas criadas dinamicamente para esta premissa.
              </div>

              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))', gap: '10px' }}>
                {getEffectiveSpellsForCircle(1, selectedClass).map((sp) => {
                  const isChecked = level1SpellsSelected.includes(sp.name);
                  return (
                    <div
                      key={sp.name}
                      data-testid={`spell-option-${sp.name}`}
                      onClick={() => {
                        setLevel1SpellsSelected((prev) => {
                          if (prev.includes(sp.name)) {
                            return prev.filter((s) => s !== sp.name);
                          } else {
                            return [...prev, sp.name];
                          }
                        });
                      }}
                      style={{
                        padding: '10px 14px',
                        borderRadius: '8px',
                        backgroundColor: isChecked
                          ? sp.isHomebrew
                            ? 'rgba(168, 85, 247, 0.22)'
                            : 'rgba(59, 130, 246, 0.15)'
                          : '#0d1017',
                        border: isChecked
                          ? sp.isHomebrew
                            ? '1px solid #c084fc'
                            : '1px solid #60a5fa'
                          : sp.isHomebrew
                          ? '1px dashed rgba(168, 85, 247, 0.45)'
                          : '1px solid var(--border-color)',
                        cursor: 'pointer',
                        transition: 'all 0.15s ease',
                      }}
                    >
                      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: '6px' }}>
                        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                          <input
                            type="checkbox"
                            checked={isChecked}
                            onChange={() => {}} // controlled via container div onClick
                            style={{ cursor: 'pointer' }}
                          />
                          <span
                            style={{
                              fontSize: '13px',
                              fontWeight: 700,
                              color: isChecked
                                ? sp.isHomebrew
                                  ? '#f3e8ff'
                                  : '#bfdbfe'
                                : '#e2e8f0',
                            }}
                          >
                            {sp.name}
                          </span>
                        </div>
                        {sp.isHomebrew ? (
                          <span
                            style={{
                              fontSize: '10px',
                              fontWeight: 800,
                              padding: '2px 6px',
                              borderRadius: '4px',
                              backgroundColor: 'rgba(168, 85, 247, 0.25)',
                              border: '1px solid #c084fc',
                              color: '#d8b4fe',
                              textTransform: 'uppercase',
                              letterSpacing: '0.04em',
                            }}
                          >
                            ✨ Magia Homebrew
                          </span>
                        ) : (
                          <span style={{ fontSize: '10px', color: '#64748b', fontWeight: 600 }}>
                            {sp.school}
                          </span>
                        )}
                      </div>
                      <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginTop: '6px', lineHeight: 1.4, paddingLeft: '22px' }}>
                        {sp.description}
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>
          )}

          <div
            style={{
              display: 'flex',
              justifyContent: 'space-between',
              marginTop: '24px',
            }}
          >
            <button
              onClick={() => setStep(1)}
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '8px',
                padding: '10px 18px',
                borderRadius: '8px',
                backgroundColor: 'var(--bg-card)',
                border: '1px solid var(--border-color)',
                color: '#fff',
                cursor: 'pointer',
              }}
            >
              <ChevronLeft size={18} /> Voltar
            </button>
            <button
              onClick={() => setStep(3)}
              data-testid="wizard-step2-next-btn"
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '8px',
                padding: '10px 20px',
                borderRadius: '8px',
                backgroundColor: 'var(--accent-gold)',
                color: '#000',
                fontWeight: 'bold',
                cursor: 'pointer',
              }}
            >
              Distribuir Atributos (SRD 5.1) <ChevronRight size={18} />
            </button>
          </div>
        </div>
      )}

      {/* STEP 3: OFFICIAL SRD 5.1 ABILITY SCORE GENERATION */}
      {step === 3 && (
        <div
          style={{
            backgroundColor: 'var(--bg-card)',
            border: '1px solid var(--border-color)',
            borderRadius: '12px',
            padding: '24px',
          }}
        >
          <div
            style={{
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'flex-start',
              flexWrap: 'wrap',
              gap: '12px',
              marginBottom: '18px',
            }}
          >
            <div>
              <h2 className="rpg-title" style={{ fontSize: '20px', color: '#fff' }}>
                Passo 3: Distribuição de Atributos Iniciais — Regras Oficiais SRD 5.1
              </h2>
              <p style={{ color: 'var(--text-muted)', fontSize: '13px' }}>
                Escolha um dos 3 métodos oficiais do Livro do Jogador (D&amp;D 5e SRD 5.1). Os bônus raciais de{' '}
                <strong style={{ color: 'var(--accent-gold)' }}>{currentRaceMeta.name}</strong> são somados automaticamente ao valor base!
              </p>
            </div>

            {/* Method Selector Tabs */}
            <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap' }}>
              {[
                { id: 'point_buy', label: '1. Compra de Pontos (27 pts)' },
                { id: 'standard_array', label: '2. Matriz Padrão (15,14,13,12,10,8)' },
                { id: 'roll_4d6', label: '3. Rolagem 4d6 (Drop Lowest)' },
              ].map((m) => (
                <button
                  key={m.id}
                  onClick={() => handleSwitchMethod(m.id as AbilityGenerationMethod)}
                  style={{
                    padding: '8px 14px',
                    borderRadius: '8px',
                    fontSize: '12px',
                    fontWeight: 700,
                    backgroundColor:
                      abilityMethod === m.id ? 'var(--accent-gold)' : '#12161f',
                    color: abilityMethod === m.id ? '#000' : '#cbd5e1',
                    border:
                      abilityMethod === m.id
                        ? '1px solid var(--accent-gold)'
                        : '1px solid var(--border-color)',
                    cursor: 'pointer',
                  }}
                >
                  {m.label}
                </button>
              ))}
            </div>
          </div>

          {/* Method Contextual Banner */}
          {abilityMethod === 'point_buy' && (
            <div
              style={{
                marginBottom: '18px',
                padding: '14px 18px',
                borderRadius: '10px',
                backgroundColor: '#101520',
                border: '1px solid #334155',
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                flexWrap: 'wrap',
                gap: '12px',
              }}
            >
              <div>
                <div style={{ fontSize: '13px', fontWeight: 700, color: '#fff' }}>
                  Regra de Compra de Pontos (Point Buy — Orçamento Oficial de 27 Pontos):
                </div>
                <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginTop: '2px' }}>
                  Todos começam em <strong>8</strong> (0 pts). O máximo comprável antes do bônus racial é <strong>15</strong> (9 pts). Tabela SRD: 8=0, 9=1, 10=2, 11=3, 12=4, 13=5, 14=7, 15=9.
                </div>
              </div>

              <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                <div
                  style={{
                    padding: '8px 14px',
                    borderRadius: '8px',
                    backgroundColor:
                      pointBuyRemaining === 0
                        ? 'rgba(16, 185, 129, 0.15)'
                        : 'rgba(234, 179, 8, 0.15)',
                    border:
                      pointBuyRemaining === 0
                        ? '1px solid #10b981'
                        : '1px solid var(--accent-gold)',
                    color: pointBuyRemaining === 0 ? '#10b981' : 'var(--accent-gold)',
                    fontWeight: 800,
                    fontSize: '14px',
                  }}
                >
                  Pontos Restantes: {pointBuyRemaining} / {POINT_BUY_TOTAL_BUDGET} (Gastos: {pointBuySpent})
                </div>

                <button
                  onClick={handleOptimizePointBuyForClass}
                  style={{
                    display: 'inline-flex',
                    alignItems: 'center',
                    gap: '6px',
                    padding: '8px 12px',
                    borderRadius: '6px',
                    backgroundColor: 'var(--accent-arcane-bg)',
                    border: '1px solid var(--accent-arcane)',
                    color: '#c084fc',
                    fontSize: '12px',
                    fontWeight: 700,
                    cursor: 'pointer',
                  }}
                >
                  <Sparkle size={14} /> Sugerir 27/27 para {currentClassMeta.namePt}
                </button>
              </div>
            </div>
          )}

          {abilityMethod === 'standard_array' && (
            <div
              style={{
                marginBottom: '18px',
                padding: '14px 18px',
                borderRadius: '10px',
                backgroundColor: '#101520',
                border: '1px solid #334155',
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                flexWrap: 'wrap',
                gap: '12px',
              }}
            >
              <div>
                <div style={{ fontSize: '13px', fontWeight: 700, color: '#fff' }}>
                  Matriz Padrão Oficial (Standard Array — Soma Fixa de 72 Pontos):
                </div>
                <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginTop: '2px' }}>
                  Distribua os 6 valores fixos <strong>[15, 14, 13, 12, 10, 8]</strong> entre seus 6 atributos. Ao escolher um valor já ocupado, os dois atributos trocam de posição automaticamente.
                </div>
              </div>
              <button
                onClick={() => handleAutoAssignPoolForClass('standard_array')}
                style={{
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '6px',
                  padding: '8px 14px',
                  borderRadius: '6px',
                  backgroundColor: 'var(--accent-arcane-bg)',
                  border: '1px solid var(--accent-arcane)',
                  color: '#c084fc',
                  fontSize: '12px',
                  fontWeight: 700,
                  cursor: 'pointer',
                }}
              >
                <Sparkle size={14} /> Auto-Distribuir Matriz para {currentClassMeta.namePt}
              </button>
            </div>
          )}

          {abilityMethod === 'roll_4d6' && (
            <div
              style={{
                marginBottom: '18px',
                padding: '14px 18px',
                borderRadius: '10px',
                backgroundColor: '#101520',
                border: '1px solid #334155',
                display: 'flex',
                flexDirection: 'column',
                gap: '12px',
              }}
            >
              <div
                style={{
                  display: 'flex',
                  justifyContent: 'space-between',
                  alignItems: 'center',
                  flexWrap: 'wrap',
                  gap: '10px',
                }}
              >
                <div>
                  <div style={{ fontSize: '13px', fontWeight: 700, color: '#fff' }}>
                    Rolagem Oficial de Dados (4d6 Drop Lowest — Soma Atual:{' '}
                    {rolledPool.reduce((acc, r) => acc + r.total, 0)} pts):
                  </div>
                  <div style={{ fontSize: '12px', color: 'var(--text-muted)' }}>
                    Para cada um dos 6 atributos, rola-se 4d6 e descarta-se o menor dado (riscado).
                  </div>
                </div>
                <div style={{ display: 'flex', gap: '8px' }}>
                  <button
                    onClick={handleReroll4d6}
                    style={{
                      display: 'inline-flex',
                      alignItems: 'center',
                      gap: '6px',
                      padding: '8px 14px',
                      borderRadius: '6px',
                      backgroundColor: 'var(--accent-gold)',
                      color: '#000',
                      fontSize: '12px',
                      fontWeight: 800,
                      cursor: 'pointer',
                    }}
                  >
                    <Dices size={15} /> 🎲 Rolar 4d6 Novamente
                  </button>
                </div>
              </div>

              {/* Rolled dice visual cards */}
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(6, 1fr)', gap: '8px' }}>
                {rolledPool.map((roll, idx) => (
                  <div
                    key={idx}
                    style={{
                      backgroundColor: '#0b0e14',
                      border: '1px solid #283142',
                      borderRadius: '8px',
                      padding: '8px',
                      textAlign: 'center',
                    }}
                  >
                    <div style={{ fontSize: '10px', color: 'var(--text-muted)', fontWeight: 700 }}>
                      Conjunto #{idx + 1}
                    </div>
                    <div style={{ fontSize: '18px', fontWeight: 800, color: 'var(--accent-gold)', margin: '2px 0' }}>
                      {roll.total}
                    </div>
                    <div style={{ display: 'flex', justifyContent: 'center', gap: '4px', fontSize: '11px' }}>
                      {roll.dice.map((d, dIdx) => (
                        <span
                          key={dIdx}
                          style={{
                            padding: '1px 4px',
                            borderRadius: '3px',
                            backgroundColor: dIdx === roll.droppedIndex ? '#261b1e' : '#1e293b',
                            color: dIdx === roll.droppedIndex ? '#ef4444' : '#e2e8f0',
                            textDecoration: dIdx === roll.droppedIndex ? 'line-through' : 'none',
                          }}
                        >
                          {d}
                        </span>
                      ))}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fit, minmax(340px, 1fr))',
              gap: '24px',
            }}
          >
            {/* Ability scores adjusters */}
            <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
              {ALL_ABILITIES.map(({ key, label }) => {
                const baseVal = baseAbilities[key];
                const racialBonus = getRacialBonus(key);
                const lvl1Score = level1Abilities[key];
                const lvl1Mod = calculateAbilityModifier(lvl1Score);
                const isPrimary = currentClassMeta.primaryAbilities.includes(key);
                const pbCost = POINT_BUY_COST_TABLE[baseVal] ?? 0;

                const nextCostDelta =
                  baseVal < 15
                    ? (POINT_BUY_COST_TABLE[baseVal + 1] ?? 0) - pbCost
                    : 99;
                const canIncrementPointBuy =
                  baseVal < POINT_BUY_MAX_SCORE && pointBuyRemaining >= nextCostDelta;
                const canDecrementPointBuy = baseVal > POINT_BUY_MIN_SCORE;

                return (
                  <div
                    key={key}
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'space-between',
                      backgroundColor: '#111419',
                      padding: '10px 14px',
                      borderRadius: '8px',
                      border: isPrimary
                        ? '1px solid var(--accent-gold)'
                        : '1px solid var(--border-color)',
                    }}
                  >
                    <div>
                      <div
                        style={{
                          fontWeight: 700,
                          fontSize: '13px',
                          color: isPrimary ? 'var(--accent-gold)' : '#fff',
                        }}
                      >
                        {label} {isPrimary && '★ Primário'}
                      </div>
                      <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginTop: '2px' }}>
                        Base: <strong>{baseVal}</strong>
                        {abilityMethod === 'point_buy' && ` (${pbCost} pts)`}{' '}
                        {racialBonus > 0 && (
                          <span style={{ color: '#38bdf8', fontWeight: 700 }}>
                            + {racialBonus} Raça
                          </span>
                        )}
                      </div>
                    </div>

                    <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                      {abilityMethod === 'point_buy' ? (
                        <>
                          <button
                            disabled={!canDecrementPointBuy}
                            onClick={() => handlePointBuyChange(key, -1)}
                            style={{
                              width: '28px',
                              height: '28px',
                              borderRadius: '4px',
                              backgroundColor: canDecrementPointBuy ? '#222834' : '#161920',
                              color: canDecrementPointBuy ? '#fff' : '#475569',
                              fontWeight: 'bold',
                              cursor: canDecrementPointBuy ? 'pointer' : 'not-allowed',
                            }}
                          >
                            -
                          </button>

                          <div style={{ textAlign: 'center', minWidth: '58px' }}>
                            <span style={{ fontSize: '18px', fontWeight: 'bold', color: '#fff' }}>
                              {lvl1Score}
                            </span>
                            <div
                              style={{
                                fontSize: '12px',
                                fontWeight: 700,
                                color:
                                  lvl1Mod >= 0
                                    ? 'var(--accent-emerald)'
                                    : 'var(--accent-crimson)',
                              }}
                            >
                              {formatModifier(lvl1Mod)}
                            </div>
                          </div>

                          <button
                            disabled={!canIncrementPointBuy}
                            onClick={() => handlePointBuyChange(key, 1)}
                            style={{
                              width: '28px',
                              height: '28px',
                              borderRadius: '4px',
                              backgroundColor: canIncrementPointBuy ? '#222834' : '#161920',
                              color: canIncrementPointBuy ? '#fff' : '#475569',
                              fontWeight: 'bold',
                              cursor: canIncrementPointBuy ? 'pointer' : 'not-allowed',
                            }}
                          >
                            +
                          </button>
                        </>
                      ) : (
                        /* Standard Array or Rolled Pool Slot Selector */
                        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
                          <select
                            value={poolAssignment[key]}
                            onChange={(e) =>
                              handleAssignPoolSlot(key, Number(e.target.value))
                            }
                            style={{
                              padding: '4px 8px',
                              borderRadius: '6px',
                              backgroundColor: '#1b202b',
                              color: '#fff',
                              fontSize: '12px',
                              fontWeight: 700,
                            }}
                          >
                            {(abilityMethod === 'standard_array'
                              ? STANDARD_ARRAY.map((val, i) => ({
                                  label: `Matriz: ${val}`,
                                  idx: i,
                                }))
                              : rolledPool.map((r, i) => ({
                                  label: `Dado #${i + 1}: ${r.total}`,
                                  idx: i,
                                }))
                            ).map((opt) => (
                              <option key={opt.idx} value={opt.idx}>
                                {opt.label}
                              </option>
                            ))}
                          </select>

                          <div style={{ textAlign: 'center', minWidth: '52px' }}>
                            <span style={{ fontSize: '18px', fontWeight: 'bold', color: '#fff' }}>
                              {lvl1Score}
                            </span>
                            <div
                              style={{
                                fontSize: '12px',
                                fontWeight: 700,
                                color:
                                  lvl1Mod >= 0
                                    ? 'var(--accent-emerald)'
                                    : 'var(--accent-crimson)',
                              }}
                            >
                              {formatModifier(lvl1Mod)}
                            </div>
                          </div>
                        </div>
                      )}
                    </div>
                  </div>
                );
              })}
            </div>

            {/* Derived Stats Live Preview (Level 1 vs Target Level) */}
            <div
              style={{
                backgroundColor: '#111419',
                borderRadius: '8px',
                padding: '20px',
                border: '1px solid var(--border-color)',
                display: 'flex',
                flexDirection: 'column',
                justifyContent: 'space-between',
              }}
            >
              <div>
                <h3
                  className="rpg-title"
                  style={{ fontSize: '16px', color: 'var(--accent-gold)', marginBottom: '14px' }}
                >
                  Estatísticas Derivadas (Nível 1 {targetLevel > 1 ? `→ Nível ${targetLevel}` : ''})
                </h3>

                <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '12px' }}>
                  {/* HP */}
                  <div
                    style={{
                      backgroundColor: 'rgba(239, 68, 68, 0.1)',
                      border: '1px solid rgba(239, 68, 68, 0.3)',
                      borderRadius: '8px',
                      padding: '12px',
                      textAlign: 'center',
                    }}
                  >
                    <div
                      style={{
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        gap: '6px',
                        color: 'var(--accent-crimson)',
                        fontSize: '12px',
                        fontWeight: 600,
                        marginBottom: '4px',
                      }}
                    >
                      <Heart size={16} /> PV NO NÍVEL 1
                    </div>
                    <span style={{ fontSize: '24px', fontWeight: 'bold', color: '#fff' }}>
                      {getCumulativeHpAtLevel(1)}
                    </span>
                    <div style={{ fontSize: '11px', color: 'var(--text-muted)' }}>
                      {currentClassMeta.hitDieValue} ({currentClassMeta.hitDie}) +{' '}
                      {calculateAbilityModifier(level1Abilities.constitution)} CON
                      {currentRaceMeta.hpBonusPerLevel ? ' + 1 Anão' : ''}
                    </div>
                  </div>

                  {/* AC */}
                  <div
                    style={{
                      backgroundColor: 'rgba(2, 132, 199, 0.1)',
                      border: '1px solid rgba(2, 132, 199, 0.3)',
                      borderRadius: '8px',
                      padding: '12px',
                      textAlign: 'center',
                    }}
                  >
                    <div
                      style={{
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        gap: '6px',
                        color: 'var(--accent-sky)',
                        fontSize: '12px',
                        fontWeight: 600,
                        marginBottom: '4px',
                      }}
                    >
                      <Shield size={16} /> CLASSE DE ARMADURA (CA)
                    </div>
                    <span style={{ fontSize: '24px', fontWeight: 'bold', color: '#fff' }}>
                      {calculatedAc}
                    </span>
                    <div style={{ fontSize: '11px', color: 'var(--text-muted)' }}>
                      {selectedClass === 'Barbarian'
                        ? '10 + DES + CON (Bárbaro)'
                        : selectedClass === 'Monk'
                        ? '10 + DES + SAB (Monge)'
                        : 'Base Sem Armadura'}
                    </div>
                  </div>
                </div>

                {/* Breakdown Table of Base + Racial */}
                <div
                  style={{
                    marginTop: '16px',
                    padding: '12px',
                    borderRadius: '8px',
                    backgroundColor: '#161a23',
                    border: '1px solid #262e3d',
                  }}
                >
                  <div style={{ fontSize: '12px', fontWeight: 700, color: '#fff', marginBottom: '8px' }}>
                    Resumo Matemático do Nível 1 (Base + Bônus Racial):
                  </div>
                  <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '6px', fontSize: '11px' }}>
                    {ALL_ABILITIES.map((a) => (
                      <div
                        key={a.key}
                        style={{
                          padding: '6px',
                          backgroundColor: '#0e1117',
                          borderRadius: '4px',
                          color: 'var(--text-muted)',
                        }}
                      >
                        <strong style={{ color: '#fff' }}>{a.short}:</strong> {baseAbilities[a.key]} +{' '}
                        {getRacialBonus(a.key)} ={' '}
                        <strong style={{ color: 'var(--accent-gold)' }}>
                          {level1Abilities[a.key]}
                        </strong>
                      </div>
                    ))}
                  </div>
                </div>
              </div>

              {targetLevel > 1 && (
                <div
                  style={{
                    marginTop: '14px',
                    padding: '12px',
                    borderRadius: '8px',
                    backgroundColor: 'rgba(234, 179, 8, 0.08)',
                    border: '1px solid rgba(234, 179, 8, 0.35)',
                    fontSize: '12px',
                    color: 'var(--accent-gold)',
                  }}
                >
                  ⚡ Como você escolheu iniciar no <strong>Nível {targetLevel}</strong>, o próximo passo permitirá evoluir seu personagem nível por nível (do 2 ao {targetLevel}) segundo o SRD 5.1!
                </div>
              )}
            </div>
          </div>

          <div
            style={{
              display: 'flex',
              justifyContent: 'space-between',
              marginTop: '24px',
            }}
          >
            <button
              onClick={() => setStep(2)}
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '8px',
                padding: '10px 18px',
                borderRadius: '8px',
                backgroundColor: 'var(--bg-card)',
                border: '1px solid var(--border-color)',
                color: '#fff',
                cursor: 'pointer',
              }}
            >
              <ChevronLeft size={18} /> Voltar
            </button>
            <button
              onClick={() => {
                if (targetLevel > 1) {
                  setActiveEvolutionLevel(2);
                  setStep(4);
                } else {
                  setStep(5);
                }
              }}
              data-testid="wizard-step3-next-btn"
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '8px',
                padding: '10px 20px',
                borderRadius: '8px',
                backgroundColor: 'var(--accent-gold)',
                color: '#000',
                fontWeight: 'bold',
                cursor: 'pointer',
              }}
            >
              {targetLevel > 1
                ? `Evoluir Nível a Nível (Nível 2 → ${targetLevel})`
                : 'Revisar Ficha Final'}{' '}
              <ChevronRight size={18} />
            </button>
          </div>
        </div>
      )}

      {/* STEP 4: LEVEL-BY-LEVEL EVOLUTION (LEVELS 2 TO TARGET_LEVEL) */}
      {step === 4 && targetLevel > 1 && (
        <div
          style={{
            backgroundColor: 'var(--bg-card)',
            border: '1px solid var(--border-color)',
            borderRadius: '12px',
            padding: '24px',
          }}
        >
          <div
            style={{
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
              flexWrap: 'wrap',
              gap: '12px',
              marginBottom: '16px',
            }}
          >
            <div>
              <h2 className="rpg-title" style={{ fontSize: '20px', color: '#fff' }}>
                Passo 4: Evolução Nível a Nível (SRD 5.1) — {currentClassMeta.namePt} (Nível 1 → {targetLevel})
              </h2>
              <p style={{ color: 'var(--text-muted)', fontSize: '13px' }}>
                Avance nível por nível escolhendo como ganhar PV (Média Fixa ou Rolagem de {currentClassMeta.hitDie}), aplicando seus ASIs (+2 ou +1/+1) e desbloqueando habilidades.
              </p>
            </div>

            <button
              onClick={handleAutoRollAllLevels}
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '6px',
                padding: '8px 14px',
                borderRadius: '6px',
                backgroundColor: 'var(--accent-arcane-bg)',
                border: '1px solid var(--accent-arcane)',
                color: '#c084fc',
                fontSize: '12px',
                fontWeight: 700,
                cursor: 'pointer',
              }}
            >
              <Sparkle size={14} /> ⚡ Auto-Preencher Todos os Níveis (Padrão SRD)
            </button>
          </div>

          {/* Interactive Level Timeline Bar (2..targetLevel) */}
          <div
            style={{
              display: 'flex',
              gap: '6px',
              flexWrap: 'wrap',
              padding: '12px',
              borderRadius: '10px',
              backgroundColor: '#0e1218',
              border: '1px solid #262f3f',
              marginBottom: '20px',
            }}
          >
            <div
              style={{
                padding: '6px 12px',
                borderRadius: '6px',
                fontSize: '12px',
                fontWeight: 700,
                backgroundColor: '#1e293b',
                color: '#10b981',
                border: '1px solid #10b98155',
              }}
            >
              Nível 1 Base ✓
            </div>

            {Array.from({ length: targetLevel - 1 }, (_, idx) => idx + 2).map((lvl) => {
              const isCurrent = activeEvolutionLevel === lvl;
              const hasAsi = isAsiLevel(selectedClass, lvl);
              const hasSub = subUnlockForSelected === lvl;
              return (
                <button
                  key={lvl}
                  onClick={() => setActiveEvolutionLevel(lvl)}
                  style={{
                    padding: '6px 12px',
                    borderRadius: '6px',
                    fontSize: '12px',
                    fontWeight: 700,
                    backgroundColor: isCurrent
                      ? 'var(--accent-gold)'
                      : lvl < activeEvolutionLevel
                      ? 'rgba(16, 185, 129, 0.14)'
                      : '#161b24',
                    color: isCurrent
                      ? '#000'
                      : lvl < activeEvolutionLevel
                      ? '#34d399'
                      : '#94a3b8',
                    border: isCurrent
                      ? '1px solid var(--accent-gold)'
                      : hasAsi || hasSub
                      ? '1px solid rgba(234, 179, 8, 0.45)'
                      : '1px solid #28303e',
                    cursor: 'pointer',
                  }}
                >
                  Nível {lvl} {hasAsi ? '★ ASI' : hasSub ? '★ Sub' : ''}
                </button>
              );
            })}
          </div>

          {/* Active Level Detail Editor + Cumulative Preview */}
          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'minmax(0, 1.35fr) minmax(290px, 0.85fr)',
              gap: '20px',
              alignItems: 'start',
            }}
          >
            {/* Left: Decisions for `activeEvolutionLevel` */}
            <div
              style={{
                backgroundColor: '#11151e',
                border: '1px solid #2d3748',
                borderRadius: '10px',
                padding: '20px',
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
                  borderBottom: '1px solid #262f3f',
                  paddingBottom: '12px',
                }}
              >
                <div>
                  <span
                    style={{
                      fontSize: '11px',
                      fontWeight: 800,
                      padding: '2px 8px',
                      borderRadius: '4px',
                      backgroundColor: 'var(--accent-gold)',
                      color: '#000',
                      marginRight: '8px',
                    }}
                  >
                    EVOLUINDO NÍVEL {activeEvolutionLevel} DE {targetLevel}
                  </span>
                  <span className="rpg-title" style={{ fontSize: '18px', color: '#fff', fontWeight: 700 }}>
                    {currentClassMeta.namePt} — Nível {activeEvolutionLevel}
                  </span>
                </div>
                <span style={{ fontSize: '12px', color: 'var(--accent-gold)', fontWeight: 700 }}>
                  Proficiência no Nível {activeEvolutionLevel}: +{getProficiencyBonusForLevel(activeEvolutionLevel)}
                </span>
              </div>

              {/* 1. Hit Points Gain for this level */}
              <div
                style={{
                  padding: '14px',
                  borderRadius: '8px',
                  backgroundColor: '#0c0f15',
                  border: '1px solid #252f3f',
                }}
              >
                <div style={{ fontSize: '13px', fontWeight: 700, color: 'var(--accent-crimson)', marginBottom: '8px', display: 'flex', alignItems: 'center', gap: '6px' }}>
                  <Heart size={15} /> 1. Aumento de Pontos de Vida no Nível {activeEvolutionLevel} (Dado {currentClassMeta.hitDie}):
                </div>
                <div style={{ display: 'flex', gap: '10px', flexWrap: 'wrap', alignItems: 'center' }}>
                  <button
                    onClick={() => handleFixedHpForLevel(activeEvolutionLevel)}
                    style={{
                      padding: '8px 14px',
                      borderRadius: '6px',
                      fontSize: '12px',
                      fontWeight: 700,
                      backgroundColor:
                        activeEvoChoice?.hpMode === 'fixed_average'
                          ? '#047857'
                          : '#1a202c',
                      color: '#fff',
                      border:
                        activeEvoChoice?.hpMode === 'fixed_average'
                          ? '1px solid #10b981'
                          : '1px solid #334155',
                      cursor: 'pointer',
                    }}
                  >
                    ✓ Usar Média Fixa SRD (+{getFixedHpAverageForHitDie(currentClassMeta.hitDieValue)} no {currentClassMeta.hitDie})
                  </button>

                  <button
                    onClick={() => handleRollHpForLevel(activeEvolutionLevel)}
                    style={{
                      padding: '8px 14px',
                      borderRadius: '6px',
                      fontSize: '12px',
                      fontWeight: 700,
                      backgroundColor:
                        activeEvoChoice?.hpMode === 'rolled' ? '#b45309' : '#1a202c',
                      color: '#fff',
                      border:
                        activeEvoChoice?.hpMode === 'rolled'
                          ? '1px solid #f59e0b'
                          : '1px solid #334155',
                      cursor: 'pointer',
                      display: 'inline-flex',
                      alignItems: 'center',
                      gap: '6px',
                    }}
                  >
                    <Dices size={14} /> 🎲 Rolar 1{currentClassMeta.hitDie} Agora
                  </button>

                  <span style={{ fontSize: '12px', color: '#e2e8f0', marginLeft: '6px' }}>
                    Resultado do Dado: <strong style={{ color: 'var(--accent-gold)' }}>{activeEvoChoice?.hpDieRoll}</strong> + Mod. CON ({formatModifier(finalConMod)}) ={' '}
                    <strong style={{ color: '#10b981' }}>
                      +{Math.max(1, (activeEvoChoice?.hpDieRoll || 5) + finalConMod) + (currentRaceMeta.hpBonusPerLevel || 0)} PV neste nível
                    </strong>
                  </span>
                </div>
              </div>

              {/* 2. Unlocked Class Features at this level */}
              <div
                style={{
                  padding: '14px',
                  borderRadius: '8px',
                  backgroundColor: '#0c0f15',
                  border: '1px solid #252f3f',
                }}
              >
                <div style={{ fontSize: '13px', fontWeight: 700, color: 'var(--accent-gold)', marginBottom: '8px', display: 'flex', alignItems: 'center', gap: '6px' }}>
                  <Award size={15} /> 2. Características Desbloqueadas no Nível {activeEvolutionLevel} (SRD 5.1):
                </div>
                <ul style={{ margin: 0, paddingLeft: '18px', color: '#e2e8f0', fontSize: '13px', lineHeight: '1.6' }}>
                  {getClassFeaturesAtLevel(selectedClass, activeEvolutionLevel).map((feat, i) => (
                    <li key={i}>{feat}</li>
                  ))}
                </ul>
              </div>

              {/* 3. Subclass Selection if unlocked at this level */}
              {subUnlockForSelected === activeEvolutionLevel && (
                <div
                  style={{
                    padding: '14px',
                    borderRadius: '8px',
                    backgroundColor: 'rgba(234, 179, 8, 0.07)',
                    border: '1px solid var(--accent-gold)',
                  }}
                >
                  <div style={{ fontSize: '13px', fontWeight: 700, color: 'var(--accent-gold)', marginBottom: '8px' }}>
                    ★ Escolha de Arquétipo / Subclasse Oficial (Nível {activeEvolutionLevel}):
                  </div>
                  <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                    {effectiveSubclasses.map((sub) => {
                      const isSel = activeEvoChoice?.subclassSelected === sub.name;
                      return (
                        <div
                          key={sub.name}
                          onClick={() =>
                            updateLevelChoice(activeEvolutionLevel, {
                              subclassSelected: sub.name,
                            })
                          }
                          style={{
                            padding: '10px 12px',
                            borderRadius: '6px',
                            backgroundColor: isSel
                              ? sub.isHomebrew
                                ? 'rgba(168, 85, 247, 0.22)'
                                : 'rgba(234, 179, 8, 0.16)'
                              : '#0e1218',
                            border: isSel
                              ? sub.isHomebrew
                                ? '1px solid #c084fc'
                                : '1px solid var(--accent-gold)'
                              : sub.isHomebrew
                              ? '1px dashed rgba(168, 85, 247, 0.45)'
                              : '1px solid #28303e',
                            cursor: 'pointer',
                          }}
                        >
                          <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: '6px' }}>
                            <div style={{ fontSize: '13px', fontWeight: 700, color: isSel ? (sub.isHomebrew ? '#e9d5ff' : 'var(--accent-gold)') : '#fff' }}>
                              {sub.name} {isSel && '✓'}
                            </div>
                            {sub.isHomebrew && (
                              <span
                                style={{
                                  fontSize: '10px',
                                  fontWeight: 800,
                                  padding: '2px 6px',
                                  borderRadius: '4px',
                                  backgroundColor: 'rgba(168, 85, 247, 0.25)',
                                  border: '1px solid #c084fc',
                                  color: '#d8b4fe',
                                  textTransform: 'uppercase',
                                  letterSpacing: '0.04em',
                                }}
                              >
                                👑 Override Homebrew
                              </span>
                            )}
                          </div>
                          <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginTop: '2px', lineHeight: 1.4 }}>
                            {sub.description}
                          </div>
                        </div>
                      );
                    })}
                  </div>
                </div>
              )}

              {/* 4. Ability Score Improvement (ASI) if this is an ASI level */}
              {isAsiLevel(selectedClass, activeEvolutionLevel) && (
                <div
                  style={{
                    padding: '14px',
                    borderRadius: '8px',
                    backgroundColor: 'rgba(59, 130, 246, 0.08)',
                    border: '1px solid #3b82f6',
                  }}
                >
                  <div style={{ fontSize: '13px', fontWeight: 700, color: '#93c5fd', marginBottom: '8px' }}>
                    ★ Incremento no Valor de Habilidade (ASI — Nível {activeEvolutionLevel} SRD 5.1, Máx 20):
                  </div>

                  <div style={{ display: 'flex', gap: '10px', marginBottom: '12px' }}>
                    <button
                      onClick={() =>
                        updateLevelChoice(activeEvolutionLevel, { asiMode: 'plus_two_one' })
                      }
                      style={{
                        padding: '6px 12px',
                        borderRadius: '6px',
                        fontSize: '12px',
                        fontWeight: 700,
                        backgroundColor:
                          activeEvoChoice?.asiMode === 'plus_two_one' ? '#2563eb' : '#161b26',
                        color: '#fff',
                        border: '1px solid #3b82f6',
                        cursor: 'pointer',
                      }}
                    >
                      +2 em 1 Atributo
                    </button>
                    <button
                      onClick={() =>
                        updateLevelChoice(activeEvolutionLevel, { asiMode: 'plus_one_two' })
                      }
                      style={{
                        padding: '6px 12px',
                        borderRadius: '6px',
                        fontSize: '12px',
                        fontWeight: 700,
                        backgroundColor:
                          activeEvoChoice?.asiMode === 'plus_one_two' ? '#2563eb' : '#161b26',
                        color: '#fff',
                        border: '1px solid #3b82f6',
                        cursor: 'pointer',
                      }}
                    >
                      +1 em 2 Atributos
                    </button>
                  </div>

                  <div style={{ display: 'flex', gap: '14px', flexWrap: 'wrap' }}>
                    <div>
                      <span style={{ fontSize: '12px', color: 'var(--text-muted)', marginRight: '8px' }}>
                        {activeEvoChoice?.asiMode === 'plus_two_one'
                          ? 'Atributo (+2):'
                          : '1º Atributo (+1):'}
                      </span>
                      <select
                        value={activeEvoChoice?.asiPrimaryStat || 'strength'}
                        onChange={(e) =>
                          updateLevelChoice(activeEvolutionLevel, {
                            asiPrimaryStat: e.target.value as AbilityName,
                          })
                        }
                      >
                        {ALL_ABILITIES.map((a) => (
                          <option key={a.key} value={a.key}>
                            {a.label}
                          </option>
                        ))}
                      </select>
                    </div>

                    {activeEvoChoice?.asiMode === 'plus_one_two' && (
                      <div>
                        <span style={{ fontSize: '12px', color: 'var(--text-muted)', marginRight: '8px' }}>
                          2º Atributo (+1):
                        </span>
                        <select
                          value={activeEvoChoice?.asiSecondaryStat || 'constitution'}
                          onChange={(e) =>
                            updateLevelChoice(activeEvolutionLevel, {
                              asiSecondaryStat: e.target.value as AbilityName,
                            })
                          }
                        >
                          {ALL_ABILITIES.filter(
                            (a) => a.key !== activeEvoChoice?.asiPrimaryStat
                          ).map((a) => (
                            <option key={a.key} value={a.key}>
                              {a.label}
                            </option>
                          ))}
                        </select>
                      </div>
                    )}
                  </div>
                </div>
              )}

              {/* 5. Spell Selection for Spellcasters at this level */}
              {currentClassMeta.isSpellcaster && (
                <div
                  style={{
                    padding: '14px',
                    borderRadius: '8px',
                    backgroundColor: '#0c0f15',
                    border: '1px solid #252f3f',
                  }}
                >
                  <div style={{ fontSize: '13px', fontWeight: 700, color: '#c084fc', marginBottom: '8px', display: 'flex', alignItems: 'center', gap: '6px' }}>
                    <Sparkles size={15} /> Magia Adicional Aprendida no Nível {activeEvolutionLevel} (Até {unlockedCircleAtActiveLvl}º Círculo):
                  </div>
                  <select
                    value={activeEvoChoice?.newSpellAdded || ''}
                    onChange={(e) =>
                      updateLevelChoice(activeEvolutionLevel, {
                        newSpellAdded: e.target.value,
                      })
                    }
                    style={{ width: '100%' }}
                  >
                    {Array.from({ length: unlockedCircleAtActiveLvl }, (_, i) => i + 1).flatMap(
                      (circle) =>
                        getEffectiveSpellsForCircle(circle, selectedClass).map((sp) => (
                          <option key={sp.name} value={sp.name}>
                            {sp.isHomebrew ? `✨ [HOMEBREW] ${sp.name}` : sp.name}
                          </option>
                        ))
                    )}
                  </select>
                </div>
              )}

              {/* Step-by-step level navigation buttons */}
              <div style={{ display: 'flex', justifyContent: 'space-between', marginTop: '8px' }}>
                <button
                  disabled={activeEvolutionLevel <= 2}
                  onClick={() => setActiveEvolutionLevel((l) => Math.max(2, l - 1))}
                  style={{
                    padding: '8px 14px',
                    borderRadius: '6px',
                    backgroundColor: '#1b202b',
                    color: activeEvolutionLevel <= 2 ? '#475569' : '#fff',
                    border: '1px solid #334155',
                    fontSize: '12px',
                    fontWeight: 700,
                    cursor: activeEvolutionLevel <= 2 ? 'not-allowed' : 'pointer',
                  }}
                >
                  ⬅ Nível {activeEvolutionLevel - 1}
                </button>

                {activeEvolutionLevel < targetLevel ? (
                  <button
                    onClick={() =>
                      setActiveEvolutionLevel((l) => Math.min(targetLevel, l + 1))
                    }
                    data-testid="wizard-step4-next-btn"
                    style={{
                      padding: '8px 18px',
                      borderRadius: '6px',
                      backgroundColor: 'var(--accent-gold)',
                      color: '#000',
                      fontSize: '12px',
                      fontWeight: 800,
                      cursor: 'pointer',
                    }}
                  >
                    Confirmar Nível {activeEvolutionLevel} &amp; Evoluir para Nível {activeEvolutionLevel + 1} ➡
                  </button>
                ) : (
                  <button
                    onClick={() => setStep(5)}
                    data-testid="wizard-step4-finish-btn"
                    style={{
                      padding: '8px 18px',
                      borderRadius: '6px',
                      backgroundColor: '#10b981',
                      color: '#000',
                      fontSize: '12px',
                      fontWeight: 800,
                      cursor: 'pointer',
                    }}
                  >
                    ✓ Concluir Evolução no Nível {targetLevel} &amp; Ir para Revisão
                  </button>
                )}
              </div>
            </div>

            {/* Right: Live Cumulative Character Status at `activeEvolutionLevel` / `targetLevel` */}
            <div
              style={{
                backgroundColor: '#111419',
                border: '1px solid var(--border-color)',
                borderRadius: '10px',
                padding: '18px',
                display: 'flex',
                flexDirection: 'column',
                gap: '14px',
              }}
            >
              <div style={{ fontSize: '14px', fontWeight: 700, color: 'var(--accent-gold)' }}>
                Ficha Acumulada no Nível {targetLevel}
              </div>

              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '8px' }}>
                <div style={{ backgroundColor: '#161b24', padding: '10px', borderRadius: '8px', textAlign: 'center' }}>
                  <div style={{ fontSize: '10px', color: 'var(--text-muted)' }}>PV MÁXIMO (NÍVEL {targetLevel})</div>
                  <div style={{ fontSize: '22px', fontWeight: 800, color: 'var(--accent-crimson)' }}>
                    {calculatedHp}
                  </div>
                </div>
                <div style={{ backgroundColor: '#161b24', padding: '10px', borderRadius: '8px', textAlign: 'center' }}>
                  <div style={{ fontSize: '10px', color: 'var(--text-muted)' }}>PROFICIÊNCIA</div>
                  <div style={{ fontSize: '22px', fontWeight: 800, color: 'var(--accent-gold)' }}>
                    +{finalProficiency}
                  </div>
                </div>
              </div>

              <div>
                <div style={{ fontSize: '11px', fontWeight: 700, color: 'var(--text-muted)', marginBottom: '6px' }}>
                  ATRIBUTOS FINAIS (COM RAÇA + ASIs ATÉ NÍVEL {targetLevel}):
                </div>
                <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '6px' }}>
                  {ALL_ABILITIES.map((a) => {
                    const val = finalAbilities[a.key];
                    const mod = calculateAbilityModifier(val);
                    return (
                      <div
                        key={a.key}
                        style={{
                          backgroundColor: '#0d1016',
                          border: '1px solid #232a38',
                          borderRadius: '6px',
                          padding: '6px',
                          textAlign: 'center',
                        }}
                      >
                        <div style={{ fontSize: '10px', color: 'var(--text-muted)', fontWeight: 700 }}>
                          {a.short}
                        </div>
                        <div style={{ fontSize: '16px', fontWeight: 800, color: '#fff' }}>{val}</div>
                        <div style={{ fontSize: '11px', color: 'var(--accent-emerald)', fontWeight: 700 }}>
                          {formatModifier(mod)}
                        </div>
                      </div>
                    );
                  })}
                </div>
              </div>

              {/* Spell Slots Preview at Target Level */}
              {currentClassMeta.isSpellcaster && (
                <div>
                  <div style={{ fontSize: '11px', fontWeight: 700, color: '#c084fc', marginBottom: '6px' }}>
                    ESPAÇOS DE MAGIA NO NÍVEL {targetLevel} (SRD 5.1):
                  </div>
                  <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px' }}>
                    {(() => {
                      const slots = getSpellSlotsForClassAndLevel(selectedClass, targetLevel);
                      if (slots.pactMagic) {
                        return (
                          <span style={{ fontSize: '12px', color: '#e2e8f0' }}>
                            Magia de Pacto: {slots.pactMagic.total} espaços de {slots.pactMagic.slotLevel}º Círculo
                          </span>
                        );
                      }
                      return Object.values(slots.levels).map((s) => (
                        <span
                          key={s.level}
                          style={{
                            fontSize: '11px',
                            padding: '3px 8px',
                            borderRadius: '4px',
                            backgroundColor: '#1e1b38',
                            border: '1px solid #6b4096',
                            color: '#e9d5ff',
                            fontWeight: 700,
                          }}
                        >
                          {s.level}º Círculo: {s.total}
                        </span>
                      ));
                    })()}
                  </div>
                </div>
              )}
            </div>
          </div>

          <div style={{ display: 'flex', justifyContent: 'space-between', marginTop: '24px' }}>
            <button
              onClick={() => setStep(3)}
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '8px',
                padding: '10px 18px',
                borderRadius: '8px',
                backgroundColor: 'var(--bg-card)',
                border: '1px solid var(--border-color)',
                color: '#fff',
                cursor: 'pointer',
              }}
            >
              <ChevronLeft size={18} /> Voltar aos Atributos Base
            </button>
            <button
              onClick={() => setStep(5)}
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '8px',
                padding: '10px 20px',
                borderRadius: '8px',
                backgroundColor: 'var(--accent-gold)',
                color: '#000',
                fontWeight: 'bold',
                cursor: 'pointer',
              }}
            >
              Revisar Ficha Completa (Nível {targetLevel}) <ChevronRight size={18} />
            </button>
          </div>
        </div>
      )}

      {/* STEP 5: SUMMARY & ENTER GAME */}
      {step === 5 && (
        <div
          style={{
            backgroundColor: 'var(--bg-card)',
            border: '1px solid var(--border-color)',
            borderRadius: '12px',
            padding: '24px',
          }}
        >
          <h2
            className="rpg-title"
            style={{ fontSize: '20px', color: '#fff', marginBottom: '16px' }}
          >
            Revisão Final do Herói (Nível {targetLevel}) e Entrada no Mundo
          </h2>

          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))',
              gap: '16px',
              marginBottom: '20px',
            }}
          >
            <div
              style={{
                backgroundColor: '#111419',
                padding: '16px',
                borderRadius: '8px',
                border: '1px solid var(--border-color)',
              }}
            >
              <div style={{ color: 'var(--accent-gold)', fontWeight: 'bold', fontSize: '18px' }}>
                {name}
              </div>
              <div style={{ color: '#fff', fontSize: '14px', fontWeight: 600, marginTop: '2px' }}>
                {currentRaceMeta.name} — {currentClassMeta.namePt} Nível {targetLevel}
                {chosenSubclass ? ` (${chosenSubclass})` : ''}
              </div>
              <div style={{ color: 'var(--text-faint)', fontSize: '12px', marginTop: '4px' }}>
                Antecedente: {background} | {alignment} | Ataques por Ação: {getAttacksPerAction()}x
              </div>
            </div>

            <div
              style={{
                backgroundColor: '#111419',
                padding: '16px',
                borderRadius: '8px',
                border: '1px solid var(--border-color)',
                display: 'flex',
                justifyContent: 'space-around',
                alignItems: 'center',
              }}
            >
              <div style={{ textAlign: 'center' }}>
                <span style={{ fontSize: '22px', fontWeight: 'bold', color: 'var(--accent-crimson)' }}>
                  {calculatedHp}
                </span>
                <div style={{ fontSize: '11px', color: 'var(--text-muted)' }}>
                  HP Total ({targetLevel}{currentClassMeta.hitDie})
                </div>
              </div>

              <div style={{ textAlign: 'center' }}>
                <span style={{ fontSize: '22px', fontWeight: 'bold', color: 'var(--accent-sky)' }}>
                  {calculatedAc}
                </span>
                <div style={{ fontSize: '11px', color: 'var(--text-muted)' }}>CA</div>
              </div>

              <div style={{ textAlign: 'center' }}>
                <span style={{ fontSize: '22px', fontWeight: 'bold', color: 'var(--accent-emerald)' }}>
                  {formatModifier(finalDexMod)}
                </span>
                <div style={{ fontSize: '11px', color: 'var(--text-muted)' }}>Iniciativa</div>
              </div>

              <div style={{ textAlign: 'center' }}>
                <span style={{ fontSize: '22px', fontWeight: 'bold', color: 'var(--accent-gold)' }}>
                  +{finalProficiency}
                </span>
                <div style={{ fontSize: '11px', color: 'var(--text-muted)' }}>Proficiência</div>
              </div>
            </div>
          </div>

          {/* All Unlocked Features List */}
          <div
            style={{
              backgroundColor: '#111419',
              padding: '16px',
              borderRadius: '8px',
              border: '1px solid var(--border-color)',
              marginBottom: '20px',
              maxHeight: '220px',
              overflowY: 'auto',
            }}
          >
            <div style={{ fontSize: '13px', fontWeight: 700, color: 'var(--accent-gold)', marginBottom: '8px' }}>
              Características de Raça &amp; Classe Desbloqueadas (Nível 1 ao {targetLevel} — SRD 5.1):
            </div>
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px' }}>
              {getAllUnlockedFeatures().map((feat, idx) => (
                <span
                  key={idx}
                  style={{
                    fontSize: '11px',
                    padding: '4px 10px',
                    borderRadius: '6px',
                    backgroundColor: '#181e29',
                    border: '1px solid #2b3548',
                    color: '#cbd5e1',
                  }}
                >
                  {feat}
                </span>
              ))}
            </div>
          </div>

          <div style={{ display: 'flex', justifyContent: 'space-between' }}>
            <button
              onClick={() => setStep(targetLevel > 1 ? 4 : 3)}
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '8px',
                padding: '10px 18px',
                borderRadius: '8px',
                backgroundColor: 'var(--bg-card)',
                border: '1px solid var(--border-color)',
                color: '#fff',
                cursor: 'pointer',
              }}
            >
              <ChevronLeft size={18} /> Voltar
            </button>

            <button
              onClick={handleFinish}
              data-testid="wizard-finish-btn"
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '10px',
                padding: '12px 28px',
                borderRadius: '8px',
                backgroundColor: 'var(--accent-gold)',
                color: '#000',
                fontWeight: 'bold',
                fontSize: '15px',
                boxShadow: '0 0 20px rgba(234, 179, 8, 0.4)',
                cursor: 'pointer',
              }}
            >
              <Sword size={18} />{' '}
              {isEditing
                ? `Salvar Alterações de ${name} (Nível ${targetLevel})`
                : isAddingToParty
                ? `Adicionar ${name} (Nível ${targetLevel}) à Party & Ir ao Jogo`
                : `Entrar na Campanha com ${name} (Nível ${targetLevel})`}
            </button>
          </div>
        </div>
      )}
    </div>
  );
};
