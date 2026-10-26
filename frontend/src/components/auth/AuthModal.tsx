import React, { useState } from 'react';
import { apiLogin, apiRegister } from '../../services/api';
import { setAuthSession, type AuthUser } from '../../services/auth';
import { UserCheck, Key, Mail, User, Crown, X } from 'lucide-react';

interface AuthModalProps {
  isOpen: boolean;
  onClose?: () => void;
  onSuccess: (user: AuthUser) => void;
  initialMode?: 'login' | 'register';
  message?: string;
  canClose?: boolean;
}

export const AuthModal: React.FC<AuthModalProps> = ({
  isOpen,
  onClose,
  onSuccess,
  initialMode = 'login',
  message,
  canClose = true,
}) => {
  const [mode, setMode] = useState<'login' | 'register'>(initialMode);
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [name, setName] = useState('');
  const [role, setRole] = useState<'host' | 'player'>('host');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  React.useEffect(() => {
    if (isOpen) {
      setMode(initialMode);
      setError(null);
    }
  }, [isOpen, initialMode]);

  if (!isOpen) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setLoading(true);

    try {
      if (mode === 'login') {
        const res = await apiLogin({ email: email.trim(), password });
        setAuthSession(res.token, res.user);
        onSuccess(res.user);
        if (onClose) onClose();
      } else {
        const res = await apiRegister({
          email: email.trim(),
          name: name.trim() || 'Mestre da Mesa',
          password,
          role,
        });
        setAuthSession(res.token, res.user);
        onSuccess(res.user);
        if (onClose) onClose();
      }
    } catch (err: any) {
      setError(err.message || 'Falha na autenticação');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div
      className="modal-backdrop-fade"
      style={{
        position: 'fixed',
        inset: 0,
        backgroundColor: 'rgba(5, 7, 12, 0.88)',
        backdropFilter: 'blur(8px)',
        zIndex: 999999,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        padding: '16px',
      }}
    >
      <div
        className="modal-content-fade"
        data-testid="auth-modal"
        style={{
          width: '100%',
          maxWidth: '460px',
          backgroundColor: '#0c1017',
          border: '1px solid rgba(234, 179, 8, 0.4)',
          borderRadius: '16px',
          padding: '24px',
          boxShadow: '0 25px 60px -12px rgba(0, 0, 0, 0.85)',
          position: 'relative',
          display: 'flex',
          flexDirection: 'column',
          gap: '16px',
        }}
      >
        {/* Fechar */}
        {canClose && onClose && (
          <button
            type="button"
            onClick={onClose}
            data-testid="close-auth-modal"
            style={{
              position: 'absolute',
              top: '16px',
              right: '16px',
              background: 'rgba(255, 255, 255, 0.08)',
              border: 'none',
              borderRadius: '50%',
              color: '#94a3b8',
              width: '30px',
              height: '30px',
              cursor: 'pointer',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
            }}
          >
            <X size={16} />
          </button>
        )}

        {/* Cabeçalho */}
        <div style={{ textAlign: 'center' }}>
          <div
            style={{
              display: 'inline-flex',
              padding: '10px',
              borderRadius: '50%',
              backgroundColor: 'rgba(234, 179, 8, 0.12)',
              border: '1px solid rgba(234, 179, 8, 0.3)',
              marginBottom: '10px',
            }}
          >
            <Crown size={24} color="#facc15" />
          </div>
          <h2 style={{ fontSize: '18px', fontWeight: 900, color: '#ffffff', margin: '0 0 6px 0', textTransform: 'uppercase', letterSpacing: '0.04em' }}>
            {mode === 'login' ? 'Portal de Acesso SaaS' : 'Cadastro de Mestre (Host)'}
          </h2>
          <p style={{ fontSize: '12px', color: '#94a3b8', margin: 0, lineHeight: 1.4 }}>
            {message ||
              (mode === 'login'
                ? 'Entre para gerenciar e narrar campanhas exclusivas D&D 5e.'
                : 'Cadastre sua conta de Host para criar mesas, salvar mundos e gerenciar aventuras.')}
          </p>
        </div>

        {/* Alternador Login / Cadastro */}
        <div
          style={{
            display: 'flex',
            backgroundColor: 'rgba(255, 255, 255, 0.04)',
            padding: '3px',
            borderRadius: '8px',
            gap: '4px',
          }}
        >
          <button
            type="button"
            onClick={() => {
              setMode('login');
              setError(null);
            }}
            data-testid="tab-login"
            style={{
              flex: 1,
              padding: '8px',
              border: 'none',
              borderRadius: '6px',
              fontSize: '12px',
              fontWeight: 700,
              cursor: 'pointer',
              backgroundColor: mode === 'login' ? 'rgba(234, 179, 8, 0.2)' : 'transparent',
              color: mode === 'login' ? '#facc15' : '#94a3b8',
            }}
          >
            Entrar (Login)
          </button>
          <button
            type="button"
            onClick={() => {
              setMode('register');
              setError(null);
            }}
            data-testid="tab-register"
            style={{
              flex: 1,
              padding: '8px',
              border: 'none',
              borderRadius: '6px',
              fontSize: '12px',
              fontWeight: 700,
              cursor: 'pointer',
              backgroundColor: mode === 'register' ? 'rgba(56, 189, 248, 0.2)' : 'transparent',
              color: mode === 'register' ? '#38bdf8' : '#94a3b8',
            }}
          >
            Criar Conta (Host)
          </button>
        </div>

        {/* Mensagem de Erro */}
        {error && (
          <div
            style={{
              padding: '10px 14px',
              backgroundColor: 'rgba(239, 68, 68, 0.12)',
              border: '1px solid rgba(239, 68, 68, 0.4)',
              borderRadius: '8px',
              color: '#fca5a5',
              fontSize: '12px',
            }}
          >
            ⚠️ {error}
          </div>
        )}

        {/* Formulário */}
        <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
          {mode === 'register' && (
            <div>
              <label style={{ display: 'flex', alignItems: 'center', gap: '5px', fontSize: '11px', color: '#94a3b8', fontWeight: 700, textTransform: 'uppercase', marginBottom: '4px' }}>
                <User size={12} /> Nome de Mestre / Exibição *
              </label>
              <input
                type="text"
                required
                placeholder="Ex: Mestre Gandalf"
                value={name}
                onChange={(e) => setName(e.target.value)}
                data-testid="auth-name-input"
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
          )}

          <div>
            <label style={{ display: 'flex', alignItems: 'center', gap: '5px', fontSize: '11px', color: '#94a3b8', fontWeight: 700, textTransform: 'uppercase', marginBottom: '4px' }}>
              <Mail size={12} /> Email *
            </label>
            <input
              type="email"
              required
              placeholder="seu-email@exemplo.com"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              data-testid="auth-email-input"
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
            <label style={{ display: 'flex', alignItems: 'center', gap: '5px', fontSize: '11px', color: '#94a3b8', fontWeight: 700, textTransform: 'uppercase', marginBottom: '4px' }}>
              <Key size={12} /> Senha (Criptografia Argon2id) *
            </label>
            <input
              type="password"
              required
              placeholder="••••••••"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              data-testid="auth-password-input"
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

          {mode === 'register' && (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
              <label style={{ fontSize: '11px', color: '#94a3b8', fontWeight: 700, textTransform: 'uppercase' }}>
                Tipo de Perfil
              </label>
              <div style={{ display: 'flex', gap: '8px' }}>
                <label
                  style={{
                    flex: 1,
                    display: 'flex',
                    alignItems: 'center',
                    gap: '6px',
                    padding: '8px 10px',
                    borderRadius: '8px',
                    backgroundColor: role === 'host' ? 'rgba(234, 179, 8, 0.15)' : 'rgba(255, 255, 255, 0.03)',
                    border: role === 'host' ? '1px solid #facc15' : '1px solid rgba(255, 255, 255, 0.1)',
                    fontSize: '11.5px',
                    color: '#e2e8f0',
                    cursor: 'pointer',
                  }}
                >
                  <input
                    type="radio"
                    name="role"
                    checked={role === 'host'}
                    onChange={() => setRole('host')}
                  />
                  <span>👑 <strong>Host</strong> (Criar Mesas)</span>
                </label>

                <label
                  style={{
                    flex: 1,
                    display: 'flex',
                    alignItems: 'center',
                    gap: '6px',
                    padding: '8px 10px',
                    borderRadius: '8px',
                    backgroundColor: role === 'player' ? 'rgba(56, 189, 248, 0.15)' : 'rgba(255, 255, 255, 0.03)',
                    border: role === 'player' ? '1px solid #38bdf8' : '1px solid rgba(255, 255, 255, 0.1)',
                    fontSize: '11.5px',
                    color: '#e2e8f0',
                    cursor: 'pointer',
                  }}
                >
                  <input
                    type="radio"
                    name="role"
                    checked={role === 'player'}
                    onChange={() => setRole('player')}
                  />
                  <span>⚔️ <strong>Jogador</strong></span>
                </label>
              </div>
            </div>
          )}

          <button
            type="submit"
            disabled={loading}
            data-testid="auth-submit-btn"
            style={{
              marginTop: '6px',
              padding: '12px',
              background: 'linear-gradient(135deg, #facc15 0%, #ca8a04 100%)',
              border: 'none',
              borderRadius: '8px',
              color: '#000',
              fontWeight: 900,
              fontSize: '13px',
              textTransform: 'uppercase',
              letterSpacing: '0.06em',
              cursor: loading ? 'not-allowed' : 'pointer',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              gap: '6px',
            }}
          >
            {loading ? (
              'Processando credenciais...'
            ) : mode === 'login' ? (
              <>
                <UserCheck size={16} /> Entrar na Plataforma
              </>
            ) : (
              <>
                <Crown size={16} /> Concluir Cadastro de Host
              </>
            )}
          </button>
        </form>

        {/* Rodapé: Segurança Argon2id */}
        <div style={{ borderTop: '1px solid rgba(255, 255, 255, 0.08)', paddingTop: '10px', textAlign: 'center' }}>
          <span style={{ fontSize: '11px', color: '#64748b' }}>Acesso Seguro Multi-Tenant • Criptografia Argon2id</span>
        </div>
      </div>
    </div>
  );
};
