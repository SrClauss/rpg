import { useState } from 'react';
import type { FC } from 'react';
import type { CharacterSheet } from '../../../../types/character';
import { Sparkles, Zap, ArrowRightLeft } from 'lucide-react';

interface SorcererHUDProps {
  character: CharacterSheet;
  onRestoreSpellSlots?: (levelsToRecover: number) => void;
  onConsumeAction?: (costType: 'action' | 'bonus_action' | 'reaction' | 'free', label: string) => void;
  onNotify?: (message: string, type?: 'info' | 'success' | 'warning') => void;
}

export const SorcererHUD: FC<SorcererHUDProps> = ({
  character,
  onRestoreSpellSlots,
  onConsumeAction,
  onNotify,
}) => {
  const maxPoints = Math.max(2, character.level);
  const [sorceryPoints, setSorceryPoints] = useState<number>(maxPoints);
  const [activeMetamagic, setActiveMetamagic] = useState<string | null>(null);

  const origin = character.classResources.sorcerer?.originName || 'Linhagem Dracônica';

  const metamagics = [
    { name: 'Magia Acelerada', cost: 2, desc: 'Conjura magia de Ação como Ação Bônus!' },
    { name: 'Magia Duplicada', cost: 1, desc: 'Atinge um segundo alvo no alcance!' },
    { name: 'Magia Sutil', cost: 1, desc: 'Conjura sem componentes verbais ou somáticos!' },
  ];

  const handleApplyMetamagic = (meta: { name: string; cost: number }) => {
    if (activeMetamagic === meta.name) {
      setActiveMetamagic(null);
      setSorceryPoints((p) => Math.min(maxPoints, p + meta.cost));
      onNotify?.(`Metamagia ${meta.name} desativada.`, 'info');
      return;
    }
    if (sorceryPoints < meta.cost) {
      onNotify?.(`Pontos de Feitiçaria insuficientes! ${meta.name} custa ${meta.cost} pontos.`, 'warning');
      return;
    }
    setSorceryPoints((p) => p - meta.cost);
    setActiveMetamagic(meta.name);
    onConsumeAction?.(
      'free',
      `Metamagia: ${meta.name} (${meta.cost} Ponto(s) de Feitiçaria)`
    );
  };

  const handleCreateSlot = () => {
    if (sorceryPoints < 2) {
      onNotify?.('São necessários 2 Pontos de Feitiçaria para criar 1 Espaço de Magia de 1º Círculo.', 'warning');
      return;
    }
    setSorceryPoints((p) => p - 2);
    onRestoreSpellSlots?.(1);
    onConsumeAction?.(
      'bonus_action',
      'Fonte de Magia: Criou 1 Espaço de Magia de 1º Círculo (2 Pts de Feitiçaria)'
    );
  };

  return (
    <div
      style={{
        backgroundColor: '#1b0e24',
        border: '1px solid #701a75',
        borderRadius: '10px',
        padding: '14px 18px',
        boxShadow: '0 0 15px rgba(168, 85, 247, 0.15)',
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
        {/* Left: Sorcerer origin & points */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <div
            style={{
              width: '42px',
              height: '42px',
              borderRadius: '8px',
              backgroundColor: '#701a75',
              border: '1px solid #c026d3',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: '#f5d0fe',
            }}
          >
            <Sparkles size={24} />
          </div>

          <div>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <span
                className="rpg-title"
                style={{ fontWeight: 'bold', fontSize: '15px', color: '#f5d0fe' }}
              >
                FONTE DE FEITIÇARIA & METAMAGIA (FEITICEIRO)
              </span>
              <span
                style={{
                  fontSize: '11px',
                  padding: '2px 8px',
                  borderRadius: '12px',
                  backgroundColor: '#86198f',
                  color: '#fff',
                  fontWeight: 'bold',
                }}
              >
                Origem: {origin}
              </span>
            </div>

            <div style={{ fontSize: '12px', color: '#e879f9', marginTop: '2px' }}>
              Pontos de Feitiçaria: <strong>{sorceryPoints} / {maxPoints}</strong> disponíveis • Metamagia Ativa:{' '}
              <strong>{activeMetamagic || 'Nenhuma'}</strong>
            </div>
          </div>
        </div>

        {/* Buttons: Flexible Casting & Metamagics */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px', flexWrap: 'wrap' }}>
          <button
            onClick={handleCreateSlot}
            style={{
              padding: '8px 12px',
              borderRadius: '6px',
              backgroundColor: '#4a044e',
              border: '1px solid #a21caf',
              color: '#fae8ff',
              fontSize: '11px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '4px',
              cursor: 'pointer',
            }}
            title="Gasta 2 pontos para criar 1 slot de 1º nível"
          >
            <ArrowRightLeft size={13} /> Criar Slot (2 pts)
          </button>

          {metamagics.map((meta) => {
            const isSelected = activeMetamagic === meta.name;
            return (
              <button
                key={meta.name}
                onClick={() => handleApplyMetamagic(meta)}
                style={{
                  padding: '8px 12px',
                  borderRadius: '6px',
                  backgroundColor: isSelected ? '#a21caf' : '#2e1065',
                  border: isSelected ? '1px solid #f0abfc' : '1px solid #7e22ce',
                  color: isSelected ? '#fff' : '#e9d5ff',
                  fontSize: '11px',
                  fontWeight: 'bold',
                  display: 'flex',
                  alignItems: 'center',
                  gap: '4px',
                  cursor: 'pointer',
                }}
                title={meta.desc}
              >
                <Zap size={13} /> {meta.name} ({meta.cost} pt)
              </button>
            );
          })}
        </div>
      </div>
    </div>
  );
};
