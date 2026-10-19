import { useState } from 'react';
import type { FC } from 'react';
import type { CharacterSheet } from '../../../../types/character';
import { calculateAbilityModifier } from '../../../../types/character';
import { Sword, Heart, Eye, Sparkles } from 'lucide-react';

interface PaladinHUDProps {
  character: CharacterSheet;
  onHealLayOnHands?: (amount: number) => void;
}

export const PaladinHUD: FC<PaladinHUDProps> = ({ character, onHealLayOnHands }) => {
  const maxLayOnHands = character.level * 5;
  const [layOnHandsUsed, setLayOnHandsUsed] = useState<number>(0);
  const [healInput, setHealInput] = useState<number>(5);
  const [smiteActive, setSmiteActive] = useState<boolean>(false);

  const chaMod = calculateAbilityModifier(character.abilities.charisma);
  const layOnHandsRemaining = maxLayOnHands - layOnHandsUsed;

  const handleUseLayOnHands = () => {
    const amount = Number(healInput);
    if (amount <= 0 || amount > layOnHandsRemaining) {
      alert(`Quantidade inválida! Você possui ${layOnHandsRemaining} pontos restantes no pool.`);
      return;
    }
    setLayOnHandsUsed((u) => u + amount);
    onHealLayOnHands?.(amount);
    alert(`Cura pelas Mãos ativada! Restaurou ${amount} PV do alvo.`);
  };

  const handleTriggerSmite = () => {
    const next = !smiteActive;
    setSmiteActive(next);
    if (next) {
      alert('Destruição Divina armada! No próximo golpe bem-sucedido, gastará 1 Espaço de Magia para causar +2d8 de dano radiante (+3d8 contra mortos-vivos/infernais)!');
    }
  };

  return (
    <div
      style={{
        backgroundColor: '#1c1708',
        border: '1px solid #b45309',
        borderRadius: '10px',
        padding: '14px 18px',
        boxShadow: '0 0 15px rgba(245, 158, 11, 0.1)',
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
        {/* Left: Paladin divine oath */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <div
            style={{
              width: '42px',
              height: '42px',
              borderRadius: '8px',
              backgroundColor: '#78350f',
              border: '1px solid #f59e0b',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: '#fef3c7',
            }}
          >
            <Sword size={24} />
          </div>

          <div>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <span
                className="rpg-title"
                style={{ fontWeight: 'bold', fontSize: '15px', color: '#fef3c7' }}
              >
                JURAMENTO SAGRADO & DESTRUIÇÃO (PALADINO)
              </span>
              <span
                style={{
                  fontSize: '11px',
                  padding: '2px 8px',
                  borderRadius: '12px',
                  backgroundColor: smiteActive ? '#991b1b' : '#92400e',
                  color: '#fff',
                  fontWeight: 'bold',
                }}
              >
                {smiteActive ? 'DESTRUIÇÃO DIVINA ARMADA (+2d8)' : 'JURAMENTO DA DEVOÇÃO'}
              </span>
            </div>

            <div style={{ fontSize: '12px', color: '#fcd34d', marginTop: '2px' }}>
              Pool de Cura pelas Mãos: <strong>{layOnHandsRemaining} / {maxLayOnHands}</strong> PV • Sentido Divino: <strong>{Math.max(1, 1 + chaMod)}</strong> usos
            </div>
          </div>
        </div>

        {/* Action buttons: Lay on Hands & Divine Smite */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          {/* Lay on Hands inline form */}
          <div style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
            <input
              type="number"
              min={1}
              max={layOnHandsRemaining}
              value={healInput}
              onChange={(e) => setHealInput(Math.max(1, Math.min(layOnHandsRemaining, Number(e.target.value))))}
              style={{ width: '50px', padding: '6px', fontSize: '12px', textAlign: 'center' }}
            />
            <button
              disabled={layOnHandsRemaining <= 0}
              onClick={handleUseLayOnHands}
              style={{
                padding: '8px 12px',
                borderRadius: '6px',
                backgroundColor: layOnHandsRemaining > 0 ? '#15803d' : '#1f242d',
                border: '1px solid #22c55e',
                color: '#fff',
                fontSize: '11px',
                fontWeight: 'bold',
                display: 'flex',
                alignItems: 'center',
                gap: '4px',
                cursor: layOnHandsRemaining > 0 ? 'pointer' : 'not-allowed',
              }}
            >
              <Heart size={13} /> Curar pelas Mãos
            </button>
          </div>

          <button
            onClick={handleTriggerSmite}
            style={{
              padding: '8px 12px',
              borderRadius: '6px',
              backgroundColor: smiteActive ? '#b91c1c' : '#b45309',
              border: smiteActive ? '1px solid #ef4444' : '1px solid #f59e0b',
              color: '#fff',
              fontSize: '11px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '4px',
              cursor: 'pointer',
            }}
          >
            <Sparkles size={13} /> {smiteActive ? 'Cancelar Smite' : 'Armar Destruição (+2d8)'}
          </button>

          <button
            onClick={() => alert('Sentido Divino ativado! Detectou presença espiritual num raio de 60ft sem barreiras de chumbo.')}
            style={{
              padding: '8px 12px',
              borderRadius: '6px',
              backgroundColor: '#451a03',
              border: '1px solid #78350f',
              color: '#fef3c7',
              fontSize: '11px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '4px',
              cursor: 'pointer',
            }}
          >
            <Eye size={13} /> Sentido Divino
          </button>
        </div>
      </div>
    </div>
  );
};
