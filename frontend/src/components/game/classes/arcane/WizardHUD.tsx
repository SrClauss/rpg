import { useState } from 'react';
import type { FC } from 'react';
import type { CharacterSheet } from '../../../../types/character';
import { calculateAbilityModifier } from '../../../../types/character';
import { BookOpen, Sparkles, RefreshCw, CheckCircle2 } from 'lucide-react';

interface WizardHUDProps {
  character: CharacterSheet;
  onUpdatePreparedSpells?: (spells: string[]) => void;
  onRestoreSpellSlots?: (levelsToRecover: number) => void;
  onNotify?: (message: string, type?: 'info' | 'success' | 'warning') => void;
}

export const WizardHUD: FC<WizardHUDProps> = ({
  character,
  onUpdatePreparedSpells,
  onRestoreSpellSlots,
  onNotify,
}) => {
  const intMod = calculateAbilityModifier(character.abilities.intelligence);
  const maxPrepared = Math.max(1, intMod + character.level);

  const [spellbook] = useState<string[]>(
    character.classResources.wizard?.spellbookSpells || [
      'Mísseis Mágicos',
      'Escudo Arcano',
      'Armadura Arcana',
      'Detectar Magia (Ritual)',
      'Onda Trovejante',
      'Dormir',
      'Raio de Fogo (Truque)',
      'Luz (Truque)',
    ]
  );

  const [preparedSpells, setPreparedSpells] = useState<string[]>(
    character.classResources.wizard?.preparedSpellsActive ||
      character.spellsKnownOrPrepared.filter((s) => !s.includes('(Truque)')) || [
        'Mísseis Mágicos',
        'Escudo Arcano',
      ]
  );

  const [arcaneRecoveryUsed, setArcaneRecoveryUsed] = useState<boolean>(false);

  const togglePrepareSpell = (spell: string) => {
    if (spell.includes('(Truque)')) return;
    if (preparedSpells.includes(spell)) {
      const next = preparedSpells.filter((s) => s !== spell);
      setPreparedSpells(next);
      const cantrips = spellbook.filter((s) => s.includes('(Truque)'));
      onUpdatePreparedSpells?.([...cantrips, ...next]);
      onNotify?.(`Magia "${spell}" removida das magias preparadas.`, 'info');
    } else {
      if (preparedSpells.length >= maxPrepared) {
        onNotify?.(
          `Limite atingido! Você pode preparar no máximo ${maxPrepared} magias (Mod. INT ${intMod >= 0 ? `+${intMod}` : intMod} + Nível ${character.level}).`,
          'warning'
        );
        return;
      }
      const next = [...preparedSpells, spell];
      setPreparedSpells(next);
      const cantrips = spellbook.filter((s) => s.includes('(Truque)'));
      onUpdatePreparedSpells?.([...cantrips, ...next]);
      onNotify?.(`Magia "${spell}" preparada no Grimório!`, 'success');
    }
  };

  const handleArcaneRecovery = () => {
    if (arcaneRecoveryUsed) {
      onNotify?.('Recuperação Arcana só pode ser usada 1 vez por dia durante um Descanso Curto.', 'warning');
      return;
    }
    const levelsToRecover = Math.ceil(character.level / 2);
    setArcaneRecoveryUsed(true);
    onRestoreSpellSlots?.(levelsToRecover);
    onNotify?.(
      `Recuperação Arcana ativada! Recuperou até ${levelsToRecover} nível(is) de espaços de magia gastos.`,
      'success'
    );
  };

  return (
    <div
      style={{
        backgroundColor: '#0c1524',
        border: '1px solid #1e3a8a',
        borderRadius: '10px',
        padding: '14px 18px',
        boxShadow: '0 0 15px rgba(59, 130, 246, 0.15)',
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
        {/* Left: Wizard identity */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <div
            style={{
              width: '42px',
              height: '42px',
              borderRadius: '8px',
              backgroundColor: '#1e3a8a',
              border: '1px solid #3b82f6',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: '#bfdbfe',
            }}
          >
            <BookOpen size={24} />
          </div>

          <div>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <span
                className="rpg-title"
                style={{ fontWeight: 'bold', fontSize: '15px', color: '#bfdbfe' }}
              >
                GRIMÓRIO & PREPARAÇÃO ARCANA (MAGO)
              </span>
              <span
                style={{
                  fontSize: '11px',
                  padding: '2px 8px',
                  borderRadius: '12px',
                  backgroundColor: '#1e40af',
                  color: '#fff',
                  fontWeight: 'bold',
                }}
              >
                Preparadas: {preparedSpells.length} / {maxPrepared}
              </span>
            </div>

            <div style={{ fontSize: '12px', color: '#93c5fd', marginTop: '2px' }}>
              Grimório: <strong>{spellbook.length} magias catalogadas</strong> • CD de Magia:{' '}
              <strong>{8 + character.proficiencyBonus + intMod}</strong>
            </div>
          </div>
        </div>

        {/* Arcane Recovery action */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <button
            disabled={arcaneRecoveryUsed}
            onClick={handleArcaneRecovery}
            style={{
              padding: '8px 14px',
              borderRadius: '6px',
              backgroundColor: arcaneRecoveryUsed ? '#1e293b' : '#2563eb',
              border: '1px solid #3b82f6',
              color: arcaneRecoveryUsed ? '#64748b' : '#fff',
              fontSize: '11px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              cursor: arcaneRecoveryUsed ? 'not-allowed' : 'pointer',
            }}
          >
            <RefreshCw size={13} />
            {arcaneRecoveryUsed ? 'Recuperação Usada' : 'Recuperação Arcana (Descanso)'}
          </button>
        </div>
      </div>

      {/* Spellbook & Preparation Interactive Drawer */}
      <div
        style={{
          display: 'flex',
          flexWrap: 'wrap',
          gap: '6px',
          paddingTop: '8px',
          borderTop: '1px dashed #1e3a8a',
        }}
      >
        <span style={{ fontSize: '11px', color: '#60a5fa', fontWeight: 600, width: '100%' }}>
          Clique nas magias do seu Grimório para preparar/despreparar para o dia:
        </span>
        {spellbook.map((spell) => {
          const isPrepared = preparedSpells.includes(spell);
          const isCantrip = spell.includes('(Truque)');

          return (
            <button
              key={spell}
              onClick={() => togglePrepareSpell(spell)}
              style={{
                padding: '4px 10px',
                borderRadius: '6px',
                fontSize: '11px',
                fontWeight: 600,
                backgroundColor: isCantrip
                  ? '#1e293b'
                  : isPrepared
                  ? '#1d4ed8'
                  : '#0f172a',
                border: isPrepared
                  ? '1px solid #60a5fa'
                  : '1px solid #1e293b',
                color: isCantrip ? '#94a3b8' : isPrepared ? '#fff' : '#64748b',
                display: 'inline-flex',
                alignItems: 'center',
                gap: '4px',
                cursor: isCantrip ? 'default' : 'pointer',
              }}
            >
              {isPrepared && <CheckCircle2 size={12} color="#93c5fd" />}
              {isCantrip && <Sparkles size={12} color="#60a5fa" />}
              {spell}
            </button>
          );
        })}
      </div>
    </div>
  );
};
