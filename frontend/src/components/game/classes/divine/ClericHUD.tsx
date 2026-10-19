import { useState } from 'react';
import type { FC } from 'react';
import type { CharacterSheet } from '../../../../types/character';
import { calculateAbilityModifier } from '../../../../types/character';
import { Sun, Sparkles, BookOpen } from 'lucide-react';

interface ClericHUDProps {
  character: CharacterSheet;
}

export const ClericHUD: FC<ClericHUDProps> = ({ character }) => {
  const [channelDivinityUsed, setChannelDivinityUsed] = useState<number>(0);
  const totalChannelDivinity = character.level >= 6 ? 2 : 1;

  const wisMod = calculateAbilityModifier(character.abilities.wisdom);
  const preparedMax = Math.max(1, wisMod + character.level);
  const domain = character.classResources.cleric?.divineDomain || 'Vida';
  const domainSpells = character.classResources.cleric?.domainSpellsAlwaysPrepared || [
    'Bênção',
    'Curar Ferimentos',
  ];

  const handleUseChannelDivinity = (type: 'TurnUndead' | 'Domain') => {
    if (channelDivinityUsed >= totalChannelDivinity) {
      alert('Você já usou todos os seus usos de Canalizar Divindade! Descanse para recuperar.');
      return;
    }
    setChannelDivinityUsed((u) => u + 1);
    if (type === 'TurnUndead') {
      alert('Expulsar Mortos-Vivos ativado! Mortos-vivos a até 30ft devem fazer Salvaguarda de SAB ou fugir por 1 minuto.');
    } else {
      alert(`Canalizar Divindade (${domain}: Preservar a Vida) ativado! Restaura pontos de vida para aliados feridos.`);
    }
  };

  return (
    <div
      style={{
        backgroundColor: '#191710',
        border: '1px solid #78350f',
        borderRadius: '10px',
        padding: '14px 18px',
        boxShadow: '0 0 15px rgba(234, 179, 8, 0.1)',
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
        {/* Left: Cleric domain & divine power */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <div
            style={{
              width: '42px',
              height: '42px',
              borderRadius: '8px',
              backgroundColor: '#451a03',
              border: '1px solid #d97706',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: '#fde68a',
            }}
          >
            <Sun size={24} />
          </div>

          <div>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <span
                className="rpg-title"
                style={{ fontWeight: 'bold', fontSize: '15px', color: '#fde68a' }}
              >
                PODER DIVINO & DOMÍNIO (CLÉRIGO)
              </span>
              <span
                style={{
                  fontSize: '11px',
                  padding: '2px 8px',
                  borderRadius: '12px',
                  backgroundColor: '#78350f',
                  color: '#fef3c7',
                  fontWeight: 'bold',
                }}
              >
                Domínio da {domain}
              </span>
            </div>

            <div style={{ fontSize: '12px', color: '#d97706', marginTop: '2px' }}>
              Canalizar Divindade: <strong>{totalChannelDivinity - channelDivinityUsed} / {totalChannelDivinity}</strong> restantes •
              Magias Preparadas: <strong>{preparedMax}</strong> (SAB {wisMod >= 0 ? `+${wisMod}` : wisMod} + Nível {character.level})
            </div>
          </div>
        </div>

        {/* Action buttons: Channel divinity */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <button
            onClick={() => handleUseChannelDivinity('TurnUndead')}
            style={{
              padding: '8px 12px',
              borderRadius: '6px',
              backgroundColor: '#92400e',
              border: '1px solid #f59e0b',
              color: '#fff',
              fontSize: '11px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '4px',
              cursor: 'pointer',
            }}
            title="Expulsar mortos-vivos num raio de 30ft"
          >
            <Sun size={13} /> Expulsar Mortos-Vivos
          </button>

          <button
            onClick={() => handleUseChannelDivinity('Domain')}
            style={{
              padding: '8px 12px',
              borderRadius: '6px',
              backgroundColor: '#b45309',
              border: '1px solid #fbbf24',
              color: '#fff',
              fontSize: '11px',
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
              gap: '4px',
              cursor: 'pointer',
            }}
            title="Canalizar Divindade do Domínio"
          >
            <Sparkles size={13} /> Preservar Vida (Cura)
          </button>

          <div
            style={{
              padding: '6px 10px',
              borderRadius: '6px',
              backgroundColor: '#291e0a',
              border: '1px solid #78350f',
              fontSize: '11px',
              color: '#fde68a',
              display: 'flex',
              alignItems: 'center',
              gap: '4px',
            }}
            title="Magias de Domínio sempre fixadas na ficha"
          >
            <BookOpen size={13} />
            <span>Domínio: {domainSpells.join(', ')}</span>
          </div>
        </div>
      </div>
    </div>
  );
};
