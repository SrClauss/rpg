export interface AuthUser {
  id: string;
  email: string;
  name: string;
  role: 'admin' | 'host' | 'player';
  plan: 'free' | 'host_pro';
  created_at: string;
}

const AUTH_TOKEN_KEY = 'rpg_saas_auth_token';
const AUTH_USER_KEY = 'rpg_saas_auth_user';

// Armazenamento isolado da Sessão Administrativa (Backoffice)
const ADMIN_AUTH_TOKEN_KEY = 'rpg_saas_admin_auth_token';
const ADMIN_AUTH_USER_KEY = 'rpg_saas_admin_auth_user';

export function getAuthToken(): string | null {
  return localStorage.getItem(AUTH_TOKEN_KEY);
}

export function getAuthUser(): AuthUser | null {
  try {
    const raw = localStorage.getItem(AUTH_USER_KEY);
    return raw ? JSON.parse(raw) : null;
  } catch {
    return null;
  }
}

export function setAuthSession(token: string, user: AuthUser) {
  localStorage.setItem(AUTH_TOKEN_KEY, token);
  localStorage.setItem(AUTH_USER_KEY, JSON.stringify(user));
}

export function clearAuthSession() {
  localStorage.removeItem(AUTH_TOKEN_KEY);
  localStorage.removeItem(AUTH_USER_KEY);
}

export function getAuthHeaders(): HeadersInit {
  const token = getAuthToken();
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
  };
  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }
  return headers;
}

// Funções exclusivas do Admin
export function getAdminToken(): string | null {
  return localStorage.getItem(ADMIN_AUTH_TOKEN_KEY);
}

export function getAdminUser(): AuthUser | null {
  try {
    const raw = localStorage.getItem(ADMIN_AUTH_USER_KEY);
    return raw ? JSON.parse(raw) : null;
  } catch {
    return null;
  }
}

export function setAdminSession(token: string, user: AuthUser) {
  localStorage.setItem(ADMIN_AUTH_TOKEN_KEY, token);
  localStorage.setItem(ADMIN_AUTH_USER_KEY, JSON.stringify(user));
}

export function clearAdminSession() {
  localStorage.removeItem(ADMIN_AUTH_TOKEN_KEY);
  localStorage.removeItem(ADMIN_AUTH_USER_KEY);
}

export function getAdminHeaders(): HeadersInit {
  const token = getAdminToken();
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
  };
  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }
  return headers;
}
