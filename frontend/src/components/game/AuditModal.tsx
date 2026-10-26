import React, { useState, useEffect } from 'react';
import { fetchCampaignAudit } from '../../services/api';
import {
  X,
  Search,
  Users,
  Shield,
  BookOpen,
  Copy,
  Check,
  Download,
  Sparkles,
} from 'lucide-react';

interface AuditModalProps {
  campaignId: string;
  onClose: () => void;
}

export const AuditModal: React.FC<AuditModalProps> = ({ campaignId, onClose }) => {
  const [activeTab, setActiveTab] = useState<'heroes' | 'entities' | 'skeleton' | 'chronicle'>('heroes');
  const [loading, setLoading] = useState<boolean>(true);
  const [auditData, setAuditData] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);
  const [copiedJson, setCopiedJson] = useState(false);

  useEffect(() => {
    let isMounted = true;
    setLoading(true);
    fetchCampaignAudit(campaignId)
      .then((data) => {
        if (isMounted) {
          setAuditData(data);
          setLoading(false);
        }
      })
      .catch((err) => {
        if (isMounted) {
          setError(err.message || 'Erro ao carregar auditoria');
          setLoading(false);
        }
      });
    return () => {
      isMounted = false;
    };
  }, [campaignId]);

  const copyAsJson = () => {
    if (!auditData) return;
    navigator.clipboard.writeText(JSON.stringify(auditData, null, 2));
    setCopiedJson(true);
    setTimeout(() => setCopiedJson(false), 2000);
  };

  const downloadJson = () => {
    if (!auditData) return;
    const blob = new Blob([JSON.stringify(auditData, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `auditoria_campanha_${campaignId.slice(0, 8)}.json`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const manifest = auditData?.manifest;
  const skeleton = manifest?.hidden_skeleton;
  const players: any[] = auditData?.player_entities || [];
  const worldEntities: any[] = auditData?.world_entities || [];
  const chronicle: any[] = auditData?.chronicle_entries || [];

  return (
    <div
      className="modal-backdrop-fade"
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        backgroundColor: 'rgba(0, 0, 0, 0.85)',
        backdropFilter: 'blur(8px)',
        zIndex: 99999,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        padding: '24px',
      }}
      onClick={onClose}
    >
      <div
        className="modal-content-fade"
        style={{
          width: '100%',
          maxWidth: '1080px',
          maxHeight: '90vh',
          backgroundColor: '#0c0f17',
          border: '1px solid rgba(168, 85, 247, 0.35)',
          borderRadius: '16px',
          boxShadow: '0 20px 60px rgba(0,0,0,0.9), 0 0 30px rgba(168, 85, 247, 0.15)',
          display: 'flex',
          flexDirection: 'column',
          overflow: 'hidden',
          color: '#f1f5f9',
        }}
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div
          style={{
            padding: '16px 24px',
            borderBottom: '1px solid rgba(255, 255, 255, 0.1)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            background: 'linear-gradient(90deg, rgba(168, 85, 247, 0.12) 0%, transparent 100%)',
          }}
        >
          <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
            <Search size={20} color="#c084fc" />
            <div>
              <h2 style={{ margin: 0, fontSize: '17px', fontWeight: 800, color: '#fff' }}>
                Painel de Auditoria da Campanha &amp; Mestre IA
              </h2>
              <span style={{ fontSize: '11px', color: '#94a3b8' }}>
                ID: {campaignId} • Sala: #{auditData?.room_code || '---'}
              </span>
            </div>
          </div>

          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <button
              onClick={copyAsJson}
              disabled={!auditData}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '6px',
                padding: '6px 12px',
                backgroundColor: 'rgba(255, 255, 255, 0.06)',
                border: '1px solid rgba(255, 255, 255, 0.15)',
                borderRadius: '8px',
                color: '#cbd5e1',
                fontSize: '11px',
                cursor: 'pointer',
              }}
            >
              {copiedJson ? <Check size={14} color="#34d399" /> : <Copy size={14} />}
              {copiedJson ? 'Copiado!' : 'Copiar JSON'}
            </button>
            <button
              onClick={downloadJson}
              disabled={!auditData}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '6px',
                padding: '6px 12px',
                backgroundColor: 'rgba(168, 85, 247, 0.15)',
                border: '1px solid rgba(168, 85, 247, 0.4)',
                borderRadius: '8px',
                color: '#e9d5ff',
                fontSize: '11px',
                fontWeight: 700,
                cursor: 'pointer',
              }}
            >
              <Download size={14} /> Exportar JSON
            </button>
            <button
              onClick={onClose}
              data-testid="close-audit-modal"
              style={{
                background: 'transparent',
                border: 'none',
                color: '#94a3b8',
                cursor: 'pointer',
                padding: '4px',
              }}
            >
              <X size={20} />
            </button>
          </div>
        </div>

        {/* Abas */}
        <div
          style={{
            display: 'flex',
            gap: '8px',
            padding: '10px 24px',
            backgroundColor: '#07090e',
            borderBottom: '1px solid rgba(255, 255, 255, 0.08)',
          }}
        >
          {[
            { id: 'heroes', label: '1. Heróis Criados', icon: Users, count: players.length },
            { id: 'entities', label: '2. Entidades & Bosses', icon: Shield, count: worldEntities.length },
            { id: 'skeleton', label: '3. Esqueleto Oculto (IA)', icon: Sparkles, count: skeleton?.expected_acts?.length },
            { id: 'chronicle', label: '4. Crônica & História', icon: BookOpen, count: chronicle.length },
          ].map((tab) => {
            const Icon = tab.icon;
            const isActive = activeTab === tab.id;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id as any)}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '7px',
                  padding: '8px 14px',
                  backgroundColor: isActive ? 'rgba(168, 85, 247, 0.2)' : 'transparent',
                  border: isActive ? '1px solid rgba(168, 85, 247, 0.5)' : '1px solid transparent',
                  borderRadius: '8px',
                  color: isActive ? '#f3e8ff' : '#94a3b8',
                  fontSize: '12px',
                  fontWeight: 700,
                  cursor: 'pointer',
                }}
              >
                <Icon size={14} /> {tab.label}
                {tab.count !== undefined && (
                  <span
                    style={{
                      fontSize: '10px',
                      padding: '1px 6px',
                      borderRadius: '10px',
                      backgroundColor: 'rgba(255,255,255,0.1)',
                      color: '#e2e8f0',
                    }}
                  >
                    {tab.count}
                  </span>
                )}
              </button>
            );
          })}
        </div>

        {/* Conteúdo com Scroll */}
        <div style={{ flex: 1, overflowY: 'auto', padding: '24px', display: 'flex', flexDirection: 'column', gap: '16px' }}>
          {loading && (
            <div style={{ textAlign: 'center', padding: '40px', color: '#94a3b8' }}>
              Carregando dados da campanha e do servidor...
            </div>
          )}

          {error && (
            <div style={{ padding: '16px', backgroundColor: 'rgba(239, 68, 68, 0.15)', borderRadius: '8px', color: '#fca5a5' }}>
              {error}
            </div>
          )}

          {!loading && !error && (
            <>
              {/* ABA 1: HERÓIS CRIADOS */}
              {activeTab === 'heroes' && (
                <div style={{ display: 'flex', flexDirection: 'column', gap: '14px' }}>
                  {players.length === 0 ? (
                    <div style={{ color: '#94a3b8', textAlign: 'center', padding: '30px' }}>
                      Nenhum personagem registrado nesta campanha ainda.
                    </div>
                  ) : (
                    players.map((p, idx) => (
                      <div
                        key={idx}
                        style={{
                          backgroundColor: '#07090e',
                          border: '1px solid rgba(255, 255, 255, 0.08)',
                          borderRadius: '12px',
                          padding: '16px',
                          display: 'flex',
                          flexDirection: 'column',
                          gap: '10px',
                        }}
                      >
                        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                          <strong style={{ fontSize: '15px', color: '#facc15' }}>
                            ⚔️ {p.name} (ID: {p.id})
                          </strong>
                          <span style={{ fontSize: '11px', color: '#34d399', backgroundColor: 'rgba(16, 185, 129, 0.15)', padding: '2px 8px', borderRadius: '4px' }}>
                            {p.entity_type}
                          </span>
                        </div>
                        <p style={{ margin: 0, fontSize: '12.5px', color: '#cbd5e1' }}>{p.description}</p>
                        {p.dnd_stats && (
                          <div style={{ display: 'flex', gap: '16px', fontSize: '12px', color: '#94a3b8' }}>
                            <span>HP: <strong style={{ color: '#fff' }}>{p.dnd_stats.current_hp}/{p.dnd_stats.max_hp}</strong></span>
                            <span>CA: <strong style={{ color: '#fff' }}>{p.dnd_stats.armor_class}</strong></span>
                            <span>Atributos: {Object.entries(p.dnd_stats.ability_scores || {}).map(([k, v]) => `${k}:${v}`).join(' | ')}</span>
                          </div>
                        )}
                      </div>
                    ))
                  )}
                </div>
              )}

              {/* ABA 2: ENTIDADES & BOSSES */}
              {activeTab === 'entities' && (
                <div style={{ display: 'flex', flexDirection: 'column', gap: '14px' }}>
                  {worldEntities.length === 0 ? (
                    <div style={{ color: '#94a3b8', textAlign: 'center', padding: '30px' }}>
                      Nenhuma entidade registrada no mundo ainda.
                    </div>
                  ) : (
                    worldEntities.map((e, idx) => {
                      const isBoss = e.custom_flags?.is_homebrew_boss;
                      return (
                        <div
                          key={idx}
                          style={{
                            backgroundColor: '#07090e',
                            border: isBoss ? '1px solid rgba(239, 68, 68, 0.35)' : '1px solid rgba(255, 255, 255, 0.08)',
                            borderRadius: '12px',
                            padding: '16px',
                            display: 'flex',
                            flexDirection: 'column',
                            gap: '10px',
                          }}
                        >
                          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                            <strong style={{ fontSize: '15px', color: isBoss ? '#f87171' : '#e2e8f0' }}>
                              {isBoss ? '👾 ' : '🛡️ '} {e.name}
                            </strong>
                            <div style={{ display: 'flex', gap: '6px' }}>
                              {isBoss && (
                                <span style={{ fontSize: '10px', backgroundColor: 'rgba(239, 68, 68, 0.2)', color: '#fca5a5', padding: '2px 8px', borderRadius: '4px', fontWeight: 700 }}>
                                  BOSS HOMEBREW {e.custom_flags?.created_by_ai ? '(IA)' : '(JOGADOR)'}
                                </span>
                              )}
                              <span style={{ fontSize: '11px', color: '#94a3b8' }}>ID: {e.id}</span>
                            </div>
                          </div>
                          <p style={{ margin: 0, fontSize: '12.5px', color: '#cbd5e1' }}>{e.description}</p>
                          {e.dnd_stats && (
                            <div style={{ display: 'flex', gap: '16px', fontSize: '12px', color: '#94a3b8' }}>
                              <span>HP: <strong style={{ color: '#fff' }}>{e.dnd_stats.current_hp}/{e.dnd_stats.max_hp}</strong></span>
                              <span>CA: <strong style={{ color: '#fff' }}>{e.dnd_stats.armor_class}</strong></span>
                              <span>Atributos: {Object.entries(e.dnd_stats.ability_scores || {}).map(([k, v]) => `${k}:${v}`).join(' | ')}</span>
                            </div>
                          )}
                        </div>
                      );
                    })
                  )}
                </div>
              )}

              {/* ABA 3: ESQUELETO OCULTO (IA) */}
              {activeTab === 'skeleton' && (
                <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
                  <div style={{ padding: '14px', backgroundColor: '#07090e', borderRadius: '10px', border: '1px solid rgba(255,255,255,0.08)' }}>
                    <h3 style={{ margin: '0 0 8px 0', fontSize: '14px', color: '#c084fc' }}>Premissa Central</h3>
                    <p style={{ margin: 0, fontSize: '13px', color: '#cbd5e1' }}>{manifest?.premise || 'Não informada'}</p>
                  </div>

                  <h3 style={{ margin: '8px 0 0 0', fontSize: '14px', color: '#facc15' }}>Atos Esperados no Esqueleto:</h3>
                  {(skeleton?.expected_acts || []).map((act: any, idx: number) => (
                    <div
                      key={idx}
                      style={{
                        backgroundColor: '#07090e',
                        border: '1px solid rgba(234, 179, 8, 0.25)',
                        borderRadius: '10px',
                        padding: '14px',
                      }}
                    >
                      <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '6px' }}>
                        <strong style={{ color: '#facc15', fontSize: '13px' }}>
                          Ato {act.act_number}: {act.title}
                        </strong>
                        <span style={{ fontSize: '11px', color: act.completed ? '#34d399' : '#94a3b8' }}>
                          {act.completed ? '✓ Concluído' : '⏳ Pendente'}
                        </span>
                      </div>
                      <p style={{ margin: '0 0 8px 0', fontSize: '12px', color: '#94a3b8' }}>{act.synopsis}</p>
                      {act.expected_milestones?.length > 0 && (
                        <div style={{ fontSize: '11px', color: '#cbd5e1' }}>
                          <strong>Marcos:</strong> {act.expected_milestones.join(' • ')}
                        </div>
                      )}
                    </div>
                  ))}

                  {(skeleton?.autonomous_world_events || []).length > 0 && (
                    <>
                      <h3 style={{ margin: '8px 0 0 0', fontSize: '14px', color: '#a855f7' }}>Eventos Autônomos de Bastidores:</h3>
                      {skeleton.autonomous_world_events.map((ev: any, idx: number) => (
                        <div key={idx} style={{ padding: '12px', backgroundColor: '#07090e', borderRadius: '8px', border: '1px solid rgba(168, 85, 247, 0.25)' }}>
                          <span style={{ fontSize: '11px', color: '#c084fc', fontWeight: 700 }}>Disparo no Capítulo {ev.trigger_chapter}:</span>
                          <p style={{ margin: '4px 0 0 0', fontSize: '12px', color: '#cbd5e1' }}>{ev.description}</p>
                        </div>
                      ))}
                    </>
                  )}
                </div>
              )}

              {/* ABA 4: CRÔNICA & HISTÓRIA */}
              {activeTab === 'chronicle' && (
                <div style={{ display: 'flex', flexDirection: 'column', gap: '14px' }}>
                  {chronicle.length === 0 ? (
                    <div style={{ color: '#94a3b8', textAlign: 'center', padding: '30px' }}>
                      Nenhum registro ainda na crônica desta campanha.
                    </div>
                  ) : (
                    chronicle.map((entry, idx) => (
                      <div
                        key={idx}
                        style={{
                          backgroundColor: '#07090e',
                          border: '1px solid rgba(255, 255, 255, 0.08)',
                          borderRadius: '10px',
                          padding: '16px',
                        }}
                      >
                        <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '8px' }}>
                          <strong style={{ color: '#facc15', fontSize: '13px' }}>
                            Capítulo {entry.chapter}: {entry.scene_title}
                          </strong>
                          <span style={{ fontSize: '11px', color: '#64748b' }}>
                            {entry.entry_type} • {new Date(entry.timestamp).toLocaleTimeString()}
                          </span>
                        </div>
                        <p style={{ margin: 0, fontSize: '12.5px', color: '#cbd5e1', lineHeight: '1.6', whiteSpace: 'pre-wrap' }}>
                          {entry.narrative_prose}
                        </p>
                      </div>
                    ))
                  )}
                </div>
              )}
            </>
          )}
        </div>
      </div>
    </div>
  );
};
