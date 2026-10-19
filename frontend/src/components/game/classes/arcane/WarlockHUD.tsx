import { useState } from 'react';
import type { FC } from 'react';
import type { CharacterSheet } from '../../../../types/character';
import { Eye, Skull, RefreshCw } from 'lucide-react';

interface WarlockHUDProps {
  character: CharacterSheet;
}

export const WarlockHUD: FC<WarlockHUDProps> = ({ character }) => {
  const patron = character.classResources.warlock?.otherworldlyPatron || 'O Corruptor (The Fiend)';
  const pactBoon = character.classResources.warlock?.pactBoon || 'Tome';
  const invocations = character.classResources.warlock?.invocationsActive || [
    'Visão Diabólica',
    'Rajada Agonizante',
  ];

  const [darkOnesBlessingHp, setDarkOnesBlessingHp] = useState<number>(0);

  const handleDarkBlessing = () => {
    const tempHp = 1 + Math.max(1, Math.floor((character.abilities.charisma - 10) / 2));
    setDarkOnesBlessingHp(tempHp);
    alert(`Bênção do Obscuro ativada! Ao abater um inimigo, o Patrono concedeu ${tempHp} Pontos de Vida Temporários!`);
  };

  return (
    <div
      style={{
        backgroundColor: '#160c20',
        border: '1px solid #581c87',
        borderRadius: '10px',
        padding: '14px 18px',
        boxShadow: '0 0 15px rgba(139, 92, 246, 0.2)',
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
        {/* Left: Warlock pact identity */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <div
            style={{
              width: '42px',
              height: '42px',
              borderRadius: '8px',
              backgroundColor: '#3b0764',
              border: '1px solid #7e22ce',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: '#d8b4fe',
            }}
          >
            <Eye size={24} />
          </div>

          <div>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <span
                className="rpg-title"
                style={{ fontWeight: 'bold', fontSize: '15px', color: '#d8b4fe' }}
              >
                MAGIA DE PACTO & INVOCAÇÕES (BRUXO)
              </span>
              <span
                style={{
                  fontSize: '11px',
                  padding: '2px 8px',
                  borderRadius: '12px',
                  backgroundColor: '#581c87',
                  color: '#fff',
                  fontWeight: 'bold',
                }}
              >
                Patrono: {patron}
              </span>
            </div>

            <div style={{ fontSize: '12px', color: '#c084fc', marginTop: '2px' }}>
              Dádiva do Pacto: <strong>Pacto do {pactBoon}</strong> • Invocações:{' '}
              <strong>{invocations.join(', ')}</strong>
            </div>
          </div>
        </div>

        {/* Action buttons: Eldritch abilities */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <button
            onClick={handleDarkBlessing}
            style={{
              padding: '8px 12px',
              borderRadius: '6px',
              backgroundColor: '#6b21a8',
              border: '1px solid #9333ea',
              color: '#fff',
              fontSize: '11px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '4px',
              cursor: 'pointer',
            }}
            title="Ao reduzir criatura a 0 PV, ganha PV temporários"
          >
            <Skull size={13} /> Bênção do Obscuro ({darkOnesBlessingHp > 0 ? `${darkOnesBlessingHp} Temp HP` : 'Ativar'})
          </button>

          <div
            style={{
              padding: '6px 10px',
              borderRadius: '6px',
              backgroundColor: '#2e1065',
              border: '1px solid #581c87',
              fontSize: '11px',
              color: '#e9d5ff',
              display: 'flex',
              alignItems: 'center',
              gap: '4px',
            }}
            title="Slots de Pacto sempre conjuram no nível máximo e voltam em descanso curto"
          >
            <RefreshCw size={13} color="#c084fc" />
            <span>Recarga em Descanso Curto</span>
          </div>
        </div>
      </div>
    </div>
  );
};
