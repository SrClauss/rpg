import { useState } from 'react';
import type { FC } from 'react';
import type { CharacterSheet } from '../../../../types/character';
import { calculateAbilityModifier } from '../../../../types/character';
import { Trees, RefreshCw, PawPrint } from 'lucide-react';

interface DruidHUDProps {
  character: CharacterSheet;
  onConsumeAction?: (costType: 'action' | 'bonus_action' | 'reaction' | 'free', label: string) => void;
  onNotify?: (message: string, type?: 'info' | 'success' | 'warning') => void;
}

interface BeastForm {
  name: string;
  hp: number;
  armorClass: number;
  attackName: string;
  attackBonus: string;
  damage: string;
  special: string;
}

const BEAST_OPTIONS: BeastForm[] = [
  {
    name: 'Lobo Cinzento',
    hp: 11,
    armorClass: 13,
    attackName: 'Mordida Voraz',
    attackBonus: '+4 para acertar',
    damage: '2d4 + 2 perfurante',
    special: 'Alvo deve passar em Salvaguarda de FOR CD 11 ou cairá prostrado.',
  },
  {
    name: 'Urso Pardo',
    hp: 34,
    armorClass: 11,
    attackName: 'Garras e Mordida',
    attackBonus: '+5 para acertar',
    damage: '2d6 + 4 cortante',
    special: 'Multiataque (Mordida + Garras brutais).',
  },
  {
    name: 'Aranha Gigante',
    hp: 26,
    armorClass: 14,
    attackName: 'Picada Venenosa',
    attackBonus: '+5 para acertar',
    damage: '1d8 + 3 perfurante + 2d8 veneno',
    special: 'Dispara Teia (Recarga 5-6) para conter o oponente.',
  },
];

export const DruidHUD: FC<DruidHUDProps> = ({
  character,
  onConsumeAction,
  onNotify,
}) => {
  const [wildShapeUsed, setWildShapeUsed] = useState<number>(0);
  const [currentBeast, setCurrentBeast] = useState<BeastForm | null>(null);
  const [beastHpCurrent, setBeastHpCurrent] = useState<number>(0);

  const wisMod = calculateAbilityModifier(character.abilities.wisdom);
  const preparedMax = Math.max(1, wisMod + character.level);
  const totalWildShape = 2;

  const handleTransform = (beast: BeastForm) => {
    if (!currentBeast && wildShapeUsed >= totalWildShape) {
      onNotify?.('Você esgotou os 2 usos de Forma Selvagem! Descanse para recarregar.', 'warning');
      return;
    }
    if (!currentBeast) {
      setWildShapeUsed((u) => u + 1);
    }
    setCurrentBeast(beast);
    setBeastHpCurrent(beast.hp);
    onConsumeAction?.(
      'action',
      `Forma Selvagem: ${beast.name} (+${beast.hp} PV de besta)`
    );
  };

  const handleRevert = () => {
    setCurrentBeast(null);
    onConsumeAction?.('bonus_action', 'Reverteu Forma Selvagem para humanoide');
  };

  return (
    <div
      style={{
        backgroundColor: '#0d1a14',
        border: currentBeast ? '2px solid #10b981' : '1px solid #064e3b',
        borderRadius: '10px',
        padding: '14px 18px',
        boxShadow: currentBeast ? '0 0 20px rgba(16, 185, 129, 0.25)' : 'none',
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
        {/* Left: Druid status & active beast */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <div
            style={{
              width: '42px',
              height: '42px',
              borderRadius: '8px',
              backgroundColor: currentBeast ? '#065f46' : '#022c22',
              border: '1px solid #10b981',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: '#6ee7b7',
            }}
          >
            <Trees size={24} />
          </div>

          <div>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <span
                className="rpg-title"
                style={{ fontWeight: 'bold', fontSize: '15px', color: '#6ee7b7' }}
              >
                MAGIA NATURAL & FORMA SELVAGEM (DRUIDA)
              </span>
              <span
                style={{
                  fontSize: '11px',
                  padding: '2px 8px',
                  borderRadius: '12px',
                  backgroundColor: currentBeast ? '#065f46' : '#042f2e',
                  color: currentBeast ? '#fff' : '#5eead4',
                  fontWeight: 'bold',
                }}
              >
                {currentBeast
                  ? `FORMA: ${currentBeast.name.toUpperCase()} (${beastHpCurrent}/${currentBeast.hp} PV)`
                  : 'FORMA HUMANOIDE'}
              </span>
            </div>

            <div style={{ fontSize: '12px', color: '#a7f3d0', marginTop: '2px' }}>
              Forma Selvagem: <strong>{totalWildShape - wildShapeUsed} / {totalWildShape}</strong> disponíveis •
              Magias Preparadas: <strong>{preparedMax}</strong> (SAB {wisMod >= 0 ? `+${wisMod}` : wisMod} + Nível {character.level})
            </div>
          </div>
        </div>

        {/* Buttons: Transform options */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px', flexWrap: 'wrap' }}>
          {currentBeast ? (
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <span style={{ fontSize: '12px', color: '#6ee7b7' }}>
                Ataque: {currentBeast.attackName} ({currentBeast.attackBonus}, {currentBeast.damage})
              </span>
              <button
                onClick={handleRevert}
                style={{
                  padding: '8px 12px',
                  borderRadius: '6px',
                  backgroundColor: '#7f1d1d',
                  border: '1px solid #ef4444',
                  color: '#fff',
                  fontSize: '11px',
                  fontWeight: 'bold',
                  display: 'flex',
                  alignItems: 'center',
                  gap: '4px',
                  cursor: 'pointer',
                }}
              >
                <RefreshCw size={13} /> Reverter Forma
              </button>
            </div>
          ) : (
            BEAST_OPTIONS.map((b) => (
              <button
                key={b.name}
                onClick={() => handleTransform(b)}
                style={{
                  padding: '8px 12px',
                  borderRadius: '6px',
                  backgroundColor: '#064e3b',
                  border: '1px solid #059669',
                  color: '#ecfdf5',
                  fontSize: '11px',
                  fontWeight: 'bold',
                  display: 'flex',
                  alignItems: 'center',
                  gap: '4px',
                  cursor: 'pointer',
                }}
                title={`${b.name}: ${b.hp} PV, CA ${b.armorClass}. ${b.special}`}
              >
                <PawPrint size={13} /> {b.name} ({b.hp} PV)
              </button>
            ))
          )}
        </div>
      </div>
    </div>
  );
};
