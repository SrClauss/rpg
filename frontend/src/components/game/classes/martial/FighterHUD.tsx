import { useState } from 'react';
import type { FC } from 'react';
import type { CharacterSheet } from '../../../../types/character';
import { Shield, Heart, Zap, Crosshair } from 'lucide-react';

interface FighterHUDProps {
  character: CharacterSheet;
  onHealSecondWind?: (amount: number) => void;
  onGrantExtraAction?: () => void;
}

export const FighterHUD: FC<FighterHUDProps> = ({
  character,
  onHealSecondWind,
  onGrantExtraAction,
}) => {
  const [secondWindUsed, setSecondWindUsed] = useState<boolean>(false);
  const [actionSurgeUsed, setActionSurgeUsed] = useState<boolean>(false);
  const [superiorityDice, setSuperiorityDice] = useState<number>(4);

  const fightingStyle =
    character.classResources.fighter?.fightingStyle || 'Duelo (+2 de Dano)';

  const handleUseSecondWind = () => {
    if (secondWindUsed) return;
    const roll = Math.floor(Math.random() * 10) + 1;
    const healAmount = roll + character.level;
    setSecondWindUsed(true);
    onHealSecondWind?.(healAmount);
    alert(`Retomar o Fôlego ativado! Recuperou ${healAmount} PV (1d10[${roll}] + ${character.level}).`);
  };

  const handleUseActionSurge = () => {
    if (actionSurgeUsed) return;
    setActionSurgeUsed(true);
    onGrantExtraAction?.();
    alert('Surto de Ação ativado! Você ganhou uma Ação adicional neste turno!');
  };

  const handleSpendSuperiority = () => {
    if (superiorityDice <= 0) return;
    const roll = Math.floor(Math.random() * 8) + 1;
    setSuperiorityDice((d) => d - 1);
    alert(`Dado de Superioridade gasto! Rolou ${roll} (d8) para adicionar à manobra!`);
  };

  return (
    <div
      style={{
        backgroundColor: '#12151b',
        border: '1px solid #2d3748',
        borderRadius: '10px',
        padding: '14px 18px',
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
        {/* Left info */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <div
            style={{
              width: '42px',
              height: '42px',
              borderRadius: '8px',
              backgroundColor: '#1e293b',
              border: '1px solid #475569',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: '#94a3b8',
            }}
          >
            <Shield size={24} />
          </div>

          <div>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <span
                className="rpg-title"
                style={{ fontWeight: 'bold', fontSize: '15px', color: '#94a3b8' }}
              >
                MAESTRIA MARCIAL (GUERREIRO)
              </span>
              <span
                style={{
                  fontSize: '11px',
                  padding: '2px 8px',
                  borderRadius: '12px',
                  backgroundColor: '#1e293b',
                  color: '#38bdf8',
                  fontWeight: 'bold',
                }}
              >
                Estilo: {fightingStyle}
              </span>
            </div>

            <div style={{ fontSize: '12px', color: '#64748b', marginTop: '2px' }}>
              Dados de Superioridade (d8): <strong>{superiorityDice} / 4</strong> disponíveis
            </div>
          </div>
        </div>

        {/* Buttons */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
          <button
            disabled={secondWindUsed}
            onClick={handleUseSecondWind}
            style={{
              padding: '8px 14px',
              borderRadius: '6px',
              backgroundColor: secondWindUsed ? '#1e242d' : '#047857',
              border: '1px solid #10b981',
              color: secondWindUsed ? '#64748b' : '#fff',
              fontSize: '12px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              cursor: secondWindUsed ? 'not-allowed' : 'pointer',
            }}
          >
            <Heart size={14} />
            {secondWindUsed ? 'Fôlego Usado' : 'Retomar o Fôlego (1d10+Nível)'}
          </button>

          <button
            disabled={actionSurgeUsed}
            onClick={handleUseActionSurge}
            style={{
              padding: '8px 14px',
              borderRadius: '6px',
              backgroundColor: actionSurgeUsed ? '#1e242d' : '#b45309',
              border: '1px solid #f59e0b',
              color: actionSurgeUsed ? '#64748b' : '#fff',
              fontSize: '12px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              cursor: actionSurgeUsed ? 'not-allowed' : 'pointer',
            }}
          >
            <Zap size={14} />
            {actionSurgeUsed ? 'Surto Usado' : 'Surto de Ação (+1 Ação)'}
          </button>

          <button
            disabled={superiorityDice <= 0}
            onClick={handleSpendSuperiority}
            style={{
              padding: '8px 14px',
              borderRadius: '6px',
              backgroundColor: superiorityDice <= 0 ? '#1e242d' : '#334155',
              border: '1px solid #64748b',
              color: superiorityDice <= 0 ? '#64748b' : '#f8fafc',
              fontSize: '12px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              cursor: superiorityDice <= 0 ? 'not-allowed' : 'pointer',
            }}
          >
            <Crosshair size={14} />
            Gastar Manobra ({superiorityDice} d8)
          </button>
        </div>
      </div>
    </div>
  );
};
