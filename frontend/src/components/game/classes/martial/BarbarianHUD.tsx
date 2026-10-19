import { useState } from 'react';
import type { FC } from 'react';
import type { CharacterSheet } from '../../../../types/character';
import { Flame, ShieldAlert, Zap, Skull } from 'lucide-react';

interface BarbarianHUDProps {
  character: CharacterSheet;
  onApplyRageBonus?: (active: boolean) => void;
  onRecklessAttackToggle?: (active: boolean) => void;
}

export const BarbarianHUD: FC<BarbarianHUDProps> = ({
  character,
  onApplyRageBonus,
  onRecklessAttackToggle,
}) => {
  const [isRaging, setIsRaging] = useState<boolean>(false);
  const [ragesUsed, setRagesUsed] = useState<number>(0);
  const [recklessActive, setRecklessActive] = useState<boolean>(false);

  const totalRages = character.classResources.barbarian?.rageUsesTotal || 2;
  const rageBonus = character.classResources.barbarian?.rageDamageBonus || 2;

  const handleToggleRage = () => {
    if (!isRaging) {
      if (ragesUsed >= totalRages) {
        alert('Você já esgotou todos os usos de Fúria hoje! Descanse para recuperar.');
        return;
      }
      setIsRaging(true);
      setRagesUsed((u) => u + 1);
      onApplyRageBonus?.(true);
    } else {
      setIsRaging(false);
      onApplyRageBonus?.(false);
    }
  };

  const handleToggleReckless = () => {
    const next = !recklessActive;
    setRecklessActive(next);
    onRecklessAttackToggle?.(next);
  };

  return (
    <div
      style={{
        backgroundColor: '#161313',
        border: isRaging ? '2px solid #ef4444' : '1px solid #3b2222',
        borderRadius: '10px',
        padding: '14px 18px',
        boxShadow: isRaging ? '0 0 25px rgba(239, 68, 68, 0.4)' : 'none',
        transition: 'all 0.3s ease',
      }}
    >
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          flexWrap: 'wrap',
          gap: '12px',
        }}
      >
        {/* Left: Class identity & Rage trigger */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <div
            style={{
              width: '42px',
              height: '42px',
              borderRadius: '8px',
              backgroundColor: isRaging ? '#ef4444' : '#2b1b1b',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: isRaging ? '#000' : '#ef4444',
              transition: 'all 0.2s',
            }}
          >
            <Flame size={24} />
          </div>

          <div>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <span
                className="rpg-title"
                style={{
                  fontWeight: 'bold',
                  fontSize: '15px',
                  color: isRaging ? '#ef4444' : '#fca5a5',
                }}
              >
                FÚRIA PRIMORDIAL (RAGE)
              </span>
              <span
                style={{
                  fontSize: '11px',
                  padding: '2px 8px',
                  borderRadius: '12px',
                  backgroundColor: isRaging ? 'rgba(239, 68, 68, 0.3)' : '#261a1a',
                  color: isRaging ? '#fff' : '#999',
                  fontWeight: 'bold',
                }}
              >
                {isRaging ? 'FÚRIA ATIVA (+2 dano, resistências)' : 'INATIVA'}
              </span>
            </div>

            <div style={{ fontSize: '12px', color: '#a8a29e', marginTop: '2px' }}>
              Usos Diários: <strong>{totalRages - ragesUsed} / {totalRages}</strong> restantes •
              Bônus de Dano de Fúria: <strong>+{rageBonus}</strong>
            </div>
          </div>
        </div>

        {/* Action buttons */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
          <button
            onClick={handleToggleRage}
            style={{
              padding: '8px 16px',
              borderRadius: '6px',
              backgroundColor: isRaging ? '#450a0a' : '#ef4444',
              border: isRaging ? '1px solid #ef4444' : 'none',
              color: '#fff',
              fontSize: '12px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              cursor: 'pointer',
            }}
          >
            <Zap size={14} />
            {isRaging ? 'Encerrar Fúria' : 'Entrar em Fúria (Ação Bônus)'}
          </button>

          <button
            onClick={handleToggleReckless}
            style={{
              padding: '8px 16px',
              borderRadius: '6px',
              backgroundColor: recklessActive ? '#78350f' : '#1c1917',
              border: recklessActive ? '1px solid #f59e0b' : '1px solid #44403c',
              color: recklessActive ? '#fef3c7' : '#d6d3d1',
              fontSize: '12px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              cursor: 'pointer',
            }}
          >
            <ShieldAlert size={14} />
            Ataque Descuidado: {recklessActive ? 'ATIVO (Vantagem)' : 'DESATIVADO'}
          </button>

          <div
            style={{
              padding: '6px 10px',
              borderRadius: '6px',
              backgroundColor: '#1c1917',
              border: '1px solid #44403c',
              fontSize: '11px',
              color: '#a8a29e',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
            }}
          >
            <Skull size={14} color="#ef4444" />
            <span>Defesa sem Armadura: 10 + DES + CON</span>
          </div>
        </div>
      </div>
    </div>
  );
};
