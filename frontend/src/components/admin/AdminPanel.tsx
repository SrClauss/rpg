import React, { useState, useEffect } from 'react';
import {
  fetchAdminMetrics,
  fetchAdminUsers,
  fetchSavedPremises,
  adminSavePremise,
  adminPreviewPremise,
  adminDeletePremise,
  apiAdminLogin,
  type PrebuiltStory,
} from '../../services/api';
import {
  getAdminUser,
  setAdminSession,
  clearAdminSession,
  type AuthUser,
} from '../../services/auth';
import {
  LayoutDashboard,
  BookOpen,
  Users,
  Plus,
  Trash2,
  Sparkles,
  ArrowLeft,
  Crown,
  Shield,
  Key,
  Mail,
  LogOut,
  Lock,
  Eye,
  Check,
  X,
  Search,
  Tag,
  List,
  LayoutGrid,
  ChevronLeft,
  ChevronRight,
} from 'lucide-react';
import { normalizeTags } from '../../utils/tags';

interface AdminPanelProps {
  onClose: () => void;
}

export const AdminPanel: React.FC<AdminPanelProps> = ({ onClose }) => {
  // Sessão Administrativa Exclusiva
  const [adminUser, setAdminUser] = useState<AuthUser | null>(() => getAdminUser());
  const [loginEmail, setLoginEmail] = useState('');
  const [loginPassword, setLoginPassword] = useState('');
  const [loginLoading, setLoginLoading] = useState(false);
  const [loginError, setLoginError] = useState<string | null>(null);

  const [activeTab, setActiveTab] = useState<'metrics' | 'premises' | 'users'>('metrics');
  const [metrics, setMetrics] = useState<any>(null);
  const [users, setUsers] = useState<any[]>([]);
  const [premises, setPremises] = useState<PrebuiltStory[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  // Form de Nova Premissa com Refinamento de IA
  const [title, setTitle] = useState('');
  const [premise, setPremise] = useState('');
  const [tagsInput, setTagsInput] = useState('');
  const [style, setStyle] = useState('Fantasia Sombria, narrativa cinematográfica com foco tático');
  const [slider, setSlider] = useState<number>(10);
  const [autoRefine, setAutoRefine] = useState(true);
  const [isSaving, setIsSaving] = useState(false);

  // Estados de Busca, Filtro e UX para o Catálogo Oficial (otimizado para até 1.000+ premissas)
  const [catalogSearch, setCatalogSearch] = useState('');
  const [catalogTag, setCatalogTag] = useState<string | null>(null);
  const [catalogViewMode, setCatalogViewMode] = useState<'list' | 'cards'>('list');
  const [catalogPage, setCatalogPage] = useState(1);
  const [inspectStoryModal, setInspectStoryModal] = useState<PrebuiltStory | null>(null);
  const CATALOG_ITEMS_PER_PAGE = 8;

  const handleAdminLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoginError(null);
    setLoginLoading(true);
    try {
      const res = await apiAdminLogin({ email: loginEmail.trim(), password: loginPassword });
      setAdminSession(res.token, res.user);
      setAdminUser(res.user);
      setLoginPassword('');
    } catch (err: any) {
      setLoginError(err.message || 'Credenciais administrativas recusadas.');
    } finally {
      setLoginLoading(false);
    }
  };

  const handleAdminLogout = () => {
    clearAdminSession();
    setAdminUser(null);
  };

  const loadData = async () => {
    if (!adminUser) return;
    setLoading(true);
    setError(null);
    try {
      const [m, u, p] = await Promise.all([
        fetchAdminMetrics(),
        fetchAdminUsers(),
        fetchSavedPremises(),
      ]);
      setMetrics(m.metrics);
      setUsers(u);
      setPremises(p);
    } catch (err: any) {
      setError(err.message || 'Falha ao carregar dados administrativos');
      if (err.message && err.message.includes('Acesso negado')) {
        handleAdminLogout();
      }
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (adminUser) {
      loadData();
    }
  }, [adminUser]);

  // Modal de Pré-Visualização antes de publicar
  const [isPreviewModalOpen, setIsPreviewModalOpen] = useState(false);
  const [previewData, setPreviewData] = useState<any>(null);
  const [isPreviewLoading, setIsPreviewLoading] = useState(false);

  const handleOpenPreview = async () => {
    if (!title.trim() || !premise.trim()) {
      setError('Informe o título e a premissa para gerar a pré-visualização.');
      return;
    }
    setError(null);
    setIsPreviewLoading(true);
    try {
      const data = await adminPreviewPremise({
        title: title.trim(),
        premise: premise.trim(),
        literary_style: style.trim(),
        dramatic_focus_slider: slider,
        auto_refine_ai: autoRefine,
      });
      setPreviewData(data);
      if (data?.analysis?.tags && Array.isArray(data.analysis.tags) && data.analysis.tags.length > 0) {
        const normalizedNewTags = normalizeTags(data.analysis.tags);
        if (!tagsInput.trim()) {
          setTagsInput(normalizedNewTags.join(', '));
        } else {
          // Adiciona tags novas geradas pela IA que ainda não estejam presentes
          const currentTags = normalizeTags(tagsInput.split(','));
          const combinedTags = [...currentTags];
          for (const newTag of normalizedNewTags) {
            if (!combinedTags.includes(newTag)) {
              combinedTags.push(newTag);
            }
          }
          setTagsInput(combinedTags.join(', '));
        }
      }
      setIsPreviewModalOpen(true);
    } catch (err: any) {
      setError(err.message || 'Erro ao gerar prévia da premissa');
    } finally {
      setIsPreviewLoading(false);
    }
  };

  const confirmPublishFromPreview = async (useOriginalText: boolean = false) => {
    setIsSaving(true);
    setError(null);
    try {
      const selectedPremise = useOriginalText
        ? (previewData?.raw_premise || premise.trim())
        : (previewData?.refined_premise || premise.trim());

      const parsedTags = normalizeTags(
        tagsInput.trim()
          ? tagsInput.split(',')
          : (previewData?.analysis?.tags || [])
      );

      const effectiveStyle = (previewData?.literary_style || style).trim();

      const res = await adminSavePremise({
        title: previewData?.title || title.trim(),
        premise: selectedPremise,
        literary_style: effectiveStyle,
        dramatic_focus_slider: slider,
        tags: parsedTags,
        auto_refine_ai: false, // já foi revisado na pré-visualização!
        custom_monsters: previewData?.analysis?.custom_monsters || [],
        subclass_overrides: previewData?.analysis?.subclass_overrides || [],
        custom_spells: previewData?.analysis?.custom_spells || [],
      } as any);
      setSuccess(
        `Premissa "${res.title}" publicada com sucesso no catálogo oficial (${useOriginalText ? 'Texto Original Preservado' : 'Versão Refinada por IA'})!`
      );
      setTitle('');
      setPremise('');
      setTagsInput('');
      setIsPreviewModalOpen(false);
      setPreviewData(null);
      setTimeout(() => setSuccess(null), 4000);
      const updated = await fetchSavedPremises();
      setPremises(updated);
    } catch (err: any) {
      setError(err.message || 'Erro ao publicar premissa');
    } finally {
      setIsSaving(false);
    }
  };

  const handleSavePremise = async (e: React.FormEvent) => {
    e.preventDefault();
    await handleOpenPreview();
  };

  const handleDeletePremise = async (id: string) => {
    if (!window.confirm('Deseja realmente remover esta história do catálogo oficial?')) return;
    try {
      await adminDeletePremise(id);
      setSuccess('História removida do catálogo oficial.');
      setTimeout(() => setSuccess(null), 3000);
      const updated = await fetchSavedPremises();
      setPremises(updated);
    } catch (err: any) {
      setError(err.message || 'Falha ao excluir premissa');
    }
  };

  // Se não estiver autenticado como administrador, exibe a tela exclusiva de Login Administrativo (Backoffice)
  if (!adminUser) {
    return (
      <div
        style={{
          minHeight: '100vh',
          backgroundColor: '#05070b',
          color: '#f8fafc',
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          justifyContent: 'center',
          padding: '20px',
        }}
      >
        <div style={{ position: 'absolute', top: '20px', left: '20px' }}>
          <button
            type="button"
            onClick={onClose}
            data-testid="admin-exit-btn"
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              padding: '8px 14px',
              borderRadius: '8px',
              backgroundColor: 'rgba(255, 255, 255, 0.05)',
              border: '1px solid rgba(255, 255, 255, 0.15)',
              color: '#cbd5e1',
              fontSize: '12px',
              fontWeight: 700,
              cursor: 'pointer',
            }}
          >
            <ArrowLeft size={14} /> Voltar ao Portal do Jogo
          </button>
        </div>

        <div
          data-testid="admin-login-card"
          style={{
            width: '100%',
            maxWidth: '420px',
            backgroundColor: '#0a0d14',
            border: '1px solid rgba(168, 85, 247, 0.4)',
            borderRadius: '16px',
            padding: '32px 28px',
            boxShadow: '0 25px 60px -12px rgba(0, 0, 0, 0.9)',
            display: 'flex',
            flexDirection: 'column',
            gap: '20px',
          }}
        >
          <div style={{ textAlign: 'center' }}>
            <div
              style={{
                display: 'inline-flex',
                padding: '12px',
                borderRadius: '50%',
                backgroundColor: 'rgba(168, 85, 247, 0.15)',
                border: '1px solid rgba(168, 85, 247, 0.4)',
                marginBottom: '12px',
              }}
            >
              <Shield size={28} color="#c084fc" />
            </div>
            <h1 style={{ fontSize: '18px', fontWeight: 900, color: '#ffffff', margin: '0 0 6px 0', letterSpacing: '0.06em' }}>
              PORTAL ADMINISTRATIVO SAAS
            </h1>
            <p style={{ fontSize: '12px', color: '#94a3b8', margin: 0, lineHeight: 1.4 }}>
              Acesso exclusivo para governança, métricas do ecossistema e curadoria editorial oficial.
            </p>
          </div>

          {loginError && (
            <div style={{ padding: '10px 14px', backgroundColor: 'rgba(239, 68, 68, 0.15)', border: '1px solid rgba(239, 68, 68, 0.4)', borderRadius: '8px', color: '#fca5a5', fontSize: '12.5px' }}>
              ⚠️ {loginError}
            </div>
          )}

          <form onSubmit={handleAdminLogin} style={{ display: 'flex', flexDirection: 'column', gap: '14px' }}>
            <div>
              <label style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '11px', color: '#cbd5e1', fontWeight: 800, textTransform: 'uppercase', marginBottom: '6px' }}>
                <Mail size={12} color="#c084fc" /> Email do Administrador
              </label>
              <input
                type="email"
                required
                placeholder="admin@dominio.com"
                value={loginEmail}
                onChange={(e) => setLoginEmail(e.target.value)}
                data-testid="admin-login-email"
                style={{
                  width: '100%',
                  padding: '11px 14px',
                  backgroundColor: '#05070b',
                  border: '1px solid rgba(255, 255, 255, 0.15)',
                  borderRadius: '8px',
                  color: '#ffffff',
                  fontSize: '13px',
                  boxSizing: 'border-box',
                }}
              />
            </div>

            <div>
              <label style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '11px', color: '#cbd5e1', fontWeight: 800, textTransform: 'uppercase', marginBottom: '6px' }}>
                <Key size={12} color="#c084fc" /> Senha Administrativa (Argon2id)
              </label>
              <input
                type="password"
                required
                placeholder="••••••••"
                value={loginPassword}
                onChange={(e) => setLoginPassword(e.target.value)}
                data-testid="admin-login-password"
                style={{
                  width: '100%',
                  padding: '11px 14px',
                  backgroundColor: '#05070b',
                  border: '1px solid rgba(255, 255, 255, 0.15)',
                  borderRadius: '8px',
                  color: '#ffffff',
                  fontSize: '13px',
                  boxSizing: 'border-box',
                }}
              />
            </div>

            <button
              type="submit"
              disabled={loginLoading}
              data-testid="admin-login-submit-btn"
              style={{
                marginTop: '8px',
                padding: '13px',
                background: 'linear-gradient(135deg, #a855f7 0%, #7e22ce 100%)',
                border: 'none',
                borderRadius: '8px',
                color: '#ffffff',
                fontWeight: 900,
                fontSize: '13px',
                textTransform: 'uppercase',
                letterSpacing: '0.06em',
                cursor: loginLoading ? 'not-allowed' : 'pointer',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                gap: '8px',
                boxShadow: '0 8px 20px rgba(168, 85, 247, 0.3)',
              }}
            >
              <Lock size={15} /> {loginLoading ? 'Autenticando credencial...' : 'Acessar Backoffice'}
            </button>
          </form>

          <div style={{ borderTop: '1px solid rgba(255, 255, 255, 0.08)', paddingTop: '12px', textAlign: 'center' }}>
            <span style={{ fontSize: '11px', color: '#64748b' }}>
              Autenticação isolada via porta de segurança <code>/api/admin/auth/login</code>
            </span>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div
      style={{
        minHeight: '100vh',
        backgroundColor: '#07090e',
        color: '#f8fafc',
        padding: '24px',
        display: 'flex',
        flexDirection: 'column',
        gap: '20px',
      }}
    >
      {/* Topo do Painel Administrativo */}
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          flexWrap: 'wrap',
          gap: '16px',
          paddingBottom: '16px',
          borderBottom: '1px solid rgba(255, 255, 255, 0.1)',
        }}
      >
        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <button
            type="button"
            onClick={onClose}
            data-testid="admin-back-btn"
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              padding: '8px 14px',
              borderRadius: '8px',
              backgroundColor: 'rgba(255, 255, 255, 0.05)',
              border: '1px solid rgba(255, 255, 255, 0.15)',
              color: '#cbd5e1',
              fontSize: '12px',
              fontWeight: 700,
              cursor: 'pointer',
            }}
          >
            <ArrowLeft size={14} /> Voltar para o Jogo
          </button>
          <div>
            <div style={{ display: 'inline-flex', alignItems: 'center', gap: '6px', color: '#facc15', fontSize: '11px', fontWeight: 800, textTransform: 'uppercase', letterSpacing: '0.08em' }}>
              <Crown size={13} /> Gestão SaaS Multi-Tenant • Módulo Operacional
            </div>
            <h1 style={{ fontSize: '22px', fontWeight: 900, color: '#ffffff', margin: '2px 0 0 0' }}>
              PAINEL ADMINISTRATIVO DO SISTEMA
            </h1>
          </div>
        </div>

        {/* Abas */}
        <div style={{ display: 'flex', gap: '8px', backgroundColor: '#0f141f', padding: '4px', borderRadius: '10px' }}>
          <button
            type="button"
            onClick={() => setActiveTab('metrics')}
            data-testid="tab-admin-metrics"
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              padding: '8px 14px',
              borderRadius: '7px',
              border: activeTab === 'metrics' ? '1px solid rgba(234, 179, 8, 0.4)' : '1px solid transparent',
              backgroundColor: activeTab === 'metrics' ? 'rgba(234, 179, 8, 0.18)' : 'transparent',
              color: activeTab === 'metrics' ? '#facc15' : '#94a3b8',
              fontSize: '12px',
              fontWeight: 700,
              cursor: 'pointer',
            }}
          >
            <LayoutDashboard size={14} /> Métricas SaaS
          </button>

          <button
            type="button"
            onClick={() => setActiveTab('premises')}
            data-testid="tab-admin-premises"
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              padding: '8px 14px',
              borderRadius: '7px',
              border: activeTab === 'premises' ? '1px solid rgba(56, 189, 248, 0.4)' : '1px solid transparent',
              backgroundColor: activeTab === 'premises' ? 'rgba(56, 189, 248, 0.18)' : 'transparent',
              color: activeTab === 'premises' ? '#38bdf8' : '#94a3b8',
              fontSize: '12px',
              fontWeight: 700,
              cursor: 'pointer',
            }}
          >
            <BookOpen size={14} /> Catálogo Oficial ({premises.length})
          </button>

          <button
            type="button"
            onClick={() => setActiveTab('users')}
            data-testid="tab-admin-users"
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              padding: '8px 14px',
              borderRadius: '7px',
              border: activeTab === 'users' ? '1px solid rgba(168, 85, 247, 0.4)' : '1px solid transparent',
              backgroundColor: activeTab === 'users' ? 'rgba(168, 85, 247, 0.18)' : 'transparent',
              color: activeTab === 'users' ? '#c084fc' : '#94a3b8',
              fontSize: '12px',
              fontWeight: 700,
              cursor: 'pointer',
            }}
          >
            <Users size={14} /> Usuários &amp; Hosts ({users.length})
          </button>

          <button
            type="button"
            onClick={handleAdminLogout}
            data-testid="admin-logout-btn"
            title="Encerrar sessão de Administrador"
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              padding: '8px 12px',
              borderRadius: '7px',
              border: '1px solid rgba(239, 68, 68, 0.4)',
              backgroundColor: 'rgba(239, 68, 68, 0.12)',
              color: '#f87171',
              fontSize: '12px',
              fontWeight: 700,
              cursor: 'pointer',
              marginLeft: '8px',
            }}
          >
            <LogOut size={13} /> Sair Admin
          </button>
        </div>
      </div>

      {/* Alertas */}
      {loading && (
        <div style={{ padding: '10px 16px', backgroundColor: 'rgba(56, 189, 248, 0.1)', border: '1px solid rgba(56, 189, 248, 0.3)', borderRadius: '10px', color: '#7dd3fc', fontSize: '13px' }}>
          ⏳ Carregando dados administrativos...
        </div>
      )}
      {error && (
        <div style={{ padding: '12px 16px', backgroundColor: 'rgba(239, 68, 68, 0.12)', border: '1px solid rgba(239, 68, 68, 0.4)', borderRadius: '10px', color: '#fca5a5', fontSize: '13px' }}>
          ⚠️ {error}
        </div>
      )}
      {success && (
        <div style={{ padding: '12px 16px', backgroundColor: 'rgba(16, 185, 129, 0.15)', border: '1px solid rgba(16, 185, 129, 0.4)', borderRadius: '10px', color: '#6ee7b7', fontSize: '13px' }}>
          ✓ {success}
        </div>
      )}

      {/* CONTEÚDO DAS ABAS */}
      {activeTab === 'metrics' && metrics && (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(220px, 1fr))', gap: '16px' }}>
            <div style={{ backgroundColor: '#0c1017', border: '1px solid rgba(255, 255, 255, 0.08)', borderRadius: '12px', padding: '20px' }}>
              <span style={{ fontSize: '11px', color: '#94a3b8', textTransform: 'uppercase', fontWeight: 800 }}>Total de Usuários</span>
              <div style={{ fontSize: '28px', fontWeight: 900, color: '#facc15', marginTop: '6px' }}>{metrics.total_users}</div>
              <span style={{ fontSize: '11.5px', color: '#64748b' }}>Cadastrados no sistema</span>
            </div>

            <div style={{ backgroundColor: '#0c1017', border: '1px solid rgba(255, 255, 255, 0.08)', borderRadius: '12px', padding: '20px' }}>
              <span style={{ fontSize: '11px', color: '#94a3b8', textTransform: 'uppercase', fontWeight: 800 }}>Contas de Host</span>
              <div style={{ fontSize: '28px', fontWeight: 900, color: '#38bdf8', marginTop: '6px' }}>{metrics.total_hosts}</div>
              <span style={{ fontSize: '11.5px', color: '#64748b' }}>Mestres com permissão de criação</span>
            </div>

            <div style={{ backgroundColor: '#0c1017', border: '1px solid rgba(255, 255, 255, 0.08)', borderRadius: '12px', padding: '20px' }}>
              <span style={{ fontSize: '11px', color: '#94a3b8', textTransform: 'uppercase', fontWeight: 800 }}>Histórias Oficiais</span>
              <div style={{ fontSize: '28px', fontWeight: 900, color: '#34d399', marginTop: '6px' }}>{metrics.saved_premises}</div>
              <span style={{ fontSize: '11.5px', color: '#64748b' }}>No catálogo global do servidor</span>
            </div>

            <div style={{ backgroundColor: '#0c1017', border: '1px solid rgba(255, 255, 255, 0.08)', borderRadius: '12px', padding: '20px' }}>
              <span style={{ fontSize: '11px', color: '#94a3b8', textTransform: 'uppercase', fontWeight: 800 }}>Salas Ativas</span>
              <div style={{ fontSize: '28px', fontWeight: 900, color: '#c084fc', marginTop: '6px' }}>{metrics.active_rooms}</div>
              <span style={{ fontSize: '11.5px', color: '#64748b' }}>Sessões em memória no Axum</span>
            </div>
          </div>

          <div style={{ backgroundColor: '#0c1017', border: '1px solid rgba(255, 255, 255, 0.08)', borderRadius: '12px', padding: '20px' }}>
            <h3 style={{ fontSize: '15px', color: '#f8fafc', margin: '0 0 8px 0' }}>Status do Modelo de Monetização SaaS</h3>
            <p style={{ fontSize: '13px', color: '#94a3b8', margin: 0, lineHeight: 1.5 }}>
              O sistema opera com separação estrita de papéis: Jogadores podem entrar e interagir gratuitamente com código de 4 letras.
              A criação de campanhas está travada para contas com papel de <strong>Host</strong>, permitindo ativar a cobrança por campanhas ou assinatura mensal sem necessidade de retrabalho na arquitetura.
            </p>
          </div>
        </div>
      )}

      {/* ABA: GESTÃO DE PREMISSAS OFICIAIS & REVISOR DE IA */}
      {activeTab === 'premises' && (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
          {/* Formulário com Revisor de IA */}
          <form
            onSubmit={handleSavePremise}
            style={{
              backgroundColor: '#0c1017',
              border: '1px solid rgba(234, 179, 8, 0.35)',
              borderRadius: '12px',
              padding: '20px',
              display: 'flex',
              flexDirection: 'column',
              gap: '14px',
            }}
          >
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexWrap: 'wrap', gap: '10px' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                <Plus size={16} color="#facc15" />
                <strong style={{ fontSize: '14px', color: '#facc15', textTransform: 'uppercase', letterSpacing: '0.04em' }}>
                  Forjar Nova Premissa de Campanha
                </strong>
              </div>
              <div style={{ display: 'flex', alignItems: 'center', gap: '14px', flexWrap: 'wrap' }}>
                <label style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '12px', color: !autoRefine ? '#4ade80' : '#94a3b8', cursor: 'pointer', fontWeight: !autoRefine ? 700 : 500 }}>
                  <input
                    type="radio"
                    name="ai_freedom_mode"
                    checked={!autoRefine}
                    onChange={() => setAutoRefine(false)}
                    data-testid="admin-premise-ai-exact"
                  />
                  <span>🔒 Preservar Texto Integral (Sem Modificação da IA)</span>
                </label>
                <label style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '12px', color: autoRefine ? '#38bdf8' : '#94a3b8', cursor: 'pointer', fontWeight: autoRefine ? 700 : 500 }}>
                  <input
                    type="radio"
                    name="ai_freedom_mode"
                    checked={autoRefine}
                    onChange={() => setAutoRefine(true)}
                    data-testid="admin-premise-ai-refine"
                  />
                  <span>✨ Enriquecer Estilo com IA (Preservando Seções & Arquétipos)</span>
                </label>
              </div>
            </div>

            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))', gap: '12px' }}>
              <div>
                <label style={{ fontSize: '11px', color: '#94a3b8', fontWeight: 700, textTransform: 'uppercase', marginBottom: '4px', display: 'block' }}>Título da História *</label>
                <input
                  type="text"
                  required
                  placeholder="Ex: A Vingança de Kar-Drak"
                  value={title}
                  onChange={(e) => setTitle(e.target.value)}
                  data-testid="admin-story-title"
                  style={{
                    width: '100%',
                    padding: '10px 12px',
                    backgroundColor: '#070a0f',
                    border: '1px solid rgba(255, 255, 255, 0.12)',
                    borderRadius: '8px',
                    color: '#ffffff',
                    fontSize: '13px',
                    boxSizing: 'border-box',
                  }}
                />
              </div>

              <div>
                <label style={{ fontSize: '11px', color: '#94a3b8', fontWeight: 700, textTransform: 'uppercase', marginBottom: '4px', display: 'block' }}>Estilo Literário</label>
                <input
                  type="text"
                  placeholder="Ex: Terror Cósmico, Fantasia Sombria"
                  value={style}
                  onChange={(e) => setStyle(e.target.value)}
                  data-testid="admin-story-style"
                  style={{
                    width: '100%',
                    padding: '10px 12px',
                    backgroundColor: '#070a0f',
                    border: '1px solid rgba(255, 255, 255, 0.12)',
                    borderRadius: '8px',
                    color: '#ffffff',
                    fontSize: '13px',
                    boxSizing: 'border-box',
                  }}
                />
              </div>
            </div>

            <div>
              <label style={{ fontSize: '11px', color: '#38bdf8', fontWeight: 700, textTransform: 'uppercase', marginBottom: '4px', display: 'flex', justifyContent: 'space-between' }}>
                <span>🏷️ Tags / Categorias Temáticas (separadas por vírgula)</span>
                <span style={{ fontSize: '10px', color: '#64748b', textTransform: 'none' }}>Ex: Anos 80/90, Sci-Fi, He-Man & She-Ra, Robôs</span>
              </label>
              <input
                type="text"
                placeholder="Ex: Anos 80/90, Ficção Científica, He-Man & She-Ra, Alta Tecnologia, Campanha Épica"
                value={tagsInput}
                onChange={(e) => setTagsInput(e.target.value)}
                data-testid="admin-story-tags"
                style={{
                  width: '100%',
                  padding: '10px 12px',
                  backgroundColor: '#070a0f',
                  border: '1px solid rgba(56, 189, 248, 0.25)',
                  borderRadius: '8px',
                  color: '#ffffff',
                  fontSize: '13px',
                  boxSizing: 'border-box',
                }}
              />
            </div>

            <div>
              <label style={{ fontSize: '11px', color: '#94a3b8', fontWeight: 700, textTransform: 'uppercase', marginBottom: '4px', display: 'block' }}>
                Premissa Completa &amp; Entidades Homebrew (Entrada Unificada) *
              </label>
              <textarea
                required
                rows={4}
                placeholder="Insira a narrativa central, requisitos de grupo e entidades homebrew (ex: lore do reino, monstros, stats de chefes, alterações de mundo). O Revisor de IA assimilará tudo em um único processo!"
                value={premise}
                onChange={(e) => setPremise(e.target.value)}
                data-testid="admin-story-premise"
                style={{
                  width: '100%',
                  padding: '10px 12px',
                  backgroundColor: '#070a0f',
                  border: '1px solid rgba(255, 255, 255, 0.12)',
                  borderRadius: '8px',
                  color: '#ffffff',
                  fontSize: '13px',
                  boxSizing: 'border-box',
                }}
              />
            </div>

            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexWrap: 'wrap', gap: '12px' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
                <span style={{ fontSize: '12px', color: '#94a3b8' }}>Foco D20 (1 Roleplay - 20 Tático):</span>
                <input
                  type="number"
                  min={1}
                  max={20}
                  value={slider}
                  onChange={(e) => setSlider(Math.min(20, Math.max(1, parseInt(e.target.value) || 10)))}
                  style={{
                    width: '60px',
                    padding: '6px 8px',
                    backgroundColor: '#000',
                    border: '1px solid rgba(255, 255, 255, 0.2)',
                    borderRadius: '6px',
                    color: '#facc15',
                    fontWeight: 800,
                    textAlign: 'center',
                  }}
                />
              </div>

              <div style={{ display: 'flex', gap: '10px' }}>
                <button
                  type="submit"
                  disabled={isPreviewLoading || isSaving}
                  data-testid="admin-preview-premise-btn"
                  style={{
                    padding: '10px 22px',
                    background: 'linear-gradient(135deg, #a855f7 0%, #7e22ce 100%)',
                    border: '1px solid #c084fc',
                    borderRadius: '8px',
                    color: '#ffffff',
                    fontWeight: 900,
                    fontSize: '12.5px',
                    cursor: isPreviewLoading || isSaving ? 'not-allowed' : 'pointer',
                    display: 'flex',
                    alignItems: 'center',
                    gap: '6px',
                    boxShadow: '0 4px 15px rgba(168, 85, 247, 0.35)',
                  }}
                >
                  {isPreviewLoading ? (
                    <Sparkles size={14} className="animate-spin" />
                  ) : (
                    <Eye size={14} />
                  )}
                  {isPreviewLoading
                    ? 'Consultando IA (Duas Vias)...'
                    : 'Pré-Visualizar com IA'}
                </button>

                <button
                  type="button"
                  onClick={handleSavePremise}
                  disabled={isPreviewLoading || isSaving}
                  data-testid="admin-save-premise-btn"
                  style={{
                    padding: '10px 24px',
                    background: 'linear-gradient(135deg, #facc15 0%, #ca8a04 100%)',
                    border: 'none',
                    borderRadius: '8px',
                    color: '#000',
                    fontWeight: 900,
                    fontSize: '12.5px',
                    cursor: isPreviewLoading || isSaving ? 'not-allowed' : 'pointer',
                    display: 'flex',
                    alignItems: 'center',
                    gap: '6px',
                  }}
                >
                  {isSaving ? <Sparkles size={14} className="animate-spin" /> : <Plus size={14} />}
                  {isSaving ? 'Gravando...' : 'Publicar no Catálogo Oficial'}
                </button>
              </div>
            </div>
          </form>

          {/* Catálogo Oficial de Premissas com Busca, Tags e Paginação para Escalabilidade (1000+ Histórias) */}
          {(() => {
            const allAvailableTags = Array.from(
              new Set(
                premises
                  .flatMap((p) => normalizeTags(p.tags))
                  .map((t) => t.toLowerCase().trim())
                  .filter(Boolean)
              )
            ).sort();

            const filteredPremises = premises.filter((p) => {
              const q = catalogSearch.toLowerCase().trim();
              const pTags = normalizeTags(p.tags);
              const matchesSearch =
                !q ||
                p.title.toLowerCase().includes(q) ||
                p.premise.toLowerCase().includes(q) ||
                (p.literary_style || '').toLowerCase().includes(q) ||
                pTags.some((t) => t.toLowerCase().includes(q));

              const matchesTag =
                !catalogTag ||
                pTags.some((t) => t.toLowerCase() === catalogTag.toLowerCase().trim());

              return matchesSearch && matchesTag;
            });

            const totalCatalogPages = Math.max(1, Math.ceil(filteredPremises.length / CATALOG_ITEMS_PER_PAGE));
            const currentCatalogPageSafe = Math.min(catalogPage, totalCatalogPages);
            const paginatedPremises = filteredPremises.slice(
              (currentCatalogPageSafe - 1) * CATALOG_ITEMS_PER_PAGE,
              currentCatalogPageSafe * CATALOG_ITEMS_PER_PAGE
            );

            return (
              <div style={{ display: 'flex', flexDirection: 'column', gap: '14px', marginTop: '8px' }}>
                {/* Header do Catálogo com Contadores */}
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexWrap: 'wrap', gap: '10px' }}>
                  <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <BookOpen size={16} color="#38bdf8" />
                    <span style={{ fontSize: '13px', fontWeight: 900, color: '#38bdf8', textTransform: 'uppercase', letterSpacing: '0.06em' }}>
                      Catálogo Oficial de Histórias Pré-Construídas ({premises.length})
                    </span>
                  </div>

                  {/* Alternador de Visualização: Lista vs Cards */}
                  <div style={{ display: 'flex', alignItems: 'center', gap: '6px', backgroundColor: '#070a0f', padding: '4px', borderRadius: '8px', border: '1px solid rgba(255,255,255,0.08)' }}>
                    <button
                      type="button"
                      onClick={() => setCatalogViewMode('list')}
                      data-testid="admin-catalog-view-list"
                      style={{
                        padding: '4px 10px',
                        borderRadius: '6px',
                        border: 'none',
                        backgroundColor: catalogViewMode === 'list' ? 'rgba(56, 189, 248, 0.2)' : 'transparent',
                        color: catalogViewMode === 'list' ? '#38bdf8' : '#94a3b8',
                        fontSize: '11.5px',
                        fontWeight: 700,
                        cursor: 'pointer',
                        display: 'flex',
                        alignItems: 'center',
                        gap: '5px',
                      }}
                    >
                      <List size={13} /> Lista Tabela
                    </button>
                    <button
                      type="button"
                      onClick={() => setCatalogViewMode('cards')}
                      data-testid="admin-catalog-view-cards"
                      style={{
                        padding: '4px 10px',
                        borderRadius: '6px',
                        border: 'none',
                        backgroundColor: catalogViewMode === 'cards' ? 'rgba(56, 189, 248, 0.2)' : 'transparent',
                        color: catalogViewMode === 'cards' ? '#38bdf8' : '#94a3b8',
                        fontSize: '11.5px',
                        fontWeight: 700,
                        cursor: 'pointer',
                        display: 'flex',
                        alignItems: 'center',
                        gap: '5px',
                      }}
                    >
                      <LayoutGrid size={13} /> Cards
                    </button>
                  </div>
                </div>

                {/* Barra de Pesquisa e Filtro de Tags */}
                <div style={{ display: 'flex', flexDirection: 'column', gap: '10px', backgroundColor: '#070a0f', padding: '12px 14px', borderRadius: '10px', border: '1px solid rgba(255, 255, 255, 0.08)' }}>
                  <div style={{ display: 'flex', gap: '10px', alignItems: 'center' }}>
                    <div style={{ position: 'relative', flex: 1 }}>
                      <Search size={15} color="#64748b" style={{ position: 'absolute', left: '10px', top: '50%', transform: 'translateY(-50%)' }} />
                      <input
                        type="text"
                        placeholder="Pesquisar por título, tags (#Anos 80, #Sci-Fi, #Sombria), estilo ou termos do enredo..."
                        value={catalogSearch}
                        onChange={(e) => {
                          setCatalogSearch(e.target.value);
                          setCatalogPage(1);
                        }}
                        data-testid="admin-catalog-search-input"
                        style={{
                          width: '100%',
                          padding: '9px 12px 9px 34px',
                          backgroundColor: '#0c1017',
                          border: '1px solid rgba(255, 255, 255, 0.12)',
                          borderRadius: '8px',
                          color: '#ffffff',
                          fontSize: '12.5px',
                          boxSizing: 'border-box',
                        }}
                      />
                      {catalogSearch && (
                        <button
                          type="button"
                          onClick={() => {
                            setCatalogSearch('');
                            setCatalogPage(1);
                          }}
                          style={{
                            position: 'absolute',
                            right: '10px',
                            top: '50%',
                            transform: 'translateY(-50%)',
                            background: 'none',
                            border: 'none',
                            color: '#94a3b8',
                            cursor: 'pointer',
                            fontSize: '12px',
                          }}
                        >
                          ✕ Limpar
                        </button>
                      )}
                    </div>
                  </div>

                  {/* Nuvem de Tag Chips com Contador */}
                  {allAvailableTags.length > 0 && (
                    <div style={{ display: 'flex', alignItems: 'center', gap: '6px', flexWrap: 'wrap', paddingTop: '4px' }}>
                      <span style={{ fontSize: '11px', color: '#64748b', display: 'flex', alignItems: 'center', gap: '4px', marginRight: '4px' }}>
                        <Tag size={12} /> Tags:
                      </span>
                      <button
                        type="button"
                        onClick={() => {
                          setCatalogTag(null);
                          setCatalogPage(1);
                        }}
                        data-testid="admin-catalog-tag-all"
                        style={{
                          padding: '3px 9px',
                          borderRadius: '6px',
                          fontSize: '11px',
                          fontWeight: 700,
                          cursor: 'pointer',
                          border: catalogTag === null ? '1px solid #38bdf8' : '1px solid rgba(255, 255, 255, 0.08)',
                          backgroundColor: catalogTag === null ? 'rgba(56, 189, 248, 0.2)' : 'rgba(255, 255, 255, 0.03)',
                          color: catalogTag === null ? '#38bdf8' : '#94a3b8',
                        }}
                      >
                        Todas ({premises.length})
                      </button>
                      {allAvailableTags.map((tag) => {
                        const count = premises.filter((p) =>
                          normalizeTags(p.tags).some((t) => t.toLowerCase() === tag.toLowerCase())
                        ).length;
                        const isSelected = catalogTag?.toLowerCase() === tag.toLowerCase();
                        return (
                          <button
                            key={tag}
                            type="button"
                            onClick={() => {
                              setCatalogTag(isSelected ? null : tag.toLowerCase());
                              setCatalogPage(1);
                            }}
                            data-testid={`admin-catalog-tag-${tag.toLowerCase()}`}
                            style={{
                              padding: '3px 9px',
                              borderRadius: '6px',
                              fontSize: '11px',
                              fontWeight: 700,
                              cursor: 'pointer',
                              border: isSelected ? '1px solid #facc15' : '1px solid rgba(255, 255, 255, 0.08)',
                              backgroundColor: isSelected ? 'rgba(234, 179, 8, 0.2)' : 'rgba(255, 255, 255, 0.03)',
                              color: isSelected ? '#facc15' : '#cbd5e1',
                              display: 'flex',
                              alignItems: 'center',
                              gap: '4px',
                            }}
                          >
                            <span>#{tag.toLowerCase()}</span>
                            <span style={{ fontSize: '10px', color: isSelected ? '#fef08a' : '#64748b' }}>({count})</span>
                          </button>
                        );
                      })}
                    </div>
                  )}
                </div>

                {/* Exibição da Lista Filtrada */}
                {filteredPremises.length === 0 ? (
                  <div style={{ padding: '30px', textAlign: 'center', backgroundColor: '#0c1017', border: '1px dashed rgba(255,255,255,0.1)', borderRadius: '12px', color: '#94a3b8', fontSize: '13px' }}>
                    Nenhuma história encontrada para os filtros aplicados. Tente ajustar o termo de busca ou limpar a tag selecionada.
                  </div>
                ) : catalogViewMode === 'list' ? (
                  /* MODO TABELA / LISTA (Ideal para centenas ou 1.000 premissas) */
                  <div style={{ backgroundColor: '#0c1017', border: '1px solid rgba(255, 255, 255, 0.08)', borderRadius: '12px', overflow: 'hidden' }}>
                    <table style={{ width: '100%', borderCollapse: 'collapse', textAlign: 'left', fontSize: '12.5px' }}>
                      <thead>
                        <tr style={{ backgroundColor: '#070a0f', borderBottom: '1px solid rgba(255, 255, 255, 0.1)', color: '#94a3b8', fontSize: '11px', textTransform: 'uppercase' }}>
                          <th style={{ padding: '12px 14px', width: '42%' }}>Título &amp; Resumo Narrativo</th>
                          <th style={{ padding: '12px 14px', width: '28%' }}>Tags &amp; Categorias</th>
                          <th style={{ padding: '12px 14px', width: '16%' }}>Estilo &amp; Foco</th>
                          <th style={{ padding: '12px 14px', width: '14%', textAlign: 'right' }}>Ações</th>
                        </tr>
                      </thead>
                      <tbody>
                        {paginatedPremises.map((p) => (
                          <tr
                            key={p.id}
                            style={{
                              borderBottom: '1px solid rgba(255, 255, 255, 0.04)',
                              transition: 'background-color 0.15s ease',
                            }}
                            onMouseEnter={(e) => (e.currentTarget.style.backgroundColor = 'rgba(255, 255, 255, 0.02)')}
                            onMouseLeave={(e) => (e.currentTarget.style.backgroundColor = 'transparent')}
                          >
                            <td style={{ padding: '12px 14px' }}>
                              <div style={{ fontWeight: 800, color: '#f8fafc', fontSize: '13.5px', marginBottom: '4px' }}>
                                {p.title}
                              </div>
                              <p style={{ color: '#94a3b8', fontSize: '11.5px', margin: 0, overflow: 'hidden', textOverflow: 'ellipsis', display: '-webkit-box', WebkitLineClamp: 2, WebkitBoxOrient: 'vertical', lineHeight: 1.4 }}>
                                {p.premise}
                              </p>
                            </td>

                            <td style={{ padding: '12px 14px' }}>
                              <div style={{ display: 'flex', flexWrap: 'wrap', gap: '5px' }}>
                                {normalizeTags(p.tags).length > 0 ? (
                                  normalizeTags(p.tags).map((t) => (
                                    <span
                                      key={t}
                                      style={{
                                        fontSize: '10.5px',
                                        padding: '2px 7px',
                                        borderRadius: '4px',
                                        backgroundColor: 'rgba(56, 189, 248, 0.12)',
                                        border: '1px solid rgba(56, 189, 248, 0.28)',
                                        color: '#38bdf8',
                                        fontWeight: 600,
                                      }}
                                    >
                                      #{t.toLowerCase()}
                                    </span>
                                  ))
                                ) : (
                                  <span style={{ fontSize: '11px', color: '#64748b' }}>Sem tags</span>
                                )}
                              </div>
                            </td>

                            <td style={{ padding: '12px 14px' }}>
                              <div style={{ fontSize: '11px', color: '#cbd5e1', marginBottom: '4px' }}>
                                {p.literary_style || 'Padrão'}
                              </div>
                              <span style={{ fontSize: '10px', color: '#facc15', backgroundColor: 'rgba(234, 179, 8, 0.15)', border: '1px solid rgba(234, 179, 8, 0.3)', padding: '2px 6px', borderRadius: '4px', fontWeight: 800 }}>
                                D20: {p.dramatic_focus_slider || 10}
                              </span>
                            </td>

                            <td style={{ padding: '12px 14px', textAlign: 'right' }}>
                              <div style={{ display: 'flex', justifyContent: 'flex-end', gap: '6px' }}>
                                <button
                                  type="button"
                                  onClick={() => setInspectStoryModal(p)}
                                  data-testid={`admin-inspect-premise-${p.id}`}
                                  title="Inspecionar História Completa"
                                  style={{
                                    padding: '6px 8px',
                                    borderRadius: '6px',
                                    backgroundColor: 'rgba(56, 189, 248, 0.1)',
                                    border: '1px solid rgba(56, 189, 248, 0.3)',
                                    color: '#38bdf8',
                                    fontSize: '11.5px',
                                    cursor: 'pointer',
                                    display: 'flex',
                                    alignItems: 'center',
                                    gap: '4px',
                                  }}
                                >
                                  <Eye size={12} /> Ver
                                </button>
                                <button
                                  type="button"
                                  onClick={() => handleDeletePremise(p.id)}
                                  data-testid={`admin-delete-premise-${p.id}`}
                                  title="Remover do Catálogo"
                                  style={{
                                    padding: '6px 8px',
                                    borderRadius: '6px',
                                    backgroundColor: 'rgba(239, 68, 68, 0.1)',
                                    border: '1px solid rgba(239, 68, 68, 0.3)',
                                    color: '#f87171',
                                    fontSize: '11.5px',
                                    cursor: 'pointer',
                                    display: 'flex',
                                    alignItems: 'center',
                                    gap: '4px',
                                  }}
                                >
                                  <Trash2 size={12} />
                                </button>
                              </div>
                            </td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>
                ) : (
                  /* MODO CARDS */
                  <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(310px, 1fr))', gap: '14px' }}>
                    {paginatedPremises.map((p) => (
                      <div
                        key={p.id}
                        style={{
                          padding: '16px',
                          borderRadius: '10px',
                          backgroundColor: '#0c1017',
                          border: '1px solid rgba(255, 255, 255, 0.08)',
                          display: 'flex',
                          flexDirection: 'column',
                          justifyContent: 'space-between',
                          gap: '12px',
                        }}
                      >
                        <div>
                          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', gap: '8px', marginBottom: '8px' }}>
                            <strong style={{ fontSize: '14px', color: '#f8fafc' }}>{p.title}</strong>
                            <span style={{ fontSize: '10px', color: '#facc15', backgroundColor: 'rgba(234, 179, 8, 0.15)', border: '1px solid rgba(234, 179, 8, 0.3)', padding: '2px 6px', borderRadius: '4px', fontWeight: 800 }}>
                              D20: {p.dramatic_focus_slider || 10}
                            </span>
                          </div>

                          {/* Tags no Card */}
                          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '4px', marginBottom: '8px' }}>
                            {(p.tags || []).map((t) => (
                              <span
                                key={t}
                                style={{
                                  fontSize: '10px',
                                  padding: '2px 6px',
                                  borderRadius: '4px',
                                  backgroundColor: 'rgba(56, 189, 248, 0.12)',
                                  border: '1px solid rgba(56, 189, 248, 0.25)',
                                  color: '#38bdf8',
                                  fontWeight: 600,
                                }}
                              >
                                #{t}
                              </span>
                            ))}
                          </div>

                          <p style={{ fontSize: '12px', color: '#94a3b8', margin: '0 0 8px 0', lineHeight: 1.45, maxHeight: '80px', overflowY: 'auto' }}>
                            {p.premise}
                          </p>

                          <div style={{ fontSize: '11px', color: '#64748b' }}>
                            <em>{p.literary_style}</em>
                          </div>
                        </div>

                        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', borderTop: '1px solid rgba(255, 255, 255, 0.06)', paddingTop: '10px' }}>
                          <button
                            type="button"
                            onClick={() => setInspectStoryModal(p)}
                            data-testid={`admin-inspect-premise-${p.id}`}
                            style={{
                              padding: '6px 10px',
                              borderRadius: '6px',
                              backgroundColor: 'rgba(56, 189, 248, 0.1)',
                              border: '1px solid rgba(56, 189, 248, 0.3)',
                              color: '#38bdf8',
                              fontSize: '11.5px',
                              cursor: 'pointer',
                              display: 'flex',
                              alignItems: 'center',
                              gap: '4px',
                            }}
                          >
                            <Eye size={12} /> Inspecionar
                          </button>

                          <button
                            type="button"
                            onClick={() => handleDeletePremise(p.id)}
                            data-testid={`admin-delete-premise-${p.id}`}
                            style={{
                              padding: '6px 10px',
                              borderRadius: '6px',
                              backgroundColor: 'rgba(239, 68, 68, 0.1)',
                              border: '1px solid rgba(239, 68, 68, 0.3)',
                              color: '#f87171',
                              fontSize: '11.5px',
                              cursor: 'pointer',
                              display: 'flex',
                              alignItems: 'center',
                              gap: '4px',
                            }}
                          >
                            <Trash2 size={12} /> Remover
                          </button>
                        </div>
                      </div>
                    ))}
                  </div>
                )}

                {/* Barra de Paginação para suportar até 1.000 histórias */}
                {totalCatalogPages > 1 && (
                  <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', backgroundColor: '#070a0f', padding: '10px 16px', borderRadius: '8px', border: '1px solid rgba(255, 255, 255, 0.08)' }}>
                    <span style={{ fontSize: '12px', color: '#94a3b8' }}>
                      Página <strong>{currentCatalogPageSafe}</strong> de <strong>{totalCatalogPages}</strong> ({filteredPremises.length} histórias filtradas)
                    </span>
                    <div style={{ display: 'flex', gap: '8px' }}>
                      <button
                        type="button"
                        disabled={currentCatalogPageSafe <= 1}
                        onClick={() => setCatalogPage((prev) => Math.max(1, prev - 1))}
                        data-testid="admin-catalog-prev-page"
                        style={{
                          padding: '6px 12px',
                          borderRadius: '6px',
                          backgroundColor: currentCatalogPageSafe <= 1 ? 'rgba(255,255,255,0.03)' : 'rgba(56, 189, 248, 0.15)',
                          border: '1px solid rgba(255,255,255,0.1)',
                          color: currentCatalogPageSafe <= 1 ? '#475569' : '#38bdf8',
                          fontSize: '11.5px',
                          cursor: currentCatalogPageSafe <= 1 ? 'not-allowed' : 'pointer',
                          display: 'flex',
                          alignItems: 'center',
                          gap: '4px',
                        }}
                      >
                        <ChevronLeft size={13} /> Anterior
                      </button>
                      <button
                        type="button"
                        disabled={currentCatalogPageSafe >= totalCatalogPages}
                        onClick={() => setCatalogPage((prev) => Math.min(totalCatalogPages, prev + 1))}
                        data-testid="admin-catalog-next-page"
                        style={{
                          padding: '6px 12px',
                          borderRadius: '6px',
                          backgroundColor: currentCatalogPageSafe >= totalCatalogPages ? 'rgba(255,255,255,0.03)' : 'rgba(56, 189, 248, 0.15)',
                          border: '1px solid rgba(255,255,255,0.1)',
                          color: currentCatalogPageSafe >= totalCatalogPages ? '#475569' : '#38bdf8',
                          fontSize: '11.5px',
                          cursor: currentCatalogPageSafe >= totalCatalogPages ? 'not-allowed' : 'pointer',
                          display: 'flex',
                          alignItems: 'center',
                          gap: '4px',
                        }}
                      >
                        Próxima <ChevronRight size={13} />
                      </button>
                    </div>
                  </div>
                )}

                {/* MODAL DE INSPEÇÃO COMPLETA DA HISTÓRIA */}
                {inspectStoryModal && (
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
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'center',
                      zIndex: 9999,
                      padding: '20px',
                    }}
                  >
                    <div
                      className="modal-content-fade"
                      style={{
                        backgroundColor: '#0c1017',
                        border: '1px solid rgba(56, 189, 248, 0.4)',
                        borderRadius: '14px',
                        width: '100%',
                        maxWidth: '750px',
                        maxHeight: '90vh',
                        display: 'flex',
                        flexDirection: 'column',
                        overflow: 'hidden',
                        boxShadow: '0 20px 40px rgba(0,0,0,0.8)',
                      }}
                    >
                      <div
                        style={{
                          padding: '16px 20px',
                          borderBottom: '1px solid rgba(255,255,255,0.1)',
                          display: 'flex',
                          justifyContent: 'space-between',
                          alignItems: 'center',
                          backgroundColor: '#070a0e',
                        }}
                      >
                        <div>
                          <h3 style={{ fontSize: '16px', fontWeight: 900, color: '#f8fafc', margin: 0 }}>
                            {inspectStoryModal.title}
                          </h3>
                          <span style={{ fontSize: '11px', color: '#94a3b8' }}>
                            Estilo: {inspectStoryModal.literary_style} | Foco D20: {inspectStoryModal.dramatic_focus_slider || 10}
                          </span>
                        </div>
                        <button
                          type="button"
                          onClick={() => setInspectStoryModal(null)}
                          data-testid="admin-close-inspect-modal"
                          style={{
                            background: 'none',
                            border: 'none',
                            color: '#94a3b8',
                            cursor: 'pointer',
                            padding: '6px',
                          }}
                        >
                          <X size={20} />
                        </button>
                      </div>

                      <div style={{ padding: '20px', overflowY: 'auto', display: 'flex', flexDirection: 'column', gap: '14px' }}>
                        {/* Tags */}
                        <div>
                          <span style={{ fontSize: '11px', fontWeight: 800, color: '#38bdf8', textTransform: 'uppercase' }}>
                            🏷️ Tags / Categorias:
                          </span>
                          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px', marginTop: '6px' }}>
                            {normalizeTags(inspectStoryModal.tags).map((t) => (
                              <span
                                key={t}
                                style={{
                                  padding: '3px 8px',
                                  borderRadius: '6px',
                                  backgroundColor: 'rgba(56, 189, 248, 0.15)',
                                  border: '1px solid rgba(56, 189, 248, 0.3)',
                                  color: '#38bdf8',
                                  fontSize: '11px',
                                  fontWeight: 700,
                                }}
                              >
                                #{t.toLowerCase()}
                              </span>
                            ))}
                          </div>
                        </div>

                        {/* Texto Integral */}
                        <div>
                          <span style={{ fontSize: '11px', fontWeight: 800, color: '#facc15', textTransform: 'uppercase' }}>
                            📜 Narrativa Integral da Premissa:
                          </span>
                          <div
                            style={{
                              backgroundColor: '#070a0e',
                              padding: '14px',
                              borderRadius: '8px',
                              border: '1px solid rgba(255,255,255,0.08)',
                              color: '#cbd5e1',
                              fontSize: '12.5px',
                              lineHeight: 1.6,
                              whiteSpace: 'pre-wrap',
                              maxHeight: '260px',
                              overflowY: 'auto',
                              marginTop: '6px',
                            }}
                          >
                            {inspectStoryModal.premise}
                          </div>
                        </div>

                        {/* Monstros & Chefes se houver */}
                        {inspectStoryModal.custom_monsters && inspectStoryModal.custom_monsters.length > 0 && (
                          <div>
                            <span style={{ fontSize: '11px', fontWeight: 800, color: '#f87171', textTransform: 'uppercase' }}>
                              👹 Ameaças e Monstros Customizados ({inspectStoryModal.custom_monsters.length}):
                            </span>
                            <div style={{ display: 'flex', flexDirection: 'column', gap: '6px', marginTop: '6px' }}>
                              {inspectStoryModal.custom_monsters.map((m: any, idx: number) => (
                                <div key={idx} style={{ backgroundColor: 'rgba(239, 68, 68, 0.08)', padding: '8px 12px', borderRadius: '6px', border: '1px solid rgba(239, 68, 68, 0.25)', fontSize: '12px' }}>
                                  <strong style={{ color: '#fca5a5' }}>{m.name}</strong> ({m.type}) — CA: {m.ac} | HP: {m.hp} | ND: {m.cr}
                                </div>
                              ))}
                            </div>
                          </div>
                        )}

                        {/* Overrides de Subclasse se houver */}
                        {inspectStoryModal.subclass_overrides && inspectStoryModal.subclass_overrides.length > 0 && (
                          <div>
                            <span style={{ fontSize: '11px', fontWeight: 800, color: '#d8b4fe', textTransform: 'uppercase' }}>
                              👑 Subclasses &amp; Regras Especiais ({inspectStoryModal.subclass_overrides.length}):
                            </span>
                            <div style={{ display: 'flex', flexDirection: 'column', gap: '6px', marginTop: '6px' }}>
                              {inspectStoryModal.subclass_overrides.map((sub: any, idx: number) => (
                                <div key={idx} style={{ backgroundColor: 'rgba(168, 85, 247, 0.08)', padding: '8px 12px', borderRadius: '6px', border: '1px solid rgba(168, 85, 247, 0.25)', fontSize: '12px' }}>
                                  <strong style={{ color: '#e9d5ff' }}>{sub.name}</strong> ({sub.parent_class}) — {sub.description}
                                </div>
                              ))}
                            </div>
                          </div>
                        )}
                      </div>

                      <div style={{ padding: '12px 20px', borderTop: '1px solid rgba(255,255,255,0.1)', display: 'flex', justifyContent: 'flex-end', backgroundColor: '#070a0e' }}>
                        <button
                          type="button"
                          onClick={() => setInspectStoryModal(null)}
                          style={{
                            padding: '8px 18px',
                            borderRadius: '6px',
                            backgroundColor: 'rgba(255,255,255,0.08)',
                            border: '1px solid rgba(255,255,255,0.15)',
                            color: '#ffffff',
                            fontSize: '12px',
                            cursor: 'pointer',
                          }}
                        >
                          Fechar
                        </button>
                      </div>
                    </div>
                  </div>
                )}
              </div>
            );
          })()}
        </div>
      )}

      {/* ABA: USUÁRIOS & ROLES */}
      {activeTab === 'users' && (
        <div style={{ backgroundColor: '#0c1017', border: '1px solid rgba(255, 255, 255, 0.08)', borderRadius: '12px', padding: '20px' }}>
          <h3 style={{ fontSize: '15px', color: '#f8fafc', margin: '0 0 12px 0' }}>Usuários Cadastrados na Plataforma</h3>
          <div style={{ overflowX: 'auto' }}>
            <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: '12.5px' }}>
              <thead>
                <tr style={{ borderBottom: '1px solid rgba(255, 255, 255, 0.1)', color: '#94a3b8', textAlign: 'left' }}>
                  <th style={{ padding: '8px' }}>Nome</th>
                  <th style={{ padding: '8px' }}>Email</th>
                  <th style={{ padding: '8px' }}>Papel (Role)</th>
                  <th style={{ padding: '8px' }}>Plano</th>
                  <th style={{ padding: '8px' }}>Cadastro</th>
                </tr>
              </thead>
              <tbody>
                {users.map((u) => (
                  <tr key={u.id} style={{ borderBottom: '1px solid rgba(255, 255, 255, 0.04)' }}>
                    <td style={{ padding: '10px 8px', fontWeight: 700, color: '#f8fafc' }}>{u.name}</td>
                    <td style={{ padding: '10px 8px', color: '#94a3b8' }}>{u.email}</td>
                    <td style={{ padding: '10px 8px' }}>
                      <span
                        style={{
                          padding: '2px 8px',
                          borderRadius: '4px',
                          fontSize: '11px',
                          fontWeight: 800,
                          backgroundColor:
                            u.role === 'admin'
                              ? 'rgba(234, 179, 8, 0.15)'
                              : u.role === 'host'
                              ? 'rgba(56, 189, 248, 0.15)'
                              : 'rgba(255, 255, 255, 0.05)',
                          color:
                            u.role === 'admin' ? '#facc15' : u.role === 'host' ? '#38bdf8' : '#cbd5e1',
                        }}
                      >
                        {u.role.toUpperCase()}
                      </span>
                    </td>
                    <td style={{ padding: '10px 8px', color: '#cbd5e1' }}>{u.plan}</td>
                    <td style={{ padding: '10px 8px', color: '#64748b', fontSize: '11px' }}>
                      {new Date(u.created_at).toLocaleDateString()}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {/* MODAL DE PRÉ-VISUALIZAÇÃO ANTES DE PUBLICAR */}
      {isPreviewModalOpen && previewData && (
        <div
          className="modal-backdrop-fade"
          data-testid="admin-preview-modal"
          style={{
            position: 'fixed',
            inset: 0,
            backgroundColor: 'rgba(0, 0, 0, 0.85)',
            backdropFilter: 'blur(8px)',
            zIndex: 9999,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            padding: '20px',
          }}
        >
          <div
            className="modal-content-fade"
            style={{
              backgroundColor: '#0d1117',
              border: '1px solid #c084fc',
              borderRadius: '16px',
              maxWidth: '850px',
              width: '100%',
              maxHeight: '90vh',
              display: 'flex',
              flexDirection: 'column',
              boxShadow: '0 25px 60px rgba(168, 85, 247, 0.35)',
              overflow: 'hidden',
            }}
          >
            {/* Modal Header */}
            <div
              style={{
                padding: '18px 24px',
                borderBottom: '1px solid rgba(255, 255, 255, 0.1)',
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                backgroundColor: '#111622',
              }}
            >
              <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
                <div
                  style={{
                    padding: '8px',
                    borderRadius: '8px',
                    backgroundColor: 'rgba(168, 85, 247, 0.2)',
                    color: '#c084fc',
                  }}
                >
                  <Eye size={20} />
                </div>
                <div>
                  <h3 style={{ fontSize: '16px', fontWeight: 900, color: '#fff', margin: 0 }}>
                    Pré-Visualização Editorial da Premissa
                  </h3>
                  <span style={{ fontSize: '11px', color: '#94a3b8' }}>
                    Revisão com Duas Vias de IA (Gemini + DeepSeek Linter)
                  </span>
                </div>
              </div>
              <button
                onClick={() => setIsPreviewModalOpen(false)}
                style={{
                  background: 'none',
                  border: 'none',
                  color: '#94a3b8',
                  cursor: 'pointer',
                  padding: '6px',
                }}
              >
                <X size={20} />
              </button>
            </div>

            {/* Modal Content Scrollable */}
            <div style={{ padding: '24px', overflowY: 'auto', display: 'flex', flexDirection: 'column', gap: '16px' }}>
              <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                <div>
                  <span style={{ fontSize: '11px', fontWeight: 800, color: '#facc15', textTransform: 'uppercase' }}>
                    Título da História:
                  </span>
                  <div style={{ fontSize: '18px', fontWeight: 900, color: '#ffffff', marginTop: '2px' }}>
                    {previewData.title}
                  </div>
                </div>
                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '8px', alignItems: 'center' }}>
                  <div style={{ fontSize: '12px', padding: '4px 10px', backgroundColor: 'rgba(56, 189, 248, 0.1)', border: '1px solid rgba(56, 189, 248, 0.3)', borderRadius: '6px', color: '#38bdf8' }}>
                    <strong>Estilo Literário:</strong> {previewData.literary_style || style}
                  </div>
                  {previewData.analysis?.tags?.length > 0 && (
                    <div style={{ display: 'flex', flexWrap: 'wrap', gap: '4px', alignItems: 'center' }}>
                      <span style={{ fontSize: '11px', color: '#94a3b8', fontWeight: 700 }}>Tags da IA:</span>
                      {normalizeTags(previewData.analysis.tags).map((t: string, idx: number) => (
                        <span key={idx} style={{ fontSize: '11px', padding: '2px 8px', borderRadius: '4px', backgroundColor: 'rgba(250, 204, 21, 0.15)', border: '1px solid rgba(250, 204, 21, 0.3)', color: '#facc15', fontWeight: 700 }}>
                          #{t.toLowerCase()}
                        </span>
                      ))}
                    </div>
                  )}
                </div>
              </div>

              {/* Comparativo de Texto */}
              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '14px' }}>
                <div style={{ backgroundColor: '#070a0e', padding: '14px', borderRadius: '8px', border: '1px solid rgba(255,255,255,0.08)' }}>
                  <span style={{ fontSize: '11px', fontWeight: 800, color: '#94a3b8', textTransform: 'uppercase' }}>
                    Texto Bruto Submetido:
                  </span>
                  <p style={{ fontSize: '12px', color: '#cbd5e1', whiteSpace: 'pre-wrap', maxHeight: '180px', overflowY: 'auto', lineHeight: 1.5, marginTop: '8px' }}>
                    {previewData.raw_premise}
                  </p>
                </div>

                <div style={{ backgroundColor: 'rgba(168, 85, 247, 0.08)', padding: '14px', borderRadius: '8px', border: '1px solid rgba(168, 85, 247, 0.4)' }}>
                  <span style={{ fontSize: '11px', fontWeight: 800, color: '#d8b4fe', textTransform: 'uppercase', display: 'flex', alignItems: 'center', gap: '6px' }}>
                    <Sparkles size={13} color="#c084fc" /> Versão Refinada pela IA:
                  </span>
                  <p style={{ fontSize: '12px', color: '#f1f5f9', whiteSpace: 'pre-wrap', maxHeight: '180px', overflowY: 'auto', lineHeight: 1.5, marginTop: '8px' }}>
                    {previewData.refined_premise}
                  </p>
                </div>
              </div>

              {/* Metadados e Diagnóstico da IA */}
              {previewData.analysis && (
                <div style={{ backgroundColor: '#111622', padding: '14px', borderRadius: '8px', border: '1px solid rgba(56, 189, 248, 0.3)' }}>
                  <div style={{ fontSize: '12px', fontWeight: 800, color: '#38bdf8', marginBottom: '8px' }}>
                    Diagnóstico Temático & Regras Mecânicas
                  </div>
                  <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: '8px', fontSize: '12px' }}>
                    <div>
                      <span style={{ color: '#94a3b8' }}>Tom da Aventura:</span>{' '}
                      <strong style={{ color: '#fff' }}>{previewData.analysis.adventure_tone || 'Equilibrado'}</strong>
                    </div>
                    <div>
                      <span style={{ color: '#94a3b8' }}>Party Sugerida:</span>{' '}
                      <strong style={{ color: '#fff' }}>{previewData.analysis.recommended_party_size || '4 jogadores'}</strong>
                    </div>
                    <div>
                      <span style={{ color: '#94a3b8' }}>Overrides de Subclasse:</span>{' '}
                      <strong style={{ color: '#c084fc' }}>{previewData.analysis.subclass_overrides?.length || 0} detectado(s)</strong>
                    </div>
                    <div>
                      <span style={{ color: '#94a3b8' }}>Magias Homebrew:</span>{' '}
                      <strong style={{ color: '#c084fc' }}>{previewData.analysis.custom_spells?.length || 0} detectada(s)</strong>
                    </div>
                    <div>
                      <span style={{ color: '#94a3b8' }}>Monstros Homebrew:</span>{' '}
                      <strong style={{ color: '#f87171' }}>{previewData.analysis.custom_monsters?.length || 0} detectado(s)</strong>
                    </div>
                  </div>

                  {/* Requisitos de Personagens & Ganchos de Roleplay */}
                  {previewData.analysis.party_requirements?.length > 0 && (
                    <div style={{ marginTop: '10px', paddingTop: '10px', borderTop: '1px solid rgba(255,255,255,0.08)' }}>
                      <span style={{ fontSize: '11px', fontWeight: 800, color: '#38bdf8' }}>
                        👥 Personagens & Requisitos do Grupo (Roleplay D&D 5e):
                      </span>
                      {previewData.analysis.party_requirements.map((req: any, idx: number) => (
                        <div key={idx} style={{ fontSize: '11.5px', color: '#e2e8f0', marginTop: '4px' }}>
                          • {req.is_mandatory ? <strong style={{ color: '#ef4444' }}>[OBRIGATÓRIO]</strong> : <span style={{ color: '#22c55e' }}>[SUGESTÃO]</span>}{' '}
                          {req.required_name && <span>Personagem: <strong style={{ color: '#facc15' }}>{req.required_name}</strong>. </span>}
                          {req.required_class && <span>Classe: <strong style={{ color: '#a855f7' }}>{req.required_class}</strong>. </span>}
                          {req.description}
                        </div>
                      ))}
                    </div>
                  )}

                  {/* Ganchos da História */}
                  {previewData.analysis.party_hooks?.length > 0 && (
                    <div style={{ marginTop: '10px', paddingTop: '10px', borderTop: '1px solid rgba(255,255,255,0.08)' }}>
                      <span style={{ fontSize: '11px', fontWeight: 800, color: '#facc15' }}>
                        📜 Ganchos Iniciais da Comitiva:
                      </span>
                      {previewData.analysis.party_hooks.map((hook: string, idx: number) => (
                        <div key={idx} style={{ fontSize: '11.5px', color: '#cbd5e1', marginTop: '4px' }}>
                          • {hook}
                        </div>
                      ))}
                    </div>
                  )}

                  {/* Overrides de Subclasse */}
                  {previewData.analysis.subclass_overrides?.length > 0 && (
                    <div style={{ marginTop: '10px', paddingTop: '10px', borderTop: '1px solid rgba(255,255,255,0.08)' }}>
                      <span style={{ fontSize: '11px', fontWeight: 800, color: '#d8b4fe' }}>
                        👑 Overrides de Regras & Subclasses:
                      </span>
                      {previewData.analysis.subclass_overrides.map((ov: any, idx: number) => (
                        <div key={idx} style={{ fontSize: '11.5px', color: '#e2e8f0', marginTop: '4px' }}>
                          • <strong>{ov.name}</strong> ({ov.parent_class}) substitui <em>{ov.replaces || 'SRD Padrão'}</em>: {ov.description}
                        </div>
                      ))}
                    </div>
                  )}

                  {/* Magias Homebrew */}
                  {previewData.analysis.custom_spells?.length > 0 && (
                    <div style={{ marginTop: '10px', paddingTop: '10px', borderTop: '1px solid rgba(255,255,255,0.08)' }}>
                      <span style={{ fontSize: '11px', fontWeight: 800, color: '#d8b4fe' }}>
                        ✨ Magias Homebrew Inclusas:
                      </span>
                      {previewData.analysis.custom_spells.map((sp: any, idx: number) => (
                        <div key={idx} style={{ fontSize: '11.5px', color: '#e2e8f0', marginTop: '4px' }}>
                          • <strong>{sp.name}</strong> ({sp.level}º Círculo - {sp.classes.join(', ')}): {sp.description}
                        </div>
                      ))}
                    </div>
                  )}

                  {/* Monstros & Antagonistas Homebrew */}
                  {previewData.analysis.custom_monsters?.length > 0 && (
                    <div style={{ marginTop: '12px', paddingTop: '10px', borderTop: '1px solid rgba(255,255,255,0.08)' }}>
                      <div style={{ fontSize: '12px', fontWeight: 900, color: '#f87171', marginBottom: '8px', display: 'flex', alignItems: 'center', gap: '6px' }}>
                        👹 Monstros & Chefes Homebrew Detectados / Forjados ({previewData.analysis.custom_monsters.length}):
                      </div>
                      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(240px, 1fr))', gap: '8px' }}>
                        {previewData.analysis.custom_monsters.map((m: any, idx: number) => (
                          <div
                            key={idx}
                            style={{
                              backgroundColor: '#0a0d14',
                              border: '1px solid rgba(239, 68, 68, 0.4)',
                              borderRadius: '8px',
                              padding: '10px 12px',
                            }}
                          >
                            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                              <strong style={{ color: '#fff', fontSize: '12.5px' }}>{m.name}</strong>
                              <span style={{ fontSize: '10px', fontWeight: 800, padding: '2px 6px', borderRadius: '4px', backgroundColor: 'rgba(239, 68, 68, 0.2)', color: '#fca5a5' }}>
                                ND {m.cr || '?'}
                              </span>
                            </div>
                            <div style={{ fontSize: '11px', color: '#94a3b8', marginTop: '2px' }}>
                              {m.type || 'Monstro Customizado'}
                            </div>
                            <div style={{ display: 'flex', gap: '10px', fontSize: '11px', marginTop: '6px' }}>
                              <span>CA: <strong style={{ color: '#facc15' }}>{m.ac || 10}</strong></span>
                              <span>PV: <strong style={{ color: '#ef4444' }}>{m.hp || 50}</strong></span>
                              <span>Vel: <strong style={{ color: '#38bdf8' }}>{m.speed || '9m'}</strong></span>
                            </div>
                            {m.description && (
                              <div style={{ fontSize: '10.5px', color: '#cbd5e1', marginTop: '6px', lineHeight: 1.4 }}>
                                {m.description}
                              </div>
                            )}
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              )}
            </div>

            {/* Modal Actions */}
            <div
              style={{
                padding: '16px 24px',
                borderTop: '1px solid rgba(255, 255, 255, 0.1)',
                display: 'flex',
                justifyContent: 'flex-end',
                gap: '12px',
                backgroundColor: '#111622',
              }}
            >
              <button
                type="button"
                onClick={() => setIsPreviewModalOpen(false)}
                style={{
                  padding: '10px 18px',
                  backgroundColor: '#1e293b',
                  border: '1px solid #334155',
                  borderRadius: '8px',
                  color: '#cbd5e1',
                  fontWeight: 700,
                  fontSize: '12.5px',
                  cursor: 'pointer',
                }}
              >
                Voltar e Editar
              </button>

              <button
                type="button"
                onClick={() => confirmPublishFromPreview(true)}
                disabled={isSaving}
                data-testid="admin-publish-original-btn"
                style={{
                  padding: '10px 20px',
                  backgroundColor: '#0f172a',
                  border: '1.5px solid #22c55e',
                  borderRadius: '8px',
                  color: '#4ade80',
                  fontWeight: 800,
                  fontSize: '12.5px',
                  cursor: isSaving ? 'not-allowed' : 'pointer',
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                }}
              >
                {isSaving ? <Sparkles size={14} className="animate-spin" /> : <Check size={14} />}
                <span>Publicar Texto Original (100% Preservado)</span>
              </button>

              <button
                type="button"
                onClick={() => confirmPublishFromPreview(false)}
                disabled={isSaving}
                data-testid="admin-confirm-publish-btn"
                style={{
                  padding: '10px 22px',
                  background: 'linear-gradient(135deg, #facc15 0%, #ca8a04 100%)',
                  border: 'none',
                  borderRadius: '8px',
                  color: '#000',
                  fontWeight: 900,
                  fontSize: '12.5px',
                  cursor: isSaving ? 'not-allowed' : 'pointer',
                  display: 'flex',
                  alignItems: 'center',
                  gap: '8px',
                  boxShadow: '0 4px 15px rgba(250, 204, 21, 0.3)',
                }}
              >
                {isSaving ? <Sparkles size={15} className="animate-spin" /> : <Sparkles size={15} />}
                <span>Publicar Versão Refinada por IA</span>
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
