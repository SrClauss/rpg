import { useState } from 'react';
import type { FC } from 'react';
import type { CharacterSheet } from '../../../../types/character';
import { Compass, Crosshair, Target } from 'lucide-react';

interface RangerHUDProps {
  character: CharacterSheet;
  onConsumeAction?: (costType: 'action' | 'bonus_action' | 'reaction' | 'free', label: string) => void;
  onNotify?: (message: string, type?: 'info' | 'success' | 'warning') => void;
}

export const RangerHUD: FC<RangerHUDProps> = ({
  character,
  onConsumeAction,
  onNotify,
}) => {
  const [huntersMarkActive, setHuntersMarkActive] = useState<boolean>(false);

  const favoredEnemies = character.classResources.ranger?.favoredEnemies || [
    'Bestas Selvagens',
    'Humanoides Hostis',
  ];
  const naturalExplorers = character.classResources.ranger?.naturalExplorers || ['Florestas Ermas'];
  const fightingStyle =
    character.classResources.ranger?.fightingStyle || 'Arquearia (+2 em Ataques a Distância)';

  const handleToggleHuntersMark = () => {
    const next = !huntersMarkActive;
    setHuntersMarkActive(next);
    if (next) {
      onConsumeAction?.('bonus_action', 'Marca do Caçador (+1d6 dano nos acertos)');
    } else {
      onNotify?.('Marca do Caçador removida.', 'info');
    }
  };

  return (
    <div
      style={{
        backgroundColor: '#12180f',
        border: '1px solid #3f6212',
        borderRadius: '10px',
        padding: '14px 18px',
        boxShadow: huntersMarkActive ? '0 0 15px rgba(132, 204, 22, 0.2)' : 'none',
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
        {/* Left: Ranger identity & favored status */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <div
            style={{
              width: '42px',
              height: '42px',
              borderRadius: '8px',
              backgroundColor: '#365314',
              border: '1px solid #65a30d',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: '#d9f99d',
            }}
          >
            <Compass size={24} />
          </div>

          <div>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <span
                className="rpg-title"
                style={{ fontWeight: 'bold', fontSize: '15px', color: '#d9f99d' }}
              >
                PATRULHA & SOBREVIVÊNCIA (PATRULHEIRO)
              </span>
              <span
                style={{
                  fontSize: '11px',
                  padding: '2px 8px',
                  borderRadius: '12px',
                  backgroundColor: huntersMarkActive ? '#4d7c0f' : '#14532d',
                  color: '#fff',
                  fontWeight: 'bold',
                }}
              >
                {huntersMarkActive ? 'MARCA DO CAÇADOR ATIVA (+1d6)' : 'PRESA NÃO MARCADA'}
              </span>
            </div>

            <div style={{ fontSize: '12px', color: '#bef264', marginTop: '2px' }}>
              Inimigo Favorito: <strong>{favoredEnemies.join(', ')}</strong> • Explorador: <strong>{naturalExplorers.join(', ')}</strong>
            </div>
          </div>
        </div>

        {/* Action buttons: Hunter's Mark & Fighting Style */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <button
            onClick={handleToggleHuntersMark}
            style={{
              padding: '8px 14px',
              borderRadius: '6px',
              backgroundColor: huntersMarkActive ? '#3f6212' : '#65a30d',
              border: huntersMarkActive ? '1px solid #84cc16' : 'none',
              color: '#000',
              fontSize: '12px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              cursor: 'pointer',
            }}
          >
            <Target size={14} />
            {huntersMarkActive ? 'Remover Marca' : 'Marcar Alvo (Ação Bônus)'}
          </button>

          <div
            style={{
              padding: '6px 10px',
              borderRadius: '6px',
              backgroundColor: '#1a2e05',
              border: '1px solid #4d7c0f',
              fontSize: '11px',
              color: '#ecfccb',
              display: 'flex',
              alignItems: 'center',
              gap: '4px',
            }}
          >
            <Crosshair size={13} />
            <span>{fightingStyle}</span>
          </div>
        </div>
      </div>
    </div>
  );
};
