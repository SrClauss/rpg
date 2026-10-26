import type { AbilityName, AbilityScores, CharacterClassType, SpellSlots } from '../../types/character';

export type AbilityGenerationMethod = 'point_buy' | 'standard_array' | 'roll_4d6';

/**
 * Tabela Oficial SRD 5.1 para Compra de Pontos (Point Buy - 27 pontos)
 * Atributo base mínimo: 8 (custo 0) | Atributo base máximo antes de raça: 15 (custo 9)
 */
export const POINT_BUY_TOTAL_BUDGET = 27;
export const POINT_BUY_MIN_SCORE = 8;
export const POINT_BUY_MAX_SCORE = 15;

export const POINT_BUY_COST_TABLE: Record<number, number> = {
  8: 0,
  9: 1,
  10: 2,
  11: 3,
  12: 4,
  13: 5,
  14: 7,
  15: 9,
};

export function calculatePointBuySpent(baseScores: AbilityScores): number {
  return (Object.values(baseScores) as number[]).reduce((acc, val) => {
    const clamped = Math.max(8, Math.min(15, val));
    return acc + (POINT_BUY_COST_TABLE[clamped] ?? 0);
  }, 0);
}

/**
 * Metadados Raciais Oficiais SRD 5.1 com Bônus de Atributo, Deslocamento e Traços
 */
export interface RaceMetadata {
  id: string;
  name: string;
  bonuses: Partial<Record<AbilityName, number>>;
  customPlusOneCount?: number; // Para Meio-Elfo (+2 CAR e +1 em outros 2 atributos)
  speedFeet: number;
  hpBonusPerLevel?: number; // Anão da Colina: Tenacidade Anã (+1 PV por nível)
  traits: string[];
}

export const SRD_RACES: RaceMetadata[] = [
  {
    id: 'Humano (+1 em todos)',
    name: 'Humano',
    bonuses: {
      strength: 1,
      dexterity: 1,
      constitution: 1,
      intelligence: 1,
      wisdom: 1,
      charisma: 1,
    },
    speedFeet: 30,
    traits: ['Versatilidade Humana (+1 em todos os 6 atributos)', 'Idioma Adicional'],
  },
  {
    id: 'Alto Elfo (+2 DES, +1 INT)',
    name: 'Alto Elfo',
    bonuses: { dexterity: 2, intelligence: 1 },
    speedFeet: 30,
    traits: ['Visão no Escuro (60 pés)', 'Sentidos Aguçados (Percepção)', 'Ancestral Feérico', 'Truque de Mago Adicional'],
  },
  {
    id: 'Elfo da Floresta (+2 DES, +1 SAB)',
    name: 'Elfo da Floresta',
    bonuses: { dexterity: 2, wisdom: 1 },
    speedFeet: 35,
    traits: ['Visão no Escuro (60 pés)', 'Pés Ligeiros (35 pés)', 'Máscara da Natureza', 'Ancestral Feérico'],
  },
  {
    id: 'Anão da Colina (+2 CON, +1 SAB)',
    name: 'Anão da Colina',
    bonuses: { constitution: 2, wisdom: 1 },
    speedFeet: 25,
    hpBonusPerLevel: 1,
    traits: ['Visão no Escuro (60 pés)', 'Resiliência Anã (Veneno)', 'Tenacidade Anã (+1 PV máximo por nível)'],
  },
  {
    id: 'Anão da Montanha (+2 FOR, +2 CON)',
    name: 'Anão da Montanha',
    bonuses: { strength: 2, constitution: 2 },
    speedFeet: 25,
    traits: ['Visão no Escuro (60 pés)', 'Resiliência Anã (Veneno)', 'Treinamento Anão com Armaduras Leves e Médias'],
  },
  {
    id: 'Halfling Pés-Leves (+2 DES, +1 CAR)',
    name: 'Halfling Pés-Leves',
    bonuses: { dexterity: 2, charisma: 1 },
    speedFeet: 25,
    traits: ['Sortudo (Rerrola 1 natural no d20)', 'Bravura (Vantagem contra Amedrontado)', 'Furtividade Natural'],
  },
  {
    id: 'Draconato (+2 FOR, +1 CAR)',
    name: 'Draconato',
    bonuses: { strength: 2, charisma: 1 },
    speedFeet: 30,
    traits: ['Ancestral Dracônico', 'Arma de Sopro (2d6 elemental)', 'Resistência a Dano Elemental'],
  },
  {
    id: 'Tiefling (+2 CAR, +1 INT)',
    name: 'Tiefling',
    bonuses: { charisma: 2, intelligence: 1 },
    speedFeet: 30,
    traits: ['Visão no Escuro (60 pés)', 'Resistência Infernal (Resistência a Fogo)', 'Legado Infernal (Taumaturgia / Repreensão Infernal)'],
  },
  {
    id: 'Gnomo da Rocha (+2 INT, +1 CON)',
    name: 'Gnomo da Rocha',
    bonuses: { intelligence: 2, constitution: 1 },
    speedFeet: 25,
    traits: ['Visão no Escuro (60 pés)', 'Esperteza Gnômica (Vantagem em Salvaguardas INT/SAB/CAR contra magia)', 'Conhecimento de Artífice'],
  },
  {
    id: 'Meio-Elfo (+2 CAR, +1 em dois à escolha)',
    name: 'Meio-Elfo',
    bonuses: { charisma: 2 },
    customPlusOneCount: 2,
    speedFeet: 30,
    traits: ['Visão no Escuro (60 pés)', 'Ancestral Feérico', 'Versatilidade em Perícias (2 perícias extras)'],
  },
  {
    id: 'Meio-Orc (+2 FOR, +1 CON)',
    name: 'Meio-Orc',
    bonuses: { strength: 2, constitution: 1 },
    speedFeet: 30,
    traits: ['Visão no Escuro (60 pés)', 'Ameaçador (Intimidação)', 'Resistência Implacável (Cai a 1 PV em vez de 0)', 'Ataques Selvagens (+1 dado em crítico)'],
  },
];

/**
 * Calcula o Bônus de Proficiência oficial do SRD 5.1 para os níveis 1 a 20
 */
export function getProficiencyBonusForLevel(level: number): number {
  const clamped = Math.max(1, Math.min(20, level));
  return Math.ceil(clamped / 4) + 1;
}

/**
 * Média fixa oficial do SRD 5.1 para aumento de PV por nível acima do 1º:
 * d6 -> 4, d8 -> 5, d10 -> 6, d12 -> 7
 */
export function getFixedHpAverageForHitDie(hitDieValue: number): number {
  return Math.floor(hitDieValue / 2) + 1;
}

/**
 * Níveis que concedem Incremento no Valor de Habilidade (ASI) por classe segundo o SRD 5.1
 */
export function getAsiLevelsForClass(className: CharacterClassType): number[] {
  if (className === 'Fighter') {
    return [4, 6, 8, 12, 14, 16, 19];
  }
  if (className === 'Rogue') {
    return [4, 8, 10, 12, 16, 19];
  }
  return [4, 8, 12, 16, 19];
}

export function isAsiLevel(className: CharacterClassType, level: number): boolean {
  return getAsiLevelsForClass(className).includes(level);
}

/**
 * Nível em que cada classe escolhe sua Subclasse / Arquétipo no SRD 5.1
 */
export function getSubclassUnlockLevel(className: CharacterClassType): number {
  switch (className) {
    case 'Cleric':
    case 'Sorcerer':
    case 'Warlock':
      return 1;
    case 'Druid':
    case 'Wizard':
      return 2;
    default:
      return 3;
  }
}

export const SRD_SUBCLASSES: Record<CharacterClassType, { name: string; description: string }[]> = {
  Barbarian: [
    { name: 'Caminho do Furioso (Berserker)', description: 'Canaliza a fúria em Frenesi devastador, permitindo um ataque corpo a corpo com ação bônus.' },
    { name: 'Caminho do Guerreiro Totêmico (Urso/Águia/Lobo)', description: 'Comunga com espíritos animais; o Totem do Urso concede resistência a todos os danos exceto psíquico.' },
  ],
  Bard: [
    { name: 'Colégio do Conhecimento (Lore)', description: 'Domina 3 perícias extras, Palavras de Interrupção com reação e Segredos Mágicos Adicionais no nível 6.' },
    { name: 'Colégio da Bravura (Valor)', description: 'Treinamento em armaduras médias, escudos, armas marciais e Inspiração de Combate para somar na CA ou dano.' },
  ],
  Cleric: [
    { name: 'Domínio da Vida', description: 'Proficiência em armadura pesada e Discípulo da Vida (+2 + nível da magia em todas as curas).' },
    { name: 'Domínio da Luz', description: 'Labareda Protetora com reação e magias solares devastadoras como Bola de Fogo e Raio Ardente.' },
    { name: 'Domínio da Guerra', description: 'Armadura pesada, armas marciais, Sacerdote da Guerra (ataque extra na ação bônus) e Golpe Guiado (+10 no ataque).' },
  ],
  Druid: [
    { name: 'Círculo da Terra', description: 'Recuperação Natural de espaços de magia e magias de terreno sempre preparadas.' },
    { name: 'Círculo da Lua', description: 'Forma Selvagem de Combate como ação bônus em feras de ND mais elevado (ND 1 no nível 2, ND Nível/3 depois).' },
  ],
  Fighter: [
    { name: 'Campeão (Champion)', description: 'Crítico Aprimorado (19–20 no d20 no nível 3; 18–20 no nível 15), Atleta Notável e Estilo de Luta Adicional.' },
    { name: 'Mestre de Batalha (Battle Master)', description: 'Domina Manobras Táticas de Combate alimentadas por Dados de Superioridade (d8 -> d10 -> d12).' },
    { name: 'Cavaleiro Arcano (Eldritch Knight)', description: 'Combina maestria marcial com magias de Abjuração e Evocação da lista de Mago e Vínculo com Arma.' },
  ],
  Monk: [
    { name: 'Tradição da Mão Aberta', description: 'Técnica da Mão Aberta para derrubar, empurrar 15ft ou tirar reações ao atingir com Rajada de Golpes.' },
    { name: 'Tradição da Sombra', description: 'Artes das Sombras via Ki, Passo das Sombras (teletransporte de 60ft na penumbra) e Manto de Sombras.' },
  ],
  Paladin: [
    { name: 'Juramento de Devoção', description: 'Arma Sagrada (+CAR no ataque), Expulsar o Profano e Aura de Devoção (imunidade a enfeitiçado).' },
    { name: 'Juramento de Vingança', description: 'Inimizade Jurada (vantagem nos ataques contra 1 alvo), Magias de Caçador e Vingador Implacável.' },
  ],
  Ranger: [
    { name: 'Caçador (Hunter)', description: 'Presa do Caçador (Assassino de Colossos +1d8 por turno), Táticas Defensivas e Ataques Múltiplos (Saraivada/Redemoinho).' },
    { name: 'Mestre das Feras (Beast Master)', description: 'Companheiro Animal leal que luta ao seu lado e escala com seu bônus de proficiência.' },
  ],
  Rogue: [
    { name: 'Ladrão (Thief)', description: 'Mãos Rápidas (usar objetos/desarmar armadilhas na ação bônus), Andarilho de Telhados e Usar Dispositivo Mágico.' },
    { name: 'Assassino (Assassin)', description: 'Assassinato (vantagem contra quem ainda não agiu e crítico automático contra alvos surpresos).' },
  ],
  Sorcerer: [
    { name: 'Linhagem Dracônica', description: 'Resiliência Dracônica (CA base 13 + DES e +1 PV por nível), Afinidade Elemental (+CAR no dano) e Asas de Dragão no nível 14.' },
    { name: 'Magia Selvagem', description: 'Marés do Caos (vantagem sob demanda) e Dobrar a Sorte para alterar rolagens com Pontos de Feitiçaria.' },
  ],
  Warlock: [
    { name: 'O Corruptor (The Fiend)', description: 'Bênção do Obscuro (ganha CAR + Nível em PV temporários ao derrotar inimigos), Sorte do Próprio Demônio e Arremessar no Inferno (10d10).' },
    { name: 'O Grande Antigo (Great Old One)', description: 'Mente Desperta (telepatia 30ft), Proteção Entrópica e Escudo de Pensamentos.' },
  ],
  Wizard: [
    { name: 'Escola de Evocação', description: 'Esculpir Magias (protege aliados de suas magias de área), Truque Potente e Evocação Potencializada (+INT no dano).' },
    { name: 'Escola de Abjuração', description: 'Proteção Arcana (barreira de PV = 2x Nível + INT que absorve dano) e Resistência a Magias no nível 14.' },
  ],
};

/**
 * Tabela Oficial SRD 5.1 de Espaços de Magia para Conjuradores Completos (Bard, Cleric, Druid, Sorcerer, Wizard)
 * Índices de nível de personagem 1..20 -> Array de 9 posições (1º ao 9º Círculo)
 */
const FULL_CASTER_SLOTS_BY_LEVEL: Record<number, number[]> = {
  1:  [2, 0, 0, 0, 0, 0, 0, 0, 0],
  2:  [3, 0, 0, 0, 0, 0, 0, 0, 0],
  3:  [4, 2, 0, 0, 0, 0, 0, 0, 0],
  4:  [4, 3, 0, 0, 0, 0, 0, 0, 0],
  5:  [4, 3, 2, 0, 0, 0, 0, 0, 0],
  6:  [4, 3, 3, 0, 0, 0, 0, 0, 0],
  7:  [4, 3, 3, 1, 0, 0, 0, 0, 0],
  8:  [4, 3, 3, 2, 0, 0, 0, 0, 0],
  9:  [4, 3, 3, 3, 1, 0, 0, 0, 0],
  10: [4, 3, 3, 3, 2, 0, 0, 0, 0],
  11: [4, 3, 3, 3, 2, 1, 0, 0, 0],
  12: [4, 3, 3, 3, 2, 1, 0, 0, 0],
  13: [4, 3, 3, 3, 2, 1, 1, 0, 0],
  14: [4, 3, 3, 3, 2, 1, 1, 0, 0],
  15: [4, 3, 3, 3, 2, 1, 1, 1, 0],
  16: [4, 3, 3, 3, 2, 1, 1, 1, 0],
  17: [4, 3, 3, 3, 2, 1, 1, 1, 1],
  18: [4, 3, 3, 3, 3, 1, 1, 1, 1],
  19: [4, 3, 3, 3, 3, 2, 1, 1, 1],
  20: [4, 3, 3, 3, 3, 2, 2, 1, 1],
};

/**
 * Tabela Oficial SRD 5.1 para Meio-Conjuradores (Paladin, Ranger)
 */
const HALF_CASTER_SLOTS_BY_LEVEL: Record<number, number[]> = {
  1:  [0, 0, 0, 0, 0, 0, 0, 0, 0],
  2:  [2, 0, 0, 0, 0, 0, 0, 0, 0],
  3:  [3, 0, 0, 0, 0, 0, 0, 0, 0],
  4:  [3, 0, 0, 0, 0, 0, 0, 0, 0],
  5:  [4, 2, 0, 0, 0, 0, 0, 0, 0],
  6:  [4, 2, 0, 0, 0, 0, 0, 0, 0],
  7:  [4, 3, 0, 0, 0, 0, 0, 0, 0],
  8:  [4, 3, 0, 0, 0, 0, 0, 0, 0],
  9:  [4, 3, 2, 0, 0, 0, 0, 0, 0],
  10: [4, 3, 2, 0, 0, 0, 0, 0, 0],
  11: [4, 3, 3, 0, 0, 0, 0, 0, 0],
  12: [4, 3, 3, 0, 0, 0, 0, 0, 0],
  13: [4, 3, 3, 1, 0, 0, 0, 0, 0],
  14: [4, 3, 3, 1, 0, 0, 0, 0, 0],
  15: [4, 3, 3, 2, 0, 0, 0, 0, 0],
  16: [4, 3, 3, 2, 0, 0, 0, 0, 0],
  17: [4, 3, 3, 3, 1, 0, 0, 0, 0],
  18: [4, 3, 3, 3, 1, 0, 0, 0, 0],
  19: [4, 3, 3, 3, 2, 0, 0, 0, 0],
  20: [4, 3, 3, 3, 2, 0, 0, 0, 0],
};

/**
 * Tabela Oficial SRD 5.1 para Magia de Pacto do Bruxo (Warlock)
 */
const WARLOCK_PACT_BY_LEVEL: Record<number, { total: number; slotLevel: number }> = {
  1:  { total: 1, slotLevel: 1 },
  2:  { total: 2, slotLevel: 1 },
  3:  { total: 2, slotLevel: 2 },
  4:  { total: 2, slotLevel: 2 },
  5:  { total: 2, slotLevel: 3 },
  6:  { total: 2, slotLevel: 3 },
  7:  { total: 2, slotLevel: 4 },
  8:  { total: 2, slotLevel: 4 },
  9:  { total: 2, slotLevel: 5 },
  10: { total: 2, slotLevel: 5 },
  11: { total: 3, slotLevel: 5 },
  12: { total: 3, slotLevel: 5 },
  13: { total: 3, slotLevel: 5 },
  14: { total: 3, slotLevel: 5 },
  15: { total: 3, slotLevel: 5 },
  16: { total: 3, slotLevel: 5 },
  17: { total: 4, slotLevel: 5 },
  18: { total: 4, slotLevel: 5 },
  19: { total: 4, slotLevel: 5 },
  20: { total: 4, slotLevel: 5 },
};

export function getSpellSlotsForClassAndLevel(
  className: CharacterClassType,
  level: number
): SpellSlots {
  const clamped = Math.max(1, Math.min(20, level));

  if (className === 'Warlock') {
    const pact = WARLOCK_PACT_BY_LEVEL[clamped] || { total: 1, slotLevel: 1 };
    return {
      levels: {},
      pactMagic: {
        slotLevel: pact.slotLevel,
        total: pact.total,
        used: 0,
      },
    };
  }

  const isFullCaster = ['Bard', 'Cleric', 'Druid', 'Sorcerer', 'Wizard'].includes(className);
  const isHalfCaster = ['Paladin', 'Ranger'].includes(className);

  const table = isFullCaster
    ? FULL_CASTER_SLOTS_BY_LEVEL[clamped]
    : isHalfCaster
    ? HALF_CASTER_SLOTS_BY_LEVEL[clamped]
    : [0, 0, 0, 0, 0, 0, 0, 0, 0];

  const levels: SpellSlots['levels'] = {};
  table.forEach((count, idx) => {
    const circle = idx + 1;
    if (count > 0) {
      levels[circle] = {
        level: circle,
        total: count,
        used: 0,
      };
    }
  });

  return { levels };
}

/**
 * Catálogo de Magias SRD 5.1 por Círculo (1º ao 9º Círculo) para Evolução de Conjuradores
 */
export const SRD_SPELLS_BY_CIRCLE: Record<number, string[]> = {
  1: [
    'Mísseis Mágicos (1º Círculo)',
    'Escudo Arcano (1º Círculo)',
    'Armadura Arcana (1º Círculo)',
    'Onda Trovejante (1º Círculo)',
    'Curar Ferimentos (1º Círculo)',
    'Bênção (1º Círculo)',
    'Raio Guiador (1º Círculo)',
    'Marca do Caçador (1º Círculo)',
    'Dormir (1º Círculo)',
  ],
  2: [
    'Passo Nebuloso (2º Círculo)',
    'Raio Ardente (2º Círculo)',
    'Invisibilidade (2º Círculo)',
    'Imobilizar Pessoa (2º Círculo)',
    'Arma Espiritual (2º Círculo)',
    'Despedaçar (2º Círculo)',
  ],
  3: [
    'Bola de Fogo (3º Círculo)',
    'Relâmpago (3º Círculo)',
    'Contramágica (3º Círculo)',
    'Voo (3º Círculo)',
    'Espíritos Guardiões (3º Círculo)',
    'Revivificar (3º Círculo)',
  ],
  4: [
    'Invisibilidade Maior (4º Círculo)',
    'Porta Dimensional (4º Círculo)',
    'Muralha de Fogo (4º Círculo)',
    'Tempestade de Gelo (4º Círculo)',
    'Banimento (4º Círculo)',
  ],
  5: [
    'Cone de Frio (5º Círculo)',
    'Imobilizar Monstro (5º Círculo)',
    'Muralha de Energia (5º Círculo)',
    'Coluna de Chamas (5º Círculo)',
    'Restauração Maior (5º Círculo)',
  ],
  6: [
    'Corrente de Relâmpagos (6º Círculo)',
    'Desintegrar (6º Círculo)',
    'Globo de Invulnerabilidade (6º Círculo)',
    'Cura Completa (6º Círculo)',
  ],
  7: [
    'Teletransporte (7º Círculo)',
    'Bola de Fogo Controlável (7º Círculo)',
    'Dedo da Morte (7º Círculo)',
    'Tempestade de Fogo (7º Círculo)',
  ],
  8: [
    'Explosão Solar (8º Círculo)',
    'Dominar Monstro (8º Círculo)',
    'Palavra de Poder: Atordoar (8º Círculo)',
    'Terremoto (8º Círculo)',
  ],
  9: [
    'Chuva de Meteoros (9º Círculo)',
    'Desejo (9º Círculo)',
    'Palavra de Poder: Matar (9º Círculo)',
    'Parar o Tempo (9º Círculo)',
    'Ressurreição Verdadeira (9º Círculo)',
  ],
};

/**
 * Características Desbloqueadas Nível a Nível (1 a 20) segundo o SRD 5.1 para cada Classe
 */
export function getClassFeaturesAtLevel(
  className: CharacterClassType,
  level: number
): string[] {
  const features: string[] = [];

  if (isAsiLevel(className, level)) {
    features.push('Incremento no Valor de Habilidade (ASI: +2 em 1 atributo ou +1 em 2 atributos)');
  }

  switch (className) {
    case 'Fighter': {
      if (level === 1) features.push('Estilo de Luta', 'Retomar o Fôlego (1d10 + Nível)');
      if (level === 2) features.push('Surto de Ação (1 uso por descanso)');
      if (level === 3) features.push('Arquétipo Marcial (Campeão / Mestre de Batalha / Cavaleiro Arcano)');
      if (level === 5) features.push('Ataque Extra (2 ataques por Ação de Ataque)');
      if (level === 7) features.push('Característica de Arquétipo Marcial (Nível 7)');
      if (level === 9) features.push('Indomável (1 uso — rerrola uma Salvaguarda falha)');
      if (level === 10) features.push('Característica de Arquétipo Marcial (Nível 10)');
      if (level === 11) features.push('Ataque Extra Aprimorado (3 ataques por Ação de Ataque)');
      if (level === 13) features.push('Indomável (2 usos por descanso longo)');
      if (level === 15) features.push('Característica de Arquétipo Marcial Superior (Nível 15)');
      if (level === 17) features.push('Surto de Ação (2 usos)', 'Indomável (3 usos)');
      if (level === 18) features.push('Característica de Arquétipo Marcial Mestre (Nível 18)');
      if (level === 20) features.push('Ataque Extra Supremo (4 ataques por Ação de Ataque)');
      break;
    }
    case 'Barbarian': {
      if (level === 1) features.push('Fúria (2 usos, +2 dano)', 'Defesa Sem Armadura (10 + DES + CON)');
      if (level === 2) features.push('Ataque Descuidado (Vantagem nos ataques FOR)', 'Sentido de Perigo (Vantagem em DES)');
      if (level === 3) features.push('Caminho Primitivo (Subclasse)', 'Fúria (3 usos)');
      if (level === 5) features.push('Ataque Extra (2 ataques)', 'Movimento Rápido (+10 pés de deslocamento)');
      if (level === 6) features.push('Característica do Caminho Primitivo (Nível 6)', 'Fúria (4 usos)');
      if (level === 7) features.push('Instinto Selvagem (Vantagem na Iniciativa)');
      if (level === 9) features.push('Crítico Brutal (+1 dado de arma em acertos críticos)', 'Dano de Fúria (+3)');
      if (level === 10) features.push('Característica do Caminho Primitivo (Nível 10)');
      if (level === 11) features.push('Fúria Implacável (Teste CD 10 CON para cair a 1 PV)');
      if (level === 12) features.push('Fúria (5 usos)');
      if (level === 13) features.push('Crítico Brutal (+2 dados de arma em críticos)');
      if (level === 14) features.push('Característica do Caminho Primitivo (Nível 14)');
      if (level === 15) features.push('Fúria Persistente');
      if (level === 16) features.push('Dano de Fúria (+4)');
      if (level === 17) features.push('Crítico Brutal (+3 dados de arma)', 'Fúria (6 usos)');
      if (level === 18) features.push('Força Indomável (Mínimo no teste de FOR = valor de FOR)');
      if (level === 20) features.push('Campeão Primitivo (+4 FOR e +4 CON até máx 24, Fúria Ilimitada)');
      break;
    }
    case 'Rogue': {
      const sneakDice = Math.ceil(level / 2);
      if (level === 1) features.push(`Ataque Furtivo (${sneakDice}d6)`, 'Especialização (Expertise em 2 perícias)', 'Gíria de Ladrão');
      if (level === 2) features.push('Ação Ardilosa (Disparada, Desengajar ou Esconder com Ação Bônus)');
      if (level === 3) features.push('Arquétipo de Ladino (Subclasse)', `Ataque Furtivo (${sneakDice}d6)`);
      if (level === 5) features.push('Esquiva Sobrenatural (Reduz dano de um ataque pela metade com Reação)', `Ataque Furtivo (${sneakDice}d6)`);
      if (level === 6) features.push('Especialização Adicional (Mais 2 perícias com proficiência dobrada)');
      if (level === 7) features.push('Evasão (Zero dano em sucesso de Salvaguarda de DES, metade em falha)', `Ataque Furtivo (${sneakDice}d6)`);
      if (level === 9) features.push('Característica de Arquétipo de Ladino (Nível 9)', `Ataque Furtivo (${sneakDice}d6)`);
      if (level === 11) features.push('Talento Confiável (Todo resultado <= 9 no d20 em perícias proficientes vira 10)', `Ataque Furtivo (${sneakDice}d6)`);
      if (level === 13) features.push('Característica de Arquétipo de Ladino (Nível 13)', `Ataque Furtivo (${sneakDice}d6)`);
      if (level === 14) features.push('Sentido Cego (Detecta criaturas ocultas/invisíveis a até 10 pés)');
      if (level === 15) features.push('Mente Escorregadia (Proficiência em Salvaguardas de Sabedoria)', `Ataque Furtivo (${sneakDice}d6)`);
      if (level === 17) features.push('Característica de Arquétipo de Ladino (Nível 17)', `Ataque Furtivo (${sneakDice}d6)`);
      if (level === 18) features.push('Elusivo (Nenhum ataque tem vantagem contra você enquanto não incapacitado)');
      if (level === 19) features.push(`Ataque Furtivo (${sneakDice}d6)`);
      if (level === 20) features.push('Golpe de Sorte (Transforma uma falha em acerto ou 20 no d20)');
      break;
    }
    case 'Wizard': {
      if (level === 1) features.push('Conjuração Arcana (Grimório)', 'Recuperação Arcana (1x/dia no Descanso Curto)');
      if (level === 2) features.push('Tradição Arcana (Escola de Magia SRD)');
      if (level === 3) features.push('Acesso ao 2º Círculo de Magias Arcanas');
      if (level === 5) features.push('Acesso ao 3º Círculo de Magias (Bola de Fogo / Contramágica)');
      if (level === 6) features.push('Característica da Tradição Arcana (Nível 6)');
      if (level === 7) features.push('Acesso ao 4º Círculo de Magias Arcanas');
      if (level === 9) features.push('Acesso ao 5º Círculo de Magias Arcanas');
      if (level === 10) features.push('Característica da Tradição Arcana (Nível 10)');
      if (level === 11) features.push('Acesso ao 6º Círculo de Magias Arcanas');
      if (level === 13) features.push('Acesso ao 7º Círculo de Magias Arcanas');
      if (level === 14) features.push('Característica da Tradição Arcana Superior (Nível 14)');
      if (level === 15) features.push('Acesso ao 8º Círculo de Magias Arcanas');
      if (level === 17) features.push('Acesso ao 9º Círculo de Magias (Desejo / Chuva de Meteoros)');
      if (level === 18) features.push('Maestria em Magia (Conjura 1 magia de 1º e 1 de 2º nível à vontade sem gastar espaço)');
      if (level === 20) features.push('Magias Assinatura (2 magias de 3º círculo sempre preparadas e 1x sem espaço)');
      break;
    }
    case 'Cleric': {
      if (level === 1) features.push('Conjuração Divina', 'Domínio Divino (Subclasse)');
      if (level === 2) features.push('Canalizar Divindade (1/descanso): Expulsar Mortos-Vivos');
      if (level === 5) features.push('Destruir Mortos-Vivos (ND 1/2 ou menor)');
      if (level === 6) features.push('Canalizar Divindade (2/descanso)', 'Característica de Domínio Divino (Nível 6)');
      if (level === 8) features.push('Destruir Mortos-Vivos (ND 1)', 'Golpe Divino / Conjuração Potente (+1d8 ou +SAB)');
      if (level === 10) features.push('Intervenção Divina (Chance percentual igual ao Nível do Clérigo)');
      if (level === 11) features.push('Destruir Mortos-Vivos (ND 2)', 'Acesso ao 6º Círculo Divino');
      if (level === 14) features.push('Destruir Mortos-Vivos (ND 3)', 'Golpe Divino Aprimorado (+2d8)');
      if (level === 17) features.push('Destruir Mortos-Vivos (ND 4)', 'Característica Suprema de Domínio (Nível 17)', 'Magias de 9º Círculo');
      if (level === 18) features.push('Canalizar Divindade (3/descanso)');
      if (level === 20) features.push('Intervenção Divina Aprimorada (Sucesso automático 100%)');
      break;
    }
    case 'Paladin': {
      if (level === 1) features.push('Sentido Divino', 'Cura pelas Mãos (5 x Nível PV)');
      if (level === 2) features.push('Estilo de Luta', 'Conjuração Divina', 'Destruição Divina (Divine Smite: +2d8 radiante por slot)');
      if (level === 3) features.push('Saúde Divina (Imune a doenças)', 'Juramento Sagrado (Subclasse)');
      if (level === 5) features.push('Ataque Extra (2 ataques por Ação)');
      if (level === 6) features.push('Aura de Proteção (+Mod. CAR em todas as Salvaguardas a até 10 pés)');
      if (level === 7) features.push('Característica de Juramento Sagrado (Nível 7)');
      if (level === 10) features.push('Aura da Coragem (Imunidade a Amedrontado a até 10 pés)');
      if (level === 11) features.push('Destruição Divina Aprimorada (+1d8 radiante automático em todo golpe corpo a corpo)');
      if (level === 14) features.push('Toque Purificador (Encerra magias com o toque)');
      if (level === 15) features.push('Característica de Juramento Sagrado (Nível 15)');
      if (level === 18) features.push('Auras Aprimoradas (Alcance expandido para 30 pés)');
      if (level === 20) features.push('Transformação de Campeão Sagrado (Nível 20)');
      break;
    }
    case 'Monk': {
      const die = level >= 17 ? '1d10' : level >= 11 ? '1d8' : level >= 5 ? '1d6' : '1d4';
      if (level === 1) features.push('Defesa Sem Armadura (10 + DES + SAB)', `Artes Marciais (${die})`);
      if (level === 2) features.push(`Ki (${level} pontos: Rajada de Golpes, Defesa Paciente, Passo do Vento)`, 'Movimento Sem Armadura (+10 pés)');
      if (level === 3) features.push('Tradição Monástica (Subclasse)', 'Defletir Projéteis (Reação: reduz 1d10 + DES + Nível)');
      if (level === 4) features.push('Queda Lenta (Reduz dano de queda em 5 x Nível)');
      if (level === 5) features.push('Ataque Extra (2 ataques)', `Ataque Atordoante (Stunning Strike) • Dado Marcial ${die}`);
      if (level === 6) features.push('Golpes de Ki (Ataques desarmados contam como mágicos)', 'Característica de Tradição Monástica (Nível 6)');
      if (level === 7) features.push('Evasão', 'Mente Tranquila (Encerra enfeitiçado/amedrontado)');
      if (level === 9) features.push('Movimento Sem Armadura Aprimorado (Corre por paredes e líquidos)');
      if (level === 10) features.push('Pureza Corporal (Imune a doenças e venenos)');
      if (level === 11) features.push('Característica de Tradição Monástica (Nível 11)', `Dado Marcial (${die})`);
      if (level === 13) features.push('Língua do Sol e da Lua (Compreende todos os idiomas)');
      if (level === 14) features.push('Alma de Diamante (Proficiência em TODAS as 6 Salvaguardas + rerrola com 1 Ki)');
      if (level === 15) features.push('Corpo Atemporal');
      if (level === 17) features.push('Característica de Tradição Monástica Suprema (Nível 17)', `Dado Marcial (${die})`);
      if (level === 18) features.push('Corpo Vazio (Invisibilidade e resistência a todos os danos exceto energia por 4 Ki)');
      if (level === 20) features.push('Auto-Perfeição (Recupera 4 Ki ao rolar Iniciativa sem Ki)');
      break;
    }
    case 'Bard': {
      const die = level >= 15 ? 'd12' : level >= 10 ? 'd10' : level >= 5 ? 'd8' : 'd6';
      if (level === 1) features.push('Conjuração de Bardo', `Inspiração de Bardo (${die})`);
      if (level === 2) features.push('Pau para Toda Obra (+Metade da Proficiência em testes não proficientes)', 'Canção de Descanso (d6)');
      if (level === 3) features.push('Colégio de Bardo (Subclasse)', 'Especialização (Expertise em 2 perícias)');
      if (level === 5) features.push(`Inspiração de Bardo (${die})`, 'Fonte de Inspiração (Recupera Inspiração em Descanso Curto)');
      if (level === 6) features.push('Canção de Proteção', 'Característica de Colégio de Bardo (Nível 6)');
      if (level === 10) features.push(`Inspiração de Bardo (${die})`, 'Especialização Adicional', 'Segredos Mágicos (2 magias de qualquer classe)');
      if (level === 14) features.push('Segredos Mágicos Adicionais (Nível 14)', 'Característica de Colégio de Bardo (Nível 14)');
      if (level === 15) features.push(`Inspiração de Bardo (${die})`);
      if (level === 18) features.push('Segredos Mágicos Supremos (Nível 18)');
      if (level === 20) features.push('Inspiração Superior');
      break;
    }
    case 'Druid': {
      if (level === 1) features.push('Druídico', 'Conjuração da Natureza');
      if (level === 2) features.push('Forma Selvagem (2 usos/descanso)', 'Círculo Druídico (Subclasse)');
      if (level === 4) features.push('Aprimoramento de Forma Selvagem (Criaturas nadadoras)');
      if (level === 6) features.push('Característica do Círculo Druídico (Nível 6)');
      if (level === 8) features.push('Aprimoramento de Forma Selvagem (Criaturas voadoras)');
      if (level === 10) features.push('Característica do Círculo Druídico (Nível 10)');
      if (level === 14) features.push('Característica do Círculo Druídico (Nível 14)');
      if (level === 18) features.push('Corpo Atemporal', 'Magias da Fera (Conjura enquanto em Forma Selvagem)');
      if (level === 20) features.push('Arquidruida (Forma Selvagem Ilimitada)');
      break;
    }
    case 'Ranger': {
      if (level === 1) features.push('Inimigo Favorito', 'Explorador Natural');
      if (level === 2) features.push('Estilo de Luta', 'Conjuração de Patrulheiro');
      if (level === 3) features.push('Arquétipo de Patrulheiro (Subclasse)', 'Consciência Primitiva');
      if (level === 5) features.push('Ataque Extra (2 ataques por Ação)');
      if (level === 6) features.push('Melhorias de Inimigo Favorito e Explorador Natural');
      if (level === 7) features.push('Característica de Arquétipo de Patrulheiro (Nível 7)');
      if (level === 8) features.push('Caminho da Floresta (Ignora terreno difícil não mágico)');
      if (level === 10) features.push('Mimetismo (Ocultar-se à Vista)');
      if (level === 11) features.push('Característica de Arquétipo de Patrulheiro (Nível 11)');
      if (level === 14) features.push('Desaparecer (Esconder-se como Ação Bônus)');
      if (level === 15) features.push('Característica de Arquétipo de Patrulheiro (Nível 15)');
      if (level === 18) features.push('Sentidos Selvagens');
      if (level === 20) features.push('Matador de Inimigos (+Mod. SAB no ataque ou dano)');
      break;
    }
    case 'Sorcerer': {
      if (level === 1) features.push('Conjuração Inata', 'Origem da Feitiçaria (Subclasse)');
      if (level === 2) features.push(`Fonte de Magia (${level} Pontos de Feitiçaria)`);
      if (level === 3) features.push('Metamagia (Escolha 2 opções: Magia Acelerada / Magia Gêmea / Magia Sutil / Magia Cuidadosa)');
      if (level === 6) features.push('Característica da Origem da Feitiçaria (Nível 6)');
      if (level === 10) features.push('Metamagia Adicional (3ª opção)');
      if (level === 14) features.push('Característica da Origem da Feitiçaria (Nível 14 — Asas Dracônicas / Explosão Controlada)');
      if (level === 17) features.push('Metamagia Suprema (4ª opção)', 'Acesso ao 9º Círculo');
      if (level === 18) features.push('Característica da Origem da Feitiçaria (Nível 18)');
      if (level === 20) features.push('Restauração do Feiticeiro (Recupera 4 Pontos de Feitiçaria em Descanso Curto)');
      break;
    }
    case 'Warlock': {
      if (level === 1) features.push('Patrono Transcendental (Subclasse)', 'Magia de Pacto (Recupera em Descanso Curto)');
      if (level === 2) features.push('Invocações Místicas (2 invocações ativas: Rajada Agonizante / Visão Diabólica)');
      if (level === 3) features.push('Dádiva do Pacto (Pacto da Corrente / Lâmina / Tomo)');
      if (level === 6) features.push('Característica do Patrono Transcendental (Nível 6)');
      if (level === 10) features.push('Característica do Patrono Transcendental (Nível 10)');
      if (level === 11) features.push('Arcana Mística (Magia de 6º Círculo 1x/dia sem gastar espaço)', '3º Espaço de Pacto');
      if (level === 13) features.push('Arcana Mística (Magia de 7º Círculo 1x/dia)');
      if (level === 14) features.push('Característica do Patrono Transcendental (Nível 14)');
      if (level === 15) features.push('Arcana Mística (Magia de 8º Círculo 1x/dia)');
      if (level === 17) features.push('Arcana Mística (Magia de 9º Círculo 1x/dia)', '4º Espaço de Pacto');
      if (level === 20) features.push('Mestre Místico (Recupera todos os espaços de Pacto em 1 minuto)');
      break;
    }
  }

  if (features.length === 0) {
    features.push(`Progressão de Nível ${level} (Aumento de Dados de Vida, Magias e Recursos de Classe)`);
  }

  return features;
}

/**
 * Estrutura de Decisão de 1 Nível específico (para o Nível 2 até o Nível 20)
 */
export interface LevelUpStepChoice {
  level: number;
  hpMode: 'fixed_average' | 'rolled';
  hpDieRoll: number; // Valor do dado (ex: 5 na média fixa do d8, ou 1..8 na rolagem)
  asiMode?: 'plus_two_one' | 'plus_one_two';
  asiPrimaryStat?: AbilityName;
  asiSecondaryStat?: AbilityName;
  subclassSelected?: string;
  newSpellAdded?: string;
}
