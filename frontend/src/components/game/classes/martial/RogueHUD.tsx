import { useState } from 'react';
import type { FC } from 'react';
import type { CharacterSheet } from '../../../../types/character';
import { Crosshair, Wind, EyeOff, ShieldCheck } from 'lucide-react';

interface RogueHUDProps {
  character: CharacterSheet;
  onUseCunningAction?: (actionName: string) => void;
}

export const RogueHUD: FC<RogueHUDProps> = ({ character, onUseCunningAction }) => {
  const [sneakAttackUsedThisTurn, setSneakAttackUsedThisTurn] = useState<boolean>(false);
  const [hasAdvantageOrAlly, setHasAdvantageOrAlly] = useState<boolean>(true);

  const sneakDice = Math.ceil(character.level / 2);
  const expertises = character.classResources.rogue?.expertises || [
    'Furtividade',
    'Prestidigitação',
  ];

  const handleTriggerSneakAttack = () => {
    if (sneakAttackUsedThisTurn) {
      alert('Ataque Furtivo só pode ser aplicado uma vez por turno!');
      return;
    }
    if (!hasAdvantageOrAlly) {
      alert('Requisito do D&D 5e: Você precisa de Vantagem no ataque OU um aliado a 5ft do alvo!');
      return;
    }
    let total = 0;
    const rolls: number[] = [];
    for (let i = 0; i < sneakDice; i++) {
      const r = Math.floor(Math.random() * 6) + 1;
      rolls.push(r);
      total += r;
    }
    setSneakAttackUsedThisTurn(true);
    alert(`ATAQUE FURTIVO CRÍTICO! +${total} de dano perfurante (${sneakDice}d6: [${rolls.join(', ')}])`);
  };

  const handleCunning = (type: 'Dash' | 'Disengage' | 'Hide') => {
    onUseCunningAction?.(type);
    alert(`Ação Ardilosa (${type}) ativada como AÇÃO BÔNUS!`);
  };

  return (
    <div
      style={{
        backgroundColor: '#121417',
        border: '1px solid #374151',
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
        {/* Left: Rogue identity & sneak status */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <div
            style={{
              width: '42px',
              height: '42px',
              borderRadius: '8px',
              backgroundColor: '#1f2937',
              border: '1px solid #4b5563',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: '#d1d5db',
            }}
          >
            <Crosshair size={24} />
          </div>

          <div>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <span
                className="rpg-title"
                style={{ fontWeight: 'bold', fontSize: '15px', color: '#e5e7eb' }}
              >
                ARDILOSIDADE & FURTIVIDADE (LADINO)
              </span>
              <span
                style={{
                  fontSize: '11px',
                  padding: '2px 8px',
                  borderRadius: '12px',
                  backgroundColor: '#374151',
                  color: '#93c5fd',
                  fontWeight: 'bold',
                }}
              >
                Ataque Furtivo: +{sneakDice}d6
              </span>
            </div>

            <div style={{ fontSize: '12px', color: '#9ca3af', marginTop: '2px' }}>
              Especializações (Bônus Duplicado +{character.proficiencyBonus * 2}):{' '}
              <strong>{expertises.join(', ')}</strong>
            </div>
          </div>
        </div>

        {/* Buttons: Sneak attack trigger & Cunning Action shortcuts */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <button
            onClick={() => setHasAdvantageOrAlly((p) => !p)}
            style={{
              padding: '6px 10px',
              borderRadius: '6px',
              backgroundColor: hasAdvantageOrAlly ? '#1e3a5f' : '#222',
              border: hasAdvantageOrAlly ? '1px solid #38bdf8' : '1px solid #444',
              color: hasAdvantageOrAlly ? '#bae6fd' : '#777',
              fontSize: '11px',
              fontWeight: 'bold',
              cursor: 'pointer',
            }}
            title="Alterna condição necessária para Ataque Furtivo"
          >
            {hasAdvantageOrAlly ? '✓ Vantagem/Aliado Válido' : '✕ Sem Vantagem'}
          </button>

          <button
            disabled={sneakAttackUsedThisTurn || !hasAdvantageOrAlly}
            onClick={handleTriggerSneakAttack}
            style={{
              padding: '8px 14px',
              borderRadius: '6px',
              backgroundColor: sneakAttackUsedThisTurn ? '#1f242d' : '#991b1b',
              border: '1px solid #ef4444',
              color: sneakAttackUsedThisTurn ? '#64748b' : '#fff',
              fontSize: '12px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              cursor: sneakAttackUsedThisTurn ? 'not-allowed' : 'pointer',
            }}
          >
            <ShieldCheck size={14} />
            {sneakAttackUsedThisTurn ? 'Furtivo Já Usado' : `Ataque Furtivo (+${sneakDice}d6)`}
          </button>

          {/* Cunning Actions (Bonus action buttons) */}
          <button
            onClick={() => handleCunning('Dash')}
            style={{
              padding: '8px 12px',
              borderRadius: '6px',
              backgroundColor: '#1f2937',
              border: '1px solid #4b5563',
              color: '#f3f4f6',
              fontSize: '11px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '4px',
              cursor: 'pointer',
            }}
            title="Disparada como Ação Bônus"
          >
            <Wind size={13} /> Disparada Bônus
          </button>

          <button
            onClick={() => handleCunning('Disengage')}
            style={{
              padding: '8px 12px',
              borderRadius: '6px',
              backgroundColor: '#1f2937',
              border: '1px solid #4b5563',
              color: '#f3f4f6',
              fontSize: '11px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '4px',
              cursor: 'pointer',
            }}
            title="Desengajar como Ação Bônus"
          >
            <Wind size={13} /> Desengajar Bônus
          </button>

          <button
            onClick={() => handleCunning('Hide')}
            style={{
              padding: '8px 12px',
              borderRadius: '6px',
              backgroundColor: '#1f2937',
              border: '1px solid #4b5563',
              color: '#f3f4f6',
              fontSize: '11px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '4px',
              cursor: 'pointer',
            }}
            title="Esconder-se nas sombras como Ação Bônus"
          >
            <EyeOff size={13} /> Esconder Bônus
          </button>
        </div>
      </div>
    </div>
  );
};
