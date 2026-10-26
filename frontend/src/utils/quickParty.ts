import type { CharacterSheet, CharacterClassType } from '../types/character';

export function createQuickTestParty(): CharacterSheet[] {
  const ts = Date.now();

  const barbarian: CharacterSheet = {
    id: `char-barbarian-${ts}`,
    name: 'Valério, o Brutal',
    race: 'Humano',
    className: 'Barbarian',
    subclass: '',
    level: 1,
    background: 'Soldado',
    alignment: 'Caótico e Bom',
    proficiencyBonus: 2,
    abilities: {
      strength: 16,
      dexterity: 14,
      constitution: 16,
      intelligence: 10,
      wisdom: 12,
      charisma: 8,
    },
    savingThrowProficiencies: ['strength', 'constitution'],
    skillProficiencies: ['Atletismo', 'Intimidação', 'Sobrevivência'],
    vitals: {
      hpCurrent: 15,
      hpMax: 15,
      hpTemp: 0,
      armorClass: 15,
      initiativeModifier: 2,
      speedFeet: 30,
      hitDiceTotal: 1,
      hitDiceCurrent: 1,
      hitDiceType: 'd12',
      deathSaves: { successes: 0, failures: 0 },
      conditions: [],
    },
    spellSlots: { levels: {} },
    spellsKnownOrPrepared: [],
    classResources: {
      barbarian: {
        rageUsesTotal: 2,
        rageUsesCurrent: 2,
        isRaging: false,
        rageDamageBonus: 2,
        unarmoredDefenseFormula: '10 + Mod. DES + Mod. CON',
      },
    },
    features: ['Fúria Primeva', 'Defesa sem Armadura'],
    attacksPerAction: 1,
    inventory: [
      { name: 'Machado Grande (1d12 cortante)', quantity: 1, weightLbs: 7 },
      { name: 'Javelinas (1d6 perfurante)', quantity: 4, weightLbs: 8 },
    ],
  };

  const cleric: CharacterSheet = {
    id: `char-cleric-${ts}`,
    name: 'Elysia da Alvorada',
    race: 'Elfo',
    className: 'Cleric',
    subclass: 'Domínio da Vida',
    level: 1,
    background: 'Acólito',
    alignment: 'Leal e Bom',
    proficiencyBonus: 2,
    abilities: {
      strength: 14,
      dexterity: 10,
      constitution: 14,
      intelligence: 10,
      wisdom: 16,
      charisma: 12,
    },
    savingThrowProficiencies: ['wisdom', 'charisma'],
    skillProficiencies: ['Medicina', 'Religião', 'Persuasão'],
    vitals: {
      hpCurrent: 10,
      hpMax: 10,
      hpTemp: 0,
      armorClass: 18,
      initiativeModifier: 0,
      speedFeet: 30,
      hitDiceTotal: 1,
      hitDiceCurrent: 1,
      hitDiceType: 'd8',
      deathSaves: { successes: 0, failures: 0 },
      conditions: [],
    },
    spellSlots: {
      levels: {
        1: { level: 1, total: 2, used: 0 },
      },
    },
    spellsKnownOrPrepared: ['Chama Sagrada', 'Bênção', 'Curar Ferimentos'],
    classResources: {
      cleric: {
        channelDivinityTotal: 0,
        channelDivinityCurrent: 0,
        divineDomain: 'Domínio da Vida',
        domainSpellsAlwaysPrepared: ['Bênção', 'Curar Ferimentos'],
      },
    },
    features: ['Conjuração Divina', 'Discípulo da Vida'],
    attacksPerAction: 1,
    inventory: [
      { name: 'Maça (1d6 concussão)', quantity: 1, weightLbs: 4 },
      { name: 'Cota de Malha (CA 16)', quantity: 1, weightLbs: 55 },
      { name: 'Escudo (+2 CA)', quantity: 1, weightLbs: 6 },
    ],
  };

  const rogue: CharacterSheet = {
    id: `char-rogue-${ts}`,
    name: 'Kaelen, Olhos da Noite',
    race: 'Meio-Elfo',
    className: 'Rogue',
    subclass: '',
    level: 1,
    background: 'Criminoso',
    alignment: 'Caótico e Neutro',
    proficiencyBonus: 2,
    abilities: {
      strength: 10,
      dexterity: 16,
      constitution: 14,
      intelligence: 12,
      wisdom: 12,
      charisma: 14,
    },
    savingThrowProficiencies: ['dexterity', 'intelligence'],
    skillProficiencies: ['Furtividade', 'Prestidigitação', 'Acrobacia', 'Enganação'],
    vitals: {
      hpCurrent: 10,
      hpMax: 10,
      hpTemp: 0,
      armorClass: 14,
      initiativeModifier: 3,
      speedFeet: 30,
      hitDiceTotal: 1,
      hitDiceCurrent: 1,
      hitDiceType: 'd8',
      deathSaves: { successes: 0, failures: 0 },
      conditions: [],
    },
    spellSlots: { levels: {} },
    spellsKnownOrPrepared: [],
    classResources: {
      rogue: {
        sneakAttackDiceCount: 1,
        cunningActionUnlocked: false,
        expertises: ['Furtividade', 'Prestidigitação'],
      },
    },
    features: ['Ataque Furtivo (1d6)', 'Especialização em Furtividade'],
    attacksPerAction: 1,
    inventory: [
      { name: 'Rapieira (1d8 perfurante)', quantity: 1, weightLbs: 2 },
      { name: 'Arco Curto (1d6 perfurante)', quantity: 1, weightLbs: 2 },
    ],
  };

  const wizard: CharacterSheet = {
    id: `char-wizard-${ts}`,
    name: 'Morvath de Netheril',
    race: 'Gnomo',
    className: 'Wizard',
    subclass: '',
    level: 1,
    background: 'Sábio',
    alignment: 'Neutro e Bom',
    proficiencyBonus: 2,
    abilities: {
      strength: 8,
      dexterity: 14,
      constitution: 14,
      intelligence: 16,
      wisdom: 12,
      charisma: 10,
    },
    savingThrowProficiencies: ['intelligence', 'wisdom'],
    skillProficiencies: ['Arcanismo', 'História', 'Investigação'],
    vitals: {
      hpCurrent: 8,
      hpMax: 8,
      hpTemp: 0,
      armorClass: 12,
      initiativeModifier: 2,
      speedFeet: 25,
      hitDiceTotal: 1,
      hitDiceCurrent: 1,
      hitDiceType: 'd6',
      deathSaves: { successes: 0, failures: 0 },
      conditions: [],
    },
    spellSlots: {
      levels: {
        1: { level: 1, total: 2, used: 0 },
      },
    },
    spellsKnownOrPrepared: ['Raio de Fogo', 'Mísseis Mágicos', 'Armadura Arcana', 'Escudo Arcano'],
    classResources: {
      wizard: {
        arcaneRecoveryAvailable: true,
        spellbookSpells: ['Mísseis Mágicos', 'Armadura Arcana', 'Escudo Arcano', 'Detectar Magia'],
        preparedSpellsMax: 4,
        preparedSpellsActive: ['Mísseis Mágicos', 'Armadura Arcana', 'Escudo Arcano'],
      },
    },
    features: ['Recuperação Arcana', 'Ritual Casting'],
    attacksPerAction: 1,
    inventory: [
      { name: 'Cajado Arcano', quantity: 1, weightLbs: 4 },
      { name: 'Grimório de Couro Encantado', quantity: 1, weightLbs: 3 },
    ],
  };

  return [barbarian, cleric, rogue, wizard];
}

export function createRequiredHero(req: {
  required_name?: string;
  required_class?: string;
  required_race?: string;
  suggested_options?: string[];
}): CharacterSheet {
  const ts = Date.now();

  // 1. Extração Inteligente de Classe D&D (inclusive de menções com parênteses ex: "A Arqueira Elfa (Ranger)")
  const candidateTexts: string[] = [];
  if (req.required_class) candidateTexts.push(req.required_class);
  if (req.suggested_options) candidateTexts.push(...req.suggested_options);

  let detectedClass: CharacterClassType = 'Fighter';
  for (const text of candidateTexts) {
    const t = text.toLowerCase();
    // Checagem canônica ou entre parênteses
    if (t.includes('ranger') || t.includes('patrulheir') || t.includes('arqueir')) {
      detectedClass = 'Ranger';
      break;
    } else if (t.includes('druid') || t.includes('xamã') || t.includes('xama')) {
      detectedClass = 'Druid';
      break;
    } else if (t.includes('cleric') || t.includes('clérig') || t.includes('clerig') || t.includes('sacerdot')) {
      detectedClass = 'Cleric';
      break;
    } else if (t.includes('paladin')) {
      detectedClass = 'Paladin';
      break;
    } else if (t.includes('rogue') || t.includes('ladin')) {
      detectedClass = 'Rogue';
      break;
    } else if (t.includes('wizard') || t.includes('mag') || t.includes('mago') || t.includes('artífice') || t.includes('artifice')) {
      detectedClass = 'Wizard';
      break;
    } else if (t.includes('barbar') || t.includes('bárbar')) {
      detectedClass = 'Barbarian';
      break;
    } else if (t.includes('bard')) {
      detectedClass = 'Bard';
      break;
    } else if (t.includes('monk') || t.includes('monge')) {
      detectedClass = 'Monk';
      break;
    } else if (t.includes('sorcerer') || t.includes('feiticeir')) {
      detectedClass = 'Sorcerer';
      break;
    } else if (t.includes('warlock') || t.includes('brux')) {
      detectedClass = 'Warlock';
      break;
    } else if (t.includes('fighter') || t.includes('guerreir')) {
      detectedClass = 'Fighter';
      break;
    }
  }

  // 2. Extração Inteligente de Raça
  let detectedRace = req.required_race || 'Humano';
  if (!req.required_race) {
    for (const text of candidateTexts) {
      const t = text.toLowerCase();
      if (t.includes('elf') || t.includes('elfa') || t.includes('elfo')) {
        detectedRace = 'Elfo';
        break;
      } else if (t.includes('anão') || t.includes('anao') || t.includes('anã') || t.includes('dwarf')) {
        detectedRace = 'Anão';
        break;
      } else if (t.includes('lagarto') || t.includes('draconat') || t.includes('dragonborn')) {
        detectedRace = 'Draconato';
        break;
      } else if (t.includes('halfling')) {
        detectedRace = 'Halfling';
        break;
      } else if (t.includes('tiefling')) {
        detectedRace = 'Tiefling';
        break;
      }
    }
  }

  // 3. Nome
  const rawName =
    req.required_name ||
    (req.suggested_options &&
      req.suggested_options.find(
        (s) =>
          !['guerreiro', 'ladino', 'mago', 'clérigo', 'fighter', 'rogue', 'wizard', 'cleric', 'ranger', 'druida', 'druid'].includes(
            s.toLowerCase().replace(/\s*\(.*?\)/g, '').trim()
          )
      )) ||
    (detectedClass === 'Cleric'
      ? 'A Sacerdotisa'
      : detectedClass === 'Ranger'
      ? 'A Arqueira Elfa'
      : detectedClass === 'Druid'
      ? 'O Xamã Anão'
      : detectedClass === 'Wizard'
      ? 'O Arcanista'
      : detectedClass === 'Paladin'
      ? 'O Campeão Sagrado'
      : detectedClass === 'Rogue'
      ? 'O Espião Ágil'
      : detectedClass === 'Barbarian'
      ? 'O Guerreiro Feroz'
      : 'O Vanguarda');

  // Limpa sufixos de parênteses do nome caso venham nele (ex: "A Arqueira Elfa (Ranger)" -> "A Arqueira Elfa")
  const name = rawName.replace(/\s*\([a-zA-Z0-9\s/]+\)\s*$/, '').trim();

  const isRanger = detectedClass === 'Ranger';
  const isDruid = detectedClass === 'Druid';
  const isCleric = detectedClass === 'Cleric';
  const isRogue = detectedClass === 'Rogue';
  const isWizard = detectedClass === 'Wizard';
  const isBarbarian = detectedClass === 'Barbarian';
  const isPaladin = detectedClass === 'Paladin';

  return {
    id: `char-req-${ts}`,
    name,
    race: detectedRace,
    className: detectedClass,
    subclass:
      isCleric ? 'Domínio da Vida' : isDruid ? 'Círculo da Terra' : isRanger ? 'Caçador' : '',
    level: 1,
    background: 'Veterano',
    alignment: 'Leal e Neutro',
    proficiencyBonus: 2,
    abilities: {
      strength: detectedClass === 'Fighter' || isBarbarian || isPaladin ? 16 : 10,
      dexterity: isRanger || isRogue ? 16 : 14,
      constitution: 14,
      intelligence: isWizard ? 16 : 10,
      wisdom: isCleric || isDruid ? 16 : isRanger ? 14 : 12,
      charisma: detectedClass === 'Bard' || isPaladin ? 16 : 10,
    },
    savingThrowProficiencies:
      detectedClass === 'Fighter'
        ? ['strength', 'constitution']
        : isRanger
        ? ['strength', 'dexterity']
        : isRogue
        ? ['dexterity', 'intelligence']
        : isDruid
        ? ['intelligence', 'wisdom']
        : ['wisdom', 'charisma'],
    skillProficiencies: isRanger
      ? ['Sobrevivência', 'Furtividade', 'Percepção']
      : isDruid
      ? ['Natureza', 'Medicina', 'Percepção']
      : ['Atletismo', 'Percepção', 'Sobrevivência'],
    vitals: {
      hpCurrent: isBarbarian ? 14 : detectedClass === 'Fighter' || isPaladin || isRanger ? 12 : 10,
      hpMax: isBarbarian ? 14 : detectedClass === 'Fighter' || isPaladin || isRanger ? 12 : 10,
      hpTemp: 0,
      armorClass: isRanger ? 15 : detectedClass === 'Fighter' || isPaladin ? 18 : isDruid ? 14 : 15,
      initiativeModifier: isRanger || isRogue ? 3 : 2,
      speedFeet: detectedRace === 'Anão' ? 25 : 30,
      hitDiceTotal: 1,
      hitDiceCurrent: 1,
      hitDiceType: isBarbarian ? 'd12' : detectedClass === 'Fighter' || isPaladin || isRanger ? 'd10' : 'd8',
      deathSaves: { successes: 0, failures: 0 },
      conditions: [],
    },
    spellSlots: isCleric || isDruid ? { levels: { 1: { level: 1, total: 2, used: 0 } } } : { levels: {} },
    spellsKnownOrPrepared: isCleric
      ? ['Curar Ferimentos', 'Bênção', 'Chama Sagrada']
      : isDruid
      ? ['Curar Ferimentos', 'Emaranhar', 'Bordoada']
      : [],
    classResources: {
      fighter: {
        secondWindAvailable: true,
        actionSurgeAvailable: false,
        fightingStyle: 'Defesa',
      },
    },
    features: isRanger ? ['Inimigo Favorito', 'Explorador Natural'] : isDruid ? ['Druídico'] : ['Retomar o Fôlego'],
    attacksPerAction: 1,
    inventory: isRanger
      ? [
          { name: 'Arco Longo Élfico (+5, 1d8+3 perfurante)', quantity: 1, weightLbs: 2, isEquipped: true },
          { name: 'Aljava com 20 Flechas', quantity: 1, weightLbs: 1, isEquipped: true },
          { name: 'Espada Curta (1d6 perfurante)', quantity: 1, weightLbs: 2, isEquipped: true },
          { name: 'Gibão de Couro Batido (12 + Des)', quantity: 1, weightLbs: 13, isEquipped: true },
        ]
      : isDruid
      ? [
          { name: 'Cajado de Carvalho dos Anciãos (1d6/1d8 contundente)', quantity: 1, weightLbs: 4, isEquipped: true },
          { name: 'Escudo de Madeira Reforçada (+2 CA)', quantity: 1, weightLbs: 6, isEquipped: true },
          { name: 'Armadura de Couro Tratado (11 + Des)', quantity: 1, weightLbs: 10, isEquipped: true },
        ]
      : [
          { name: 'Espada Longa de Aço (1d8/1d10 cortante)', quantity: 1, weightLbs: 3, isEquipped: true },
          { name: 'Escudo Pesado (+2 CA)', quantity: 1, weightLbs: 6, isEquipped: true },
          { name: 'Cota de Malha Completa (16 CA)', quantity: 1, weightLbs: 40, isEquipped: true },
        ],
  };
}
