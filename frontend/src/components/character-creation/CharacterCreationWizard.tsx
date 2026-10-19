import { useState } from 'react';
import type { FC } from 'react';
import type {
  CharacterClassType,
  CharacterSheet,
  AbilityScores,
  SpellSlots,
  ClassSpecificResources,
} from '../../types/character';
import {
  calculateAbilityModifier,
  formatModifier,
} from '../../types/character';
import {
  DND_CLASSES,
  RACES,
  BACKGROUNDS,
  STANDARD_ARRAY,
} from './constants';
import type { ClassMetadata } from './constants';
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
} from 'lucide-react';

interface CharacterCreationWizardProps {
  onCharacterCreated: (sheet: CharacterSheet) => void;
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

export const CharacterCreationWizard: FC<CharacterCreationWizardProps> = ({
  onCharacterCreated,
}) => {
  const [step, setStep] = useState<number>(1);
  const [name, setName] = useState<string>('Valério de Arton');
  const [race, setRace] = useState<string>(RACES[0]);
  const [background, setBackground] = useState<string>(BACKGROUNDS[1].name);
  const [alignment, setAlignment] = useState<string>('Neutro e Bom');
  const [selectedClass, setSelectedClass] = useState<CharacterClassType>('Wizard');

  const [abilities, setAbilities] = useState<AbilityScores>({
    strength: 10,
    dexterity: 14,
    constitution: 13,
    intelligence: 15,
    wisdom: 12,
    charisma: 8,
  });

  const currentClassMeta: ClassMetadata =
    DND_CLASSES.find((c) => c.id === selectedClass) || DND_CLASSES[0];

  const dexMod = calculateAbilityModifier(abilities.dexterity);
  const conMod = calculateAbilityModifier(abilities.constitution);
  const intMod = calculateAbilityModifier(abilities.intelligence);
  const wisMod = calculateAbilityModifier(abilities.wisdom);
  const chaMod = calculateAbilityModifier(abilities.charisma);

  // Initial HP calculation
  const calculatedHp = Math.max(1, currentClassMeta.hitDieValue + conMod);

  // Initial AC calculation based on class unarmored defense rules
  let calculatedAc = 10 + dexMod;
  if (selectedClass === 'Barbarian') {
    calculatedAc = 10 + dexMod + conMod;
  } else if (selectedClass === 'Monk') {
    calculatedAc = 10 + dexMod + wisMod;
  }

  const handleAbilityChange = (ability: keyof AbilityScores, delta: number) => {
    setAbilities((prev) => {
      const nextVal = Math.min(18, Math.max(6, prev[ability] + delta));
      return { ...prev, [ability]: nextVal };
    });
  };

  const handleQuickStandardArray = () => {
    // Intelligently assign Standard Array according to primary class abilities
    const primary = currentClassMeta.primaryAbilities;
    const sortedArray = [...STANDARD_ARRAY]; // [15, 14, 13, 12, 10, 8]

    const newAbilities: AbilityScores = {
      strength: 10,
      dexterity: 10,
      constitution: 12,
      intelligence: 10,
      wisdom: 10,
      charisma: 10,
    };

    if (primary[0]) {
      newAbilities[primary[0]] = sortedArray[0]; // 15
    }
    if (primary[1]) {
      newAbilities[primary[1]] = sortedArray[1]; // 14
    } else {
      newAbilities.constitution = sortedArray[1]; // 14
    }

    // Fill remaining
    const remainingAbilities: (keyof AbilityScores)[] = (
      ['strength', 'dexterity', 'constitution', 'intelligence', 'wisdom', 'charisma'] as (keyof AbilityScores)[]
    ).filter((a) => a !== primary[0] && a !== primary[1]);

    remainingAbilities.forEach((stat, idx) => {
      newAbilities[stat] = sortedArray[2 + idx] || 8;
    });

    setAbilities(newAbilities);
  };

  const buildFinalCharacterSheet = (): CharacterSheet => {
    // Initial spell slots
    const initialSlots: SpellSlots = {
      levels: {
        1: {
          level: 1,
          total: currentClassMeta.isSpellcaster && selectedClass !== 'Warlock' ? 2 : 0,
          used: 0,
        },
      },
      pactMagic:
        selectedClass === 'Warlock'
          ? {
              slotLevel: 1,
              total: 1,
              used: 0,
            }
          : undefined,
    };

    // Class specific initial resources
    const classResources: ClassSpecificResources = {};
    if (selectedClass === 'Barbarian') {
      classResources.barbarian = {
        rageUsesTotal: 2,
        rageUsesCurrent: 2,
        isRaging: false,
        rageDamageBonus: 2,
        unarmoredDefenseFormula: '10 + Mod. DES + Mod. CON',
      };
    } else if (selectedClass === 'Bard') {
      classResources.bard = {
        inspirationDiceTotal: Math.max(1, chaMod),
        inspirationDiceCurrent: Math.max(1, chaMod),
        inspirationDieType: 'd6',
        songOfRestDie: 'd6',
      };
    } else if (selectedClass === 'Cleric') {
      classResources.cleric = {
        channelDivinityTotal: 1,
        channelDivinityCurrent: 1,
        divineDomain: 'Vida',
        domainSpellsAlwaysPrepared: ['Bênção', 'Curar Ferimentos'],
      };
    } else if (selectedClass === 'Druid') {
      classResources.druid = {
        wildShapeTotal: 2,
        wildShapeCurrent: 2,
        isWildShaped: false,
      };
    } else if (selectedClass === 'Fighter') {
      classResources.fighter = {
        secondWindAvailable: true,
        actionSurgeAvailable: false,
        fightingStyle: 'Duelo (+2 dano com 1 arma)',
      };
    } else if (selectedClass === 'Monk') {
      classResources.monk = {
        kiPointsTotal: 0,
        kiPointsCurrent: 0,
        martialArtsDie: '1d4',
        unarmoredDefenseFormula: '10 + Mod. DES + Mod. SAB',
      };
    } else if (selectedClass === 'Paladin') {
      classResources.paladin = {
        layOnHandsPoolTotal: 5,
        layOnHandsPoolCurrent: 5,
        divineSenseUsesTotal: Math.max(1, 1 + chaMod),
        divineSenseUsesCurrent: Math.max(1, 1 + chaMod),
        fightingStyle: 'Defesa (+1 CA)',
        divineSmiteUnlocked: false,
      };
    } else if (selectedClass === 'Ranger') {
      classResources.ranger = {
        favoredEnemies: ['Bestas selvagens'],
        naturalExplorers: ['Floresta'],
        fightingStyle: 'Arquearia (+2 ataque a distância)',
        huntersMarkActive: false,
      };
    } else if (selectedClass === 'Rogue') {
      classResources.rogue = {
        sneakAttackDiceCount: 1,
        cunningActionUnlocked: false,
        expertises: ['Furtividade', 'Prestidigitação'],
      };
    } else if (selectedClass === 'Sorcerer') {
      classResources.sorcerer = {
        sorceryPointsTotal: 0,
        sorceryPointsCurrent: 0,
        metamagicKnown: [],
        originName: 'Linhagem Dracônica',
      };
    } else if (selectedClass === 'Warlock') {
      classResources.warlock = {
        otherworldlyPatron: 'O Corruptor (Fiend)',
        pactBoon: 'Tome',
        invocationsActive: ['Visão Diabólica', 'Rajada Agonizante'],
      };
    } else if (selectedClass === 'Wizard') {
      classResources.wizard = {
        arcaneRecoveryAvailable: true,
        spellbookSpells: [
          'Mísseis Mágicos',
          'Escudo Arcano',
          'Armadura Arcana',
          'Detectar Magia',
          'Onda Trovejante',
          'Dormir',
        ],
        preparedSpellsMax: Math.max(1, intMod + 1),
        preparedSpellsActive: ['Mísseis Mágicos', 'Escudo Arcano'],
      };
    }

    return {
      id: `char-${Date.now()}`,
      name,
      race,
      className: selectedClass,
      level: 1,
      background,
      alignment,
      proficiencyBonus: 2,
      abilities,
      savingThrowProficiencies: currentClassMeta.savingThrows,
      skillProficiencies: BACKGROUNDS.find((b) => b.name === background)?.skills || [
        'Percepção',
      ],
      vitals: {
        hpCurrent: calculatedHp,
        hpMax: calculatedHp,
        hpTemp: 0,
        armorClass: calculatedAc,
        initiativeModifier: dexMod,
        speedFeet: 30,
        hitDiceTotal: 1,
        hitDiceCurrent: 1,
        hitDiceType: currentClassMeta.hitDie,
        deathSaves: { successes: 0, failures: 0 },
        conditions: [],
      },
      spellSlots: initialSlots,
      spellsKnownOrPrepared:
        selectedClass === 'Wizard'
          ? ['Mísseis Mágicos', 'Escudo Arcano']
          : currentClassMeta.isSpellcaster
          ? ['Curar Ferimentos', 'Bênção']
          : [],
      classResources,
      inventory: [
        { name: 'Adaga', quantity: 1, weightLbs: 1, isEquipped: true },
        { name: 'Mochila de Aventureiro', quantity: 1, weightLbs: 5 },
        { name: 'Cantil de Água', quantity: 1, weightLbs: 2 },
        { name: 'Tochas', quantity: 5, weightLbs: 5 },
      ],
    };
  };

  const handleFinish = () => {
    const sheet = buildFinalCharacterSheet();
    onCharacterCreated(sheet);
  };

  return (
    <div style={{ maxWidth: '1100px', margin: '0 auto', padding: '24px 16px' }}>
      {/* Header */}
      <div style={{ textAlign: 'center', marginBottom: '28px' }}>
        <h1
          className="rpg-title"
          style={{
            fontSize: '32px',
            color: 'var(--accent-gold)',
            marginBottom: '8px',
            textShadow: '0 0 15px rgba(234, 179, 8, 0.3)',
          }}
        >
          Criação de Personagem — D&D 5e
        </h1>
        <p style={{ color: 'var(--text-muted)', fontSize: '15px' }}>
          Defina sua identidade, distribua seus atributos e selecione uma das 12 classes originais.
        </p>

        {/* Wizard Steps indicator */}
        <div
          style={{
            display: 'flex',
            justifyContent: 'center',
            gap: '12px',
            marginTop: '20px',
          }}
        >
          {[
            { num: 1, label: '1. Identidade & Raça' },
            { num: 2, label: '2. As 12 Classes Originais' },
            { num: 3, label: '3. Atributos & Cálculos' },
            { num: 4, label: '4. Resumo & Confirmação' },
          ].map((s) => (
            <button
              key={s.num}
              onClick={() => setStep(s.num)}
              style={{
                padding: '6px 14px',
                borderRadius: '20px',
                fontSize: '13px',
                fontWeight: 600,
                backgroundColor: step === s.num ? 'var(--accent-gold)' : 'var(--bg-card)',
                color: step === s.num ? '#000' : 'var(--text-muted)',
                border:
                  step === s.num
                    ? '1px solid var(--accent-gold)'
                    : '1px solid var(--border-color)',
                transition: 'all 0.2s',
              }}
            >
              {s.label}
            </button>
          ))}
        </div>
      </div>

      {/* STEP 1: IDENTITY */}
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
            Passo 1: Identidade do Herói
          </h2>

          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))',
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
                Raça:
              </label>
              <select
                value={race}
                onChange={(e) => setRace(e.target.value)}
                style={{ width: '100%', fontSize: '15px' }}
              >
                {RACES.map((r) => (
                  <option key={r} value={r}>
                    {r}
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

          <div style={{ display: 'flex', justifyContent: 'flex-end', marginTop: '24px' }}>
            <button
              onClick={() => setStep(2)}
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '8px',
                padding: '10px 20px',
                borderRadius: '8px',
                backgroundColor: 'var(--accent-gold)',
                color: '#000',
                fontWeight: 'bold',
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
            }}
          >
            <div>
              <h2 className="rpg-title" style={{ fontSize: '20px', color: '#fff' }}>
                Passo 2: Selecione sua Classe (1 das 12 Classes Oficiais)
              </h2>
              <p style={{ color: 'var(--text-muted)', fontSize: '13px' }}>
                Cada classe possui um cockpit de HUD exclusivo com seus próprios medidores de recursos.
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
              Classe Selecionada: {currentClassMeta.namePt} ({currentClassMeta.id})
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
                    boxShadow: isSelected
                      ? `0 0 15px ${cls.color}33`
                      : 'none',
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
                      {cls.hitDie}
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
              }}
            >
              <ChevronLeft size={18} /> Voltar
            </button>
            <button
              onClick={() => setStep(3)}
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '8px',
                padding: '10px 20px',
                borderRadius: '8px',
                backgroundColor: 'var(--accent-gold)',
                color: '#000',
                fontWeight: 'bold',
              }}
            >
              Distribuir Atributos da Classe <ChevronRight size={18} />
            </button>
          </div>
        </div>
      )}

      {/* STEP 3: ATTRIBUTES & DERIVED STATS */}
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
              alignItems: 'center',
              marginBottom: '16px',
            }}
          >
            <div>
              <h2 className="rpg-title" style={{ fontSize: '20px', color: '#fff' }}>
                Passo 3: Atributos & Cálculos em Tempo Real
              </h2>
              <p style={{ color: 'var(--text-muted)', fontSize: '13px' }}>
                Ajuste os valores ou aplique o Standard Array com foco automático nas habilidades primárias do {currentClassMeta.namePt}.
              </p>
            </div>
            <button
              onClick={handleQuickStandardArray}
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
                fontWeight: 600,
              }}
            >
              <Sparkle size={14} /> Atribuir Standard Array Ideal
            </button>
          </div>

          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fit, minmax(320px, 1fr))',
              gap: '24px',
            }}
          >
            {/* Ability scores adjusters */}
            <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
              {(
                [
                  { key: 'strength', label: 'FORÇA (STR)' },
                  { key: 'dexterity', label: 'DESTREZA (DEX)' },
                  { key: 'constitution', label: 'CONSTITUIÇÃO (CON)' },
                  { key: 'intelligence', label: 'INTELIGÊNCIA (INT)' },
                  { key: 'wisdom', label: 'SABEDORIA (WIS)' },
                  { key: 'charisma', label: 'CARISMA (CHA)' },
                ] as const
              ).map(({ key, label }) => {
                const score = abilities[key];
                const mod = calculateAbilityModifier(score);
                const isPrimary = currentClassMeta.primaryAbilities.includes(key);

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
                      <span
                        style={{
                          fontWeight: 600,
                          fontSize: '13px',
                          color: isPrimary ? 'var(--accent-gold)' : '#fff',
                        }}
                      >
                        {label} {isPrimary && '★ Primário'}
                      </span>
                    </div>

                    <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                      <button
                        onClick={() => handleAbilityChange(key, -1)}
                        style={{
                          width: '28px',
                          height: '28px',
                          borderRadius: '4px',
                          backgroundColor: '#222834',
                          color: '#fff',
                          fontWeight: 'bold',
                        }}
                      >
                        -
                      </button>

                      <div style={{ textAlign: 'center', minWidth: '40px' }}>
                        <span style={{ fontSize: '18px', fontWeight: 'bold', color: '#fff' }}>
                          {score}
                        </span>
                        <div
                          style={{
                            fontSize: '12px',
                            fontWeight: 600,
                            color: mod >= 0 ? 'var(--accent-emerald)' : 'var(--accent-crimson)',
                          }}
                        >
                          {formatModifier(mod)}
                        </div>
                      </div>

                      <button
                        onClick={() => handleAbilityChange(key, 1)}
                        style={{
                          width: '28px',
                          height: '28px',
                          borderRadius: '4px',
                          backgroundColor: '#222834',
                          color: '#fff',
                          fontWeight: 'bold',
                        }}
                      >
                        +
                      </button>
                    </div>
                  </div>
                );
              })}
            </div>

            {/* Derived Stats Live Preview */}
            <div
              style={{
                backgroundColor: '#111419',
                borderRadius: '8px',
                padding: '20px',
                border: '1px solid var(--border-color)',
              }}
            >
              <h3
                className="rpg-title"
                style={{ fontSize: '16px', color: 'var(--accent-gold)', marginBottom: '14px' }}
              >
                Estatísticas Derivadas Calculadas
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
                    <Heart size={16} /> PONTOS DE VIDA (HP)
                  </div>
                  <span style={{ fontSize: '24px', fontWeight: 'bold', color: '#fff' }}>
                    {calculatedHp}
                  </span>
                  <div style={{ fontSize: '11px', color: 'var(--text-muted)' }}>
                    {currentClassMeta.hitDieValue} ({currentClassMeta.hitDie}) + {conMod} CON
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
                      : '10 + DES base'}
                  </div>
                </div>
              </div>

              {/* Special Class Mechanics note */}
              <div
                style={{
                  marginTop: '16px',
                  padding: '12px',
                  borderRadius: '6px',
                  backgroundColor: '#171b22',
                  border: '1px solid #28303d',
                  fontSize: '13px',
                }}
              >
                <div
                  style={{
                    color: currentClassMeta.color,
                    fontWeight: 'bold',
                    marginBottom: '4px',
                  }}
                >
                  Mecânica Central do {currentClassMeta.namePt}:
                </div>
                <p style={{ color: 'var(--text-muted)', fontSize: '12px' }}>
                  {selectedClass === 'Barbarian' &&
                    '2 Usos de Fúria (+2 de dano), Defesa Sem Armadura e dados massivos de d12.'}
                  {selectedClass === 'Bard' &&
                    `Inspiração de Bardo com ${Math.max(1, chaMod)} dados d6 por descanso.`}
                  {selectedClass === 'Cleric' &&
                    'Canalizar Divindade, Domínio Divino com magias fixas e conjuração baseada em Sabedoria.'}
                  {selectedClass === 'Druid' &&
                    'Forma Selvagem para adotar corpos de bestas e magias preparadas naturais.'}
                  {selectedClass === 'Fighter' &&
                    'Retomar o Fôlego (1d10 + 1 de autocura por descanso curto) e proficiência total marcial.'}
                  {selectedClass === 'Monk' &&
                    'Artes Marciais (1d4 de ataque desarmado com bônus de ação) e Ki no nível 2.'}
                  {selectedClass === 'Paladin' &&
                    '5 Pontos de Cura pelas Mãos e Sentido Divino para caçar infernais e mortos-vivos.'}
                  {selectedClass === 'Ranger' &&
                    'Inimigo Favorito e Explorador Natural para domínio de terrenos hostis.'}
                  {selectedClass === 'Rogue' &&
                    'Ataque Furtivo de +1d6 com vantagem e perícias com proficiência duplicada (Expertise).'}
                  {selectedClass === 'Sorcerer' &&
                    'Origem da Feitiçaria inata e preparação para manipulação de Metamagia.'}
                  {selectedClass === 'Warlock' &&
                    'Slots de Pacto que sempre conjuram no nível máximo e recarregam em Descanso Curto.'}
                  {selectedClass === 'Wizard' &&
                    `Grimório arcano com 6 magias e ${Math.max(1, intMod + 1)} magias preparadas para o dia.`}
                </p>
              </div>
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
              }}
            >
              <ChevronLeft size={18} /> Voltar
            </button>
            <button
              onClick={() => setStep(4)}
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '8px',
                padding: '10px 20px',
                borderRadius: '8px',
                backgroundColor: 'var(--accent-gold)',
                color: '#000',
                fontWeight: 'bold',
              }}
            >
              Revisar Ficha <ChevronRight size={18} />
            </button>
          </div>
        </div>
      )}

      {/* STEP 4: SUMMARY & ENTER GAME */}
      {step === 4 && (
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
            Passo 4: Revisão do Herói e Entrada no Mundo
          </h2>

          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))',
              gap: '16px',
              marginBottom: '24px',
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
              <div style={{ color: 'var(--text-muted)', fontSize: '13px' }}>
                {race} — {currentClassMeta.namePt} Nível 1
              </div>
              <div style={{ color: 'var(--text-faint)', fontSize: '12px', marginTop: '4px' }}>
                Antecedente: {background} | {alignment}
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
                <div style={{ fontSize: '11px', color: 'var(--text-muted)' }}>HP Total</div>
              </div>

              <div style={{ textAlign: 'center' }}>
                <span style={{ fontSize: '22px', fontWeight: 'bold', color: 'var(--accent-sky)' }}>
                  {calculatedAc}
                </span>
                <div style={{ fontSize: '11px', color: 'var(--text-muted)' }}>CA</div>
              </div>

              <div style={{ textAlign: 'center' }}>
                <span style={{ fontSize: '22px', fontWeight: 'bold', color: 'var(--accent-emerald)' }}>
                  {formatModifier(dexMod)}
                </span>
                <div style={{ fontSize: '11px', color: 'var(--text-muted)' }}>Iniciativa</div>
              </div>

              <div style={{ textAlign: 'center' }}>
                <span style={{ fontSize: '22px', fontWeight: 'bold', color: 'var(--accent-gold)' }}>
                  +2
                </span>
                <div style={{ fontSize: '11px', color: 'var(--text-muted)' }}>Proficiência</div>
              </div>
            </div>
          </div>

          <div
            style={{
              padding: '16px',
              borderRadius: '8px',
              backgroundColor: 'var(--accent-gold-bg)',
              border: '1px solid var(--accent-gold)',
              marginBottom: '24px',
              textAlign: 'center',
            }}
          >
            <div style={{ fontWeight: 'bold', color: 'var(--accent-gold)', fontSize: '15px' }}>
              Cockpit de Classe:{' '}
              <span style={{ color: currentClassMeta.color }}>{currentClassMeta.namePt}</span>
            </div>
            <p style={{ color: 'var(--text-main)', fontSize: '13px', marginTop: '4px' }}>
              Ao adentrar o jogo, sua interface se transformará no HUD exclusivo do{' '}
              {currentClassMeta.namePt}, liberando o monitor de recursos e as 3 vias de ação no turno.
            </p>
          </div>

          <div style={{ display: 'flex', justifyContent: 'space-between' }}>
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
              }}
            >
              <ChevronLeft size={18} /> Ajustar Atributos
            </button>

            <button
              onClick={handleFinish}
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
              }}
            >
              <Sword size={18} /> Entrar no Jogo com esta Classe
            </button>
          </div>
        </div>
      )}
    </div>
  );
};
