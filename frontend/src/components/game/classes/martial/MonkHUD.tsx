import { useState } from 'react';
import type { FC } from 'react';
import type { CharacterSheet } from '../../../../types/character';
import { Zap, Shield, Wind, Sparkles } from 'lucide-react';

interface MonkHUDProps {
  character: CharacterSheet;
  onSpendKi?: (amount: number, ability: string) => void;
  onConsumeAction?: (costType: 'action' | 'bonus_action' | 'reaction' | 'free', label: string) => void;
  onNotify?: (message: string, type?: 'info' | 'success' | 'warning') => void;
}

export const MonkHUD: FC<MonkHUDProps> = ({
  character,
  onSpendKi,
  onConsumeAction,
  onNotify,
}) => {
  const totalKi = Math.max(1, character.level);
  const [kiUsed, setKiUsed] = useState<number>(0);

  const kiRemaining = totalKi - kiUsed;
  const martialDie = character.level >= 17 ? '1d10' : character.level >= 11 ? '1d8' : character.level >= 5 ? '1d6' : '1d4';

  const spendKi = (amount: number, abilityName: string) => {
    if (kiRemaining < amount) {
      onNotify?.(`Pontos de Ki insuficientes! Você precisa de ${amount} ponto(s) de Ki.`, 'warning');
      return;
    }
    setKiUsed((k) => k + amount);
    onSpendKi?.(amount, abilityName);
    onConsumeAction?.('bonus_action', `${abilityName} (${amount} Ki)`);
  };

  const handleRestKi = () => {
    setKiUsed(0);
    onNotify?.('Descanso concluído! Todos os Pontos de Ki foram recuperados.', 'success');
  };

  return (
    <div
      style={{
        backgroundColor: '#0f171e',
        border: '1px solid #164e63',
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
        {/* Left: Monk Ki Pool & Martial Arts */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <div
            style={{
              width: '42px',
              height: '42px',
              borderRadius: '8px',
              backgroundColor: '#155e75',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: '#67e8f9',
            }}
          >
            <Zap size={24} />
          </div>

          <div>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <span
                className="rpg-title"
                style={{ fontWeight: 'bold', fontSize: '15px', color: '#67e8f9' }}
              >
                DISCIPLINA DE KI & ARTES MARCIAIS (MONGE)
              </span>
              <span
                style={{
                  fontSize: '11px',
                  padding: '2px 8px',
                  borderRadius: '12px',
                  backgroundColor: '#164e63',
                  color: '#cffafe',
                  fontWeight: 'bold',
                }}
              >
                Dado Desarmado: {martialDie}
              </span>
            </div>

            <div style={{ fontSize: '12px', color: '#a5f3fc', marginTop: '2px' }}>
              Pontos de Ki: <strong>{kiRemaining} / {totalKi}</strong> disponíveis • Defesa Sem Armadura: 10 + DES + SAB
            </div>
          </div>
        </div>

        {/* Ki Ability Shortcuts */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <button
            onClick={() => spendKi(1, 'Rajada de Golpes (Flurry of Blows)')}
            style={{
              padding: '8px 12px',
              borderRadius: '6px',
              backgroundColor: '#0e7490',
              color: '#fff',
              fontSize: '11px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '4px',
              cursor: 'pointer',
            }}
            title="1 Ki: 2 ataques desarmados extras como Ação Bônus"
          >
            <Sparkles size={13} /> Rajada de Golpes (1 Ki)
          </button>

          <button
            onClick={() => spendKi(1, 'Defesa Paciente (Patient Defense)')}
            style={{
              padding: '8px 12px',
              borderRadius: '6px',
              backgroundColor: '#0369a1',
              color: '#fff',
              fontSize: '11px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '4px',
              cursor: 'pointer',
            }}
            title="1 Ki: Esquivar como Ação Bônus"
          >
            <Shield size={13} /> Defesa Paciente (1 Ki)
          </button>

          <button
            onClick={() => spendKi(1, 'Passo do Vento (Step of the Wind)')}
            style={{
              padding: '8px 12px',
              borderRadius: '6px',
              backgroundColor: '#1e40af',
              color: '#fff',
              fontSize: '11px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '4px',
              cursor: 'pointer',
            }}
            title="1 Ki: Desengajar ou Disparada como Ação Bônus e dobro de salto"
          >
            <Wind size={13} /> Passo do Vento (1 Ki)
          </button>

          <button
            onClick={handleRestKi}
            style={{
              padding: '8px 10px',
              borderRadius: '6px',
              backgroundColor: '#1f2937',
              border: '1px solid #374151',
              color: '#9ca3af',
              fontSize: '11px',
              cursor: 'pointer',
            }}
            title="Recupera Ki em Descanso Curto"
          >
            Recarregar Ki
          </button>
        </div>
      </div>
    </div>
  );
};
