import { useState } from 'react';
import type { FC } from 'react';
import type { CharacterSheet } from '../../../../types/character';
import { calculateAbilityModifier } from '../../../../types/character';
import { Music, HeartHandshake, Mic2 } from 'lucide-react';

interface BardHUDProps {
  character: CharacterSheet;
}

export const BardHUD: FC<BardHUDProps> = ({ character }) => {
  const chaMod = calculateAbilityModifier(character.abilities.charisma);
  const totalInspiration = Math.max(1, chaMod);

  const [inspirationUsed, setInspirationUsed] = useState<number>(0);
  const inspirationRemaining = totalInspiration - inspirationUsed;
  const inspirationDie = character.level >= 15 ? 'd12' : character.level >= 10 ? 'd10' : character.level >= 5 ? 'd8' : 'd6';

  const handleInspireAlly = () => {
    if (inspirationRemaining <= 0) {
      alert('Você esgotou seus dados de Inspiração de Bardo! Descanse para recuperar.');
      return;
    }
    setInspirationUsed((u) => u + 1);
    alert(`Inspiração de Bardo concedida! Um aliado a até 60ft ganhou um dado de +1${inspirationDie} para somar a um teste de atributo, ataque ou salvaguarda nos próximos 10 minutos!`);
  };

  const handleSongOfRest = () => {
    alert(`Canção de Descanso tocada! Todos os aliados que gastarem dados de vida no descanso curto recuperam +1d6 pontos de vida adicionais.`);
  };

  return (
    <div
      style={{
        backgroundColor: '#1c0f18',
        border: '1px solid #831843',
        borderRadius: '10px',
        padding: '14px 18px',
        boxShadow: '0 0 15px rgba(236, 72, 153, 0.15)',
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
        {/* Left: Bard identity */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <div
            style={{
              width: '42px',
              height: '42px',
              borderRadius: '8px',
              backgroundColor: '#500724',
              border: '1px solid #db2777',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: '#fbcfe8',
            }}
          >
            <Music size={24} />
          </div>

          <div>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <span
                className="rpg-title"
                style={{ fontWeight: 'bold', fontSize: '15px', color: '#fbcfe8' }}
              >
                INSPIRAÇÃO DE BARDO & VERSATILIDADE (BARDO)
              </span>
              <span
                style={{
                  fontSize: '11px',
                  padding: '2px 8px',
                  borderRadius: '12px',
                  backgroundColor: '#831843',
                  color: '#fff',
                  fontWeight: 'bold',
                }}
              >
                Dado de Inspiração: 1{inspirationDie}
              </span>
            </div>

            <div style={{ fontSize: '12px', color: '#f472b6', marginTop: '2px' }}>
              Dados de Inspiração: <strong>{inspirationRemaining} / {totalInspiration}</strong> disponíveis (Mod. CAR) • Foco: <strong>Instrumento Musical</strong>
            </div>
          </div>
        </div>

        {/* Action buttons: Inspire and Song of Rest */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <button
            disabled={inspirationRemaining <= 0}
            onClick={handleInspireAlly}
            style={{
              padding: '8px 14px',
              borderRadius: '6px',
              backgroundColor: inspirationRemaining > 0 ? '#db2777' : '#1e242d',
              border: '1px solid #f472b6',
              color: inspirationRemaining > 0 ? '#fff' : '#64748b',
              fontSize: '11px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '4px',
              cursor: inspirationRemaining > 0 ? 'pointer' : 'not-allowed',
            }}
            title="Ação Bônus: Inspira um aliado a até 60ft"
          >
            <HeartHandshake size={13} /> Inspirar Aliado (+1{inspirationDie})
          </button>

          <button
            onClick={handleSongOfRest}
            style={{
              padding: '8px 12px',
              borderRadius: '6px',
              backgroundColor: '#4c0519',
              border: '1px solid #9d174d',
              color: '#fce7f3',
              fontSize: '11px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '4px',
              cursor: 'pointer',
            }}
            title="Cura +1d6 extra no descanso curto"
          >
            <Mic2 size={13} /> Canção de Descanso (+1d6)
          </button>
        </div>
      </div>
    </div>
  );
};
