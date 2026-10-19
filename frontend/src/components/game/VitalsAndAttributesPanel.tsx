import type { FC } from 'react';
import type { CharacterSheet, AbilityName } from '../../types/character';
import { calculateAbilityModifier, formatModifier } from '../../types/character';
import {
  Heart,
  Shield,
  Zap,
  Footprints,
  Sparkles,
  Plus,
  Minus,
} from 'lucide-react';

interface VitalsAndAttributesPanelProps {
  character: CharacterSheet;
  onUpdateHp: (delta: number) => void;
  onToggleSpellSlot: (level: number, isPact?: boolean) => void;
}

const ABILITY_LABELS: Record<AbilityName, string> = {
  strength: 'FOR',
  dexterity: 'DES',
  constitution: 'CON',
  intelligence: 'INT',
  wisdom: 'SAB',
  charisma: 'CAR',
};

export const VitalsAndAttributesPanel: FC<VitalsAndAttributesPanelProps> = ({
  character,
  onUpdateHp,
  onToggleSpellSlot,
}) => {
  const hpPercent = Math.min(
    100,
    Math.max(0, (character.vitals.hpCurrent / character.vitals.hpMax) * 100)
  );

  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        gap: '16px',
        backgroundColor: 'var(--bg-card)',
        border: '1px solid var(--border-color)',
        borderRadius: '12px',
        padding: '16px',
      }}
    >
      {/* Vitals Bar: HP, AC, Init, Speed */}
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(130px, 1fr))',
          gap: '10px',
        }}
      >
        {/* HP Box */}
        <div
          style={{
            backgroundColor: '#13161c',
            border: '1px solid rgba(239, 68, 68, 0.3)',
            borderRadius: '8px',
            padding: '10px',
            display: 'flex',
            flexDirection: 'column',
            justifyContent: 'space-between',
          }}
        >
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'space-between',
              marginBottom: '4px',
            }}
          >
            <span
              style={{
                fontSize: '11px',
                fontWeight: 700,
                color: 'var(--accent-crimson)',
                display: 'flex',
                alignItems: 'center',
                gap: '4px',
              }}
            >
              <Heart size={14} /> HP VITAL
            </span>
            <div style={{ display: 'flex', gap: '4px' }}>
              <button
                onClick={() => onUpdateHp(-1)}
                title="Dano -1"
                style={{
                  width: '20px',
                  height: '20px',
                  borderRadius: '4px',
                  backgroundColor: '#261b1e',
                  color: 'var(--accent-crimson)',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                }}
              >
                <Minus size={12} />
              </button>
              <button
                onClick={() => onUpdateHp(1)}
                title="Cura +1"
                style={{
                  width: '20px',
                  height: '20px',
                  borderRadius: '4px',
                  backgroundColor: '#1b2621',
                  color: 'var(--accent-emerald)',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                }}
              >
                <Plus size={12} />
              </button>
            </div>
          </div>

          <div
            style={{
              fontSize: '20px',
              fontWeight: 'bold',
              color: '#fff',
              textAlign: 'center',
              margin: '2px 0',
            }}
          >
            {character.vitals.hpCurrent} / {character.vitals.hpMax}
          </div>

          {/* Health Bar */}
          <div
            style={{
              width: '100%',
              height: '6px',
              backgroundColor: '#262c37',
              borderRadius: '3px',
              overflow: 'hidden',
            }}
          >
            <div
              style={{
                width: `${hpPercent}%`,
                height: '100%',
                backgroundColor:
                  hpPercent > 50
                    ? 'var(--accent-emerald)'
                    : hpPercent > 25
                    ? 'var(--accent-gold)'
                    : 'var(--accent-crimson)',
                transition: 'width 0.2s',
              }}
            />
          </div>
        </div>

        {/* AC */}
        <div
          style={{
            backgroundColor: '#13161c',
            border: '1px solid rgba(2, 132, 199, 0.3)',
            borderRadius: '8px',
            padding: '10px',
            textAlign: 'center',
          }}
        >
          <span
            style={{
              fontSize: '11px',
              fontWeight: 700,
              color: 'var(--accent-sky)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              gap: '4px',
              marginBottom: '2px',
            }}
          >
            <Shield size={14} /> CLASSE ARMADURA
          </span>
          <div style={{ fontSize: '22px', fontWeight: 'bold', color: '#fff' }}>
            {character.vitals.armorClass}
          </div>
          <span style={{ fontSize: '10px', color: 'var(--text-muted)' }}>
            Base D&D 5e
          </span>
        </div>

        {/* Initiative */}
        <div
          style={{
            backgroundColor: '#13161c',
            border: '1px solid rgba(234, 179, 8, 0.3)',
            borderRadius: '8px',
            padding: '10px',
            textAlign: 'center',
          }}
        >
          <span
            style={{
              fontSize: '11px',
              fontWeight: 700,
              color: 'var(--accent-gold)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              gap: '4px',
              marginBottom: '2px',
            }}
          >
            <Zap size={14} /> INICIATIVA
          </span>
          <div style={{ fontSize: '22px', fontWeight: 'bold', color: '#fff' }}>
            {formatModifier(character.vitals.initiativeModifier)}
          </div>
          <span style={{ fontSize: '10px', color: 'var(--text-muted)' }}>
            Mod. DES
          </span>
        </div>

        {/* Speed */}
        <div
          style={{
            backgroundColor: '#13161c',
            border: '1px solid rgba(16, 185, 129, 0.3)',
            borderRadius: '8px',
            padding: '10px',
            textAlign: 'center',
          }}
        >
          <span
            style={{
              fontSize: '11px',
              fontWeight: 700,
              color: 'var(--accent-emerald)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              gap: '4px',
              marginBottom: '2px',
            }}
          >
            <Footprints size={14} /> DESLOCAMENTO
          </span>
          <div style={{ fontSize: '22px', fontWeight: 'bold', color: '#fff' }}>
            {character.vitals.speedFeet} <span style={{ fontSize: '12px' }}>pés</span>
          </div>
          <span style={{ fontSize: '10px', color: 'var(--text-muted)' }}>
            6 células (5ft)
          </span>
        </div>
      </div>

      {/* Attributes Bar */}
      <div>
        <div
          style={{
            fontSize: '12px',
            fontWeight: 700,
            color: 'var(--text-muted)',
            marginBottom: '8px',
            textTransform: 'uppercase',
            letterSpacing: '0.05em',
          }}
        >
          Atributos & Salvaguardas (Proficiência: +{character.proficiencyBonus})
        </div>

        <div
          style={{
            display: 'grid',
            gridTemplateColumns: 'repeat(6, 1fr)',
            gap: '8px',
          }}
        >
          {(
            [
              'strength',
              'dexterity',
              'constitution',
              'intelligence',
              'wisdom',
              'charisma',
            ] as AbilityName[]
          ).map((ability) => {
            const score = character.abilities[ability];
            const mod = calculateAbilityModifier(score);
            const isProficientSave = character.savingThrowProficiencies.includes(ability);
            const saveBonus = isProficientSave ? mod + character.proficiencyBonus : mod;

            return (
              <div
                key={ability}
                style={{
                  backgroundColor: '#101318',
                  border: isProficientSave
                    ? '1px solid var(--accent-gold)'
                    : '1px solid var(--border-color)',
                  borderRadius: '8px',
                  padding: '8px 4px',
                  textAlign: 'center',
                  position: 'relative',
                }}
              >
                <div
                  style={{
                    fontSize: '10px',
                    fontWeight: 700,
                    color: isProficientSave ? 'var(--accent-gold)' : 'var(--text-muted)',
                  }}
                >
                  {ABILITY_LABELS[ability]}
                </div>
                <div
                  style={{
                    fontSize: '16px',
                    fontWeight: 'bold',
                    color: '#fff',
                    margin: '2px 0',
                  }}
                >
                  {score}
                </div>
                <div
                  style={{
                    fontSize: '12px',
                    fontWeight: 700,
                    color: mod >= 0 ? 'var(--accent-emerald)' : 'var(--accent-crimson)',
                  }}
                >
                  {formatModifier(mod)}
                </div>
                <div
                  style={{
                    fontSize: '9px',
                    color: isProficientSave ? 'var(--accent-gold)' : 'var(--text-faint)',
                    borderTop: '1px dashed #222834',
                    marginTop: '4px',
                    paddingTop: '2px',
                  }}
                  title={`Salvaguarda: ${formatModifier(saveBonus)}`}
                >
                  Save {formatModifier(saveBonus)}
                </div>
              </div>
            );
          })}
        </div>
      </div>

      {/* Spell Slots Used vs. Total (Present for all classes, adapts if non-caster) */}
      <div
        style={{
          backgroundColor: '#101318',
          border: '1px solid var(--border-color)',
          borderRadius: '8px',
          padding: '12px',
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
          <span
            style={{
              fontSize: '12px',
              fontWeight: 700,
              color: 'var(--accent-arcane)',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
            }}
          >
            <Sparkles size={14} /> ESPAÇOS DE MAGIA (USADOS / DISPONÍVEIS)
          </span>
          <span style={{ fontSize: '11px', color: 'var(--text-faint)' }}>
            Clique para gastar / recuperar
          </span>
        </div>

        {character.spellSlots.pactMagic ? (
          /* Warlock Pact Magic Display */
          <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
            <span style={{ fontSize: '12px', color: '#fff', fontWeight: 600 }}>
              Magia de Pacto (Nível {character.spellSlots.pactMagic.slotLevel}):
            </span>
            <div style={{ display: 'flex', gap: '6px' }}>
              {Array.from({ length: character.spellSlots.pactMagic.total }).map((_, i) => {
                const isUsed = i < (character.spellSlots.pactMagic?.used || 0);
                return (
                  <button
                    key={i}
                    onClick={() => onToggleSpellSlot(character.spellSlots.pactMagic?.slotLevel || 1, true)}
                    style={{
                      width: '24px',
                      height: '24px',
                      borderRadius: '4px',
                      backgroundColor: isUsed ? '#2a1b38' : 'var(--accent-arcane)',
                      border: isUsed ? '1px dashed #6b4096' : '1px solid #c084fc',
                      color: isUsed ? '#6b4096' : '#000',
                      fontSize: '10px',
                      fontWeight: 'bold',
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'center',
                    }}
                    title={isUsed ? 'Espaço de Pacto Usado (Clique para recuperar)' : 'Espaço Disponível (Clique para gastar)'}
                  >
                    {isUsed ? '✕' : '●'}
                  </button>
                );
              })}
            </div>
            <span style={{ fontSize: '11px', color: 'var(--text-muted)' }}>
              ({character.spellSlots.pactMagic.total - character.spellSlots.pactMagic.used} restantes)
            </span>
          </div>
        ) : Object.keys(character.spellSlots.levels).length > 0 &&
          Object.values(character.spellSlots.levels).some((l) => l.total > 0) ? (
          /* Standard Spellcaster Display */
          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '12px' }}>
            {Object.values(character.spellSlots.levels)
              .filter((slot) => slot.total > 0)
              .map((slot) => (
                <div
                  key={slot.level}
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: '6px',
                    backgroundColor: '#161922',
                    padding: '4px 8px',
                    borderRadius: '6px',
                    border: '1px solid #232936',
                  }}
                >
                  <span style={{ fontSize: '11px', color: 'var(--text-muted)', fontWeight: 600 }}>
                    {slot.level}º Círculo:
                  </span>
                  <div style={{ display: 'flex', gap: '4px' }}>
                    {Array.from({ length: slot.total }).map((_, i) => {
                      const isUsed = i < slot.used;
                      return (
                        <button
                          key={i}
                          onClick={() => onToggleSpellSlot(slot.level, false)}
                          style={{
                            width: '20px',
                            height: '20px',
                            borderRadius: '4px',
                            backgroundColor: isUsed ? '#261b1e' : 'var(--accent-arcane)',
                            border: isUsed ? '1px dashed #64748b' : '1px solid #c084fc',
                            color: isUsed ? '#64748b' : '#000',
                            fontSize: '9px',
                            fontWeight: 'bold',
                            display: 'flex',
                            alignItems: 'center',
                            justifyContent: 'center',
                          }}
                          title={isUsed ? 'Slot Usado (Clique para recuperar)' : 'Slot Disponível (Clique para gastar)'}
                        >
                          {isUsed ? '✕' : '●'}
                        </button>
                      );
                    })}
                  </div>
                </div>
              ))}
          </div>
        ) : (
          <div style={{ fontSize: '12px', color: 'var(--text-faint)' }}>
            Classe Marcial sem espaços de magia nativos (Usa recursos marciais como Fúria, Ki, Surto de Ação ou Superioridade).
          </div>
        )}
      </div>
    </div>
  );
};
