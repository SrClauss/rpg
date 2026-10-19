import { useState } from 'react';
import type { FC } from 'react';
import type { NarrativeBeat } from '../../types/game';
import {
  Users,
  Grid,
  Scroll,
  MapPin,
} from 'lucide-react';

interface WorldViewPanelProps {
  beat: NarrativeBeat;
  onCellClick?: (x: number, y: number) => void;
  playerPos?: { x: number; y: number };
  enemyPos?: { x: number; y: number };
}

export const WorldViewPanel: FC<WorldViewPanelProps> = ({
  beat,
  onCellClick,
  playerPos = { x: 1, y: 1 },
  enemyPos = { x: 4, y: 3 },
}) => {
  const [activeTab, setActiveTab] = useState<'narrative' | 'tactical'>('narrative');

  // Simple 6x6 grid representing 30x30 feet room
  const gridSize = 6;
  const obstacles = [
    { x: 2, y: 2 },
    { x: 2, y: 3 },
    { x: 3, y: 2 },
  ];

  return (
    <div
      style={{
        backgroundColor: 'var(--bg-card)',
        border: '1px solid var(--border-color)',
        borderRadius: '12px',
        padding: '16px',
        display: 'flex',
        flexDirection: 'column',
        gap: '12px',
      }}
    >
      {/* Beat Header */}
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          borderBottom: '1px solid var(--border-color)',
          paddingBottom: '10px',
        }}
      >
        <div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <span
              style={{
                fontSize: '11px',
                fontWeight: 700,
                padding: '2px 6px',
                borderRadius: '4px',
                backgroundColor: 'var(--accent-gold-bg)',
                color: 'var(--accent-gold)',
              }}
            >
              BEAT #{beat.beatIndex}
            </span>
            <h2 className="rpg-title" style={{ fontSize: '16px', color: '#fff', margin: 0 }}>
              {beat.title}
            </h2>
          </div>
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              fontSize: '12px',
              color: 'var(--text-muted)',
              marginTop: '4px',
            }}
          >
            <MapPin size={13} color="var(--accent-gold)" /> {beat.location}
          </div>
        </div>

        {/* View mode toggle */}
        <div style={{ display: 'flex', gap: '6px' }}>
          <button
            onClick={() => setActiveTab('narrative')}
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: '6px',
              padding: '6px 12px',
              borderRadius: '6px',
              fontSize: '12px',
              fontWeight: 600,
              backgroundColor: activeTab === 'narrative' ? 'var(--accent-gold)' : '#101318',
              color: activeTab === 'narrative' ? '#000' : 'var(--text-muted)',
              border: '1px solid var(--border-color)',
            }}
          >
            <Scroll size={14} /> Narrativa
          </button>
          <button
            onClick={() => setActiveTab('tactical')}
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: '6px',
              padding: '6px 12px',
              borderRadius: '6px',
              fontSize: '12px',
              fontWeight: 600,
              backgroundColor: activeTab === 'tactical' ? 'var(--accent-gold)' : '#101318',
              color: activeTab === 'tactical' ? '#000' : 'var(--text-muted)',
              border: '1px solid var(--border-color)',
            }}
          >
            <Grid size={14} /> Grid Tático 5ft
          </button>
        </div>
      </div>

      {/* Narrative Tab */}
      {activeTab === 'narrative' && (
        <div
          style={{
            backgroundColor: '#101318',
            borderRadius: '8px',
            padding: '16px',
            border: '1px solid var(--border-color)',
            lineHeight: '1.7',
            fontSize: '14px',
            color: '#e2e8f0',
            fontStyle: 'normal',
          }}
        >
          <div style={{ whiteSpace: 'pre-line' }}>{beat.proseContent}</div>

          {/* Active NPCs badge list */}
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              marginTop: '16px',
              paddingTop: '12px',
              borderTop: '1px dashed #222834',
            }}
          >
            <span
              style={{
                fontSize: '11px',
                fontWeight: 600,
                color: 'var(--text-faint)',
                display: 'flex',
                alignItems: 'center',
                gap: '4px',
              }}
            >
              <Users size={13} /> Entidades no Beat:
            </span>
            {beat.activeNpcs.map((npc, idx) => (
              <span
                key={idx}
                style={{
                  fontSize: '11px',
                  backgroundColor: '#1c222d',
                  color: 'var(--accent-sky)',
                  padding: '2px 8px',
                  borderRadius: '4px',
                  border: '1px solid #2d3748',
                }}
              >
                {npc}
              </span>
            ))}
          </div>
        </div>
      )}

      {/* Tactical Grid Tab */}
      {activeTab === 'tactical' && (
        <div
          style={{
            backgroundColor: '#101318',
            borderRadius: '8px',
            padding: '16px',
            border: '1px solid var(--border-color)',
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
          }}
        >
          <div
            style={{
              display: 'flex',
              justifyContent: 'space-between',
              width: '100%',
              marginBottom: '10px',
              fontSize: '12px',
              color: 'var(--text-muted)',
            }}
          >
            <span>Campo de Batalha (Células de 5 pés / 1.5m)</span>
            <span style={{ color: 'var(--accent-emerald)' }}>
              Distância ao Inimigo: 20 pés (Alcance Válido)
            </span>
          </div>

          <div
            style={{
              display: 'grid',
              gridTemplateColumns: `repeat(${gridSize}, 44px)`,
              gridTemplateRows: `repeat(${gridSize}, 44px)`,
              gap: '4px',
              padding: '8px',
              backgroundColor: '#0a0c0f',
              borderRadius: '8px',
              border: '2px solid #222834',
            }}
          >
            {Array.from({ length: gridSize * gridSize }).map((_, index) => {
              const x = index % gridSize;
              const y = Math.floor(index / gridSize);

              const isPlayer = playerPos.x === x && playerPos.y === y;
              const isEnemy = enemyPos.x === x && enemyPos.y === y;
              const isObstacle = obstacles.some((o) => o.x === x && o.y === y);

              return (
                <div
                  key={index}
                  onClick={() => onCellClick?.(x, y)}
                  style={{
                    width: '44px',
                    height: '44px',
                    borderRadius: '4px',
                    backgroundColor: isPlayer
                      ? 'rgba(16, 185, 129, 0.25)'
                      : isEnemy
                      ? 'rgba(239, 68, 68, 0.25)'
                      : isObstacle
                      ? '#2b1b1f'
                      : '#131720',
                    border: isPlayer
                      ? '2px solid var(--accent-emerald)'
                      : isEnemy
                      ? '2px solid var(--accent-crimson)'
                      : isObstacle
                      ? '1px solid #6b2130'
                      : '1px solid #1f2633',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    cursor: 'pointer',
                    fontSize: '11px',
                    fontWeight: 'bold',
                    position: 'relative',
                  }}
                  title={`Célula (${x}, ${y}) - ${
                    isPlayer ? 'Você' : isEnemy ? 'Inimigo' : isObstacle ? 'Obstáculo / Cobertura' : 'Terreno Normal'
                  }`}
                >
                  {isPlayer && <span style={{ color: 'var(--accent-emerald)' }}>PJ</span>}
                  {isEnemy && <span style={{ color: 'var(--accent-crimson)' }}>ORC</span>}
                  {isObstacle && <span style={{ color: '#888', fontSize: '9px' }}>PILLAR</span>}
                </div>
              );
            })}
          </div>

          <div
            style={{
              display: 'flex',
              gap: '16px',
              marginTop: '12px',
              fontSize: '11px',
              color: 'var(--text-faint)',
            }}
          >
            <span style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
              <span
                style={{
                  width: '10px',
                  height: '10px',
                  backgroundColor: 'var(--accent-emerald)',
                  borderRadius: '2px',
                }}
              />
              Você (PJ)
            </span>
            <span style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
              <span
                style={{
                  width: '10px',
                  height: '10px',
                  backgroundColor: 'var(--accent-crimson)',
                  borderRadius: '2px',
                }}
              />
              Inimigo
            </span>
            <span style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
              <span
                style={{
                  width: '10px',
                  height: '10px',
                  backgroundColor: '#6b2130',
                  borderRadius: '2px',
                }}
              />
              Meia Cobertura (+2 CA)
            </span>
          </div>
        </div>
      )}
    </div>
  );
};
