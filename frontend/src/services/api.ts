import type { CharacterSheet } from '../types/character';
import type { FreeformTextActionSubmission } from '../types/game';
import type { CampaignSession, ChronicleEntryData, LivingSceneData } from '../types/session';

const API_BASE_URL = 'http://localhost:3001/api';
const WS_BASE_URL = 'ws://localhost:3001/ws';

export interface BackendHealth {
  status: string;
  engine: string;
  rules: string;
  version: string;
  multiplayer?: string;
}

export async function checkBackendHealth(): Promise<BackendHealth | null> {
  try {
    const res = await fetch(`${API_BASE_URL}/health`);
    if (!res.ok) return null;
    return await res.json();
  } catch {
    return null;
  }
}

export interface PrebuiltStory {
  id: string;
  title: string;
  premise: string;
  literary_style?: string;
  dramatic_focus_slider?: number;
  ai_creative_freedom?: 'strict' | 'balanced' | 'free';
  custom_monsters?: any[];
  subclass_overrides?: any[];
  custom_spells?: any[];
  tags?: string[];
  recommended_starting_level?: number;
  raw_premise_submitted?: string;
  created_at?: string;
}

export interface PartyRequirement {
  requirement_type: string;
  description: string;
  is_mandatory: boolean;
  required_name?: string;
  required_class?: string;
  required_race?: string;
  suggested_options: string[];
}

export interface SubclassOverride {
  parent_class: string;
  replaces?: string | null;
  name: string;
  description: string;
  special_ability?: string;
  rhai_script?: string;
}

export interface CustomSpell {
  name: string;
  level: number;
  classes: string[];
  school?: string;
  description: string;
  rhai_script?: string;
}

export interface CustomMonster {
  name: string;
  type?: string;
  cr?: string | number;
  ac?: number;
  hp?: number;
  speed?: string;
  stats?: Record<string, number>;
  abilities?: string[];
  actions?: string[];
  description?: string;
}

export interface PremiseAnalysis {
  analysis_summary: string;
  adventure_tone: string;
  recommended_party_size: string;
  recommended_starting_level?: number;
  has_thematic_requirement: boolean;
  subclass_overrides?: SubclassOverride[];
  custom_spells?: CustomSpell[];
  custom_monsters?: CustomMonster[];
  party_requirements: PartyRequirement[];
  party_hooks: string[];
}

import { getAuthHeaders, getAdminHeaders } from './auth';

export async function apiRegister(payload: { email: string; name: string; password: string; role?: string }): Promise<any> {
  const res = await fetch(`${API_BASE_URL}/auth/register`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });
  const data = await res.json();
  if (!res.ok) throw new Error(data.message || 'Falha ao registrar conta');
  return data;
}

export async function apiLogin(payload: { email: string; password: string }): Promise<any> {
  const res = await fetch(`${API_BASE_URL}/auth/login`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });
  const data = await res.json();
  if (!res.ok) throw new Error(data.message || 'Credenciais inválidas');
  return data;
}

export async function apiAdminLogin(payload: { email: string; password: string }): Promise<any> {
  const res = await fetch(`${API_BASE_URL}/admin/auth/login`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });
  const data = await res.json();
  if (!res.ok) throw new Error(data.message || 'Acesso administrativo negado');
  return data;
}

export async function apiGetMe(): Promise<any> {
  const res = await fetch(`${API_BASE_URL}/auth/me`, {
    headers: getAuthHeaders(),
  });
  if (!res.ok) return null;
  const data = await res.json();
  return data.user;
}

export async function apiLogout(): Promise<void> {
  await fetch(`${API_BASE_URL}/auth/logout`, {
    method: 'POST',
    headers: getAuthHeaders(),
  }).catch(() => {});
}

// Admin APIs (protegidas com o token administrativo)
export async function fetchAdminMetrics(): Promise<any> {
  const res = await fetch(`${API_BASE_URL}/admin/metrics`, {
    headers: getAdminHeaders(),
  });
  if (!res.ok) {
    const err = await res.json().catch(() => ({ message: 'Acesso negado ao painel administrativo' }));
    throw new Error(err.message || 'Acesso negado');
  }
  return await res.json();
}

export async function fetchAdminUsers(): Promise<any[]> {
  const res = await fetch(`${API_BASE_URL}/admin/users`, {
    headers: getAdminHeaders(),
  });
  if (!res.ok) return [];
  const data = await res.json();
  return data.users || [];
}

export async function adminPreviewPremise(story: Partial<PrebuiltStory> & { auto_refine_ai?: boolean }): Promise<any> {
  const res = await fetch(`${API_BASE_URL}/admin/premises/preview`, {
    method: 'POST',
    headers: getAdminHeaders(),
    body: JSON.stringify(story),
  });
  const data = await res.json();
  if (!res.ok) throw new Error(data.message || 'Falha ao gerar prévia da premissa');
  return data;
}

export async function adminSavePremise(story: Partial<PrebuiltStory> & { auto_refine_ai?: boolean }): Promise<any> {
  const res = await fetch(`${API_BASE_URL}/admin/premises`, {
    method: 'POST',
    headers: getAdminHeaders(),
    body: JSON.stringify(story),
  });
  const data = await res.json();
  if (!res.ok) throw new Error(data.message || 'Falha ao gravar premissa administrativa');
  return data;
}

export async function adminDeletePremise(id: string): Promise<boolean> {
  const res = await fetch(`${API_BASE_URL}/admin/premises/${id}`, {
    method: 'DELETE',
    headers: getAdminHeaders(),
  });
  return res.ok;
}

export async function fetchSavedPremises(): Promise<PrebuiltStory[]> {
  try {
    const res = await fetch(`${API_BASE_URL}/premises/saved`);
    if (!res.ok) return [];
    const data = await res.json();
    return data.premises || [];
  } catch {
    return [];
  }
}

export async function analyzePremise(payload: {
  premise: string;
  literary_style?: string;
  ai_creative_freedom?: 'strict' | 'balanced' | 'free';
  dramatic_focus_slider?: number;
}): Promise<PremiseAnalysis> {
  const res = await fetch(`${API_BASE_URL}/campaign/analyze-premise`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });

  if (!res.ok) {
    const err = await res.json().catch(() => ({ message: 'Erro ao analisar premissa' }));
    throw new Error(err.message || 'Falha ao analisar premissa');
  }

  const data = await res.json();
  return data.analysis;
}

export async function resolveScene(
  campaignId: string,
  reason: 'objectives_completed' | 'party_departed' | 'party_defeated' = 'objectives_completed'
): Promise<any> {
  const res = await fetch(`${API_BASE_URL}/campaign/${campaignId}/scene/resolve`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ reason }),
  });

  if (!res.ok) {
    const err = await res.json().catch(() => ({ message: 'Erro ao concluir cena' }));
    throw new Error(err.message || 'Falha ao concluir cena');
  }

  return await res.json();
}

export async function deleteCampaign(campaignId: string): Promise<void> {
  const res = await fetch(`${API_BASE_URL}/campaign/${campaignId}`, {
    method: 'DELETE',
  });

  if (!res.ok) {
    const err = await res.json().catch(() => ({ message: 'Erro ao excluir campanha' }));
    throw new Error(err.message || 'Falha ao excluir campanha');
  }
}

export async function createCampaign(payload: {
  premise: string;
  literary_style: string;
  host_player_name?: string;
  host_class?: string;
  custom_monsters?: any[];
  ai_creative_freedom?: 'strict' | 'balanced' | 'free';
  story_focus?: 'combat' | 'roleplay' | 'd20';
  dramatic_focus_slider?: number;
  party?: any[];
}): Promise<CampaignSession> {
  const pName = payload.host_player_name || (payload.party && payload.party[0]?.name) || 'Aventureiro';
  const pClass = payload.host_class || (payload.party && (payload.party[0]?.class || payload.party[0]?.character_class)) || 'Fighter';
  const res = await fetch(`${API_BASE_URL}/campaign/create`, {
    method: 'POST',
    headers: getAuthHeaders(),
    body: JSON.stringify({
      ...payload,
      host_player_name: pName,
      host_class: pClass,
    }),
  });

  if (!res.ok) {
    const err = await res.json().catch(() => ({ message: 'Erro na criação da campanha' }));
    throw new Error(err.message || 'Falha ao criar campanha');
  }

  const data = await res.json();
  return {
    campaign_id: data.campaign_id,
    room_code: data.room_code,
    premise: payload.premise,
    literary_style: payload.literary_style,
    host_player_id: data.player_id,
    player_ids: (data.player_slots || []).map((s: any) => s.player_id),
    player_slots: data.player_slots || [],
    player_number: data.player_number || 1,
    player_code: data.player_code,
    current_act_number: data.current_act_number || 1,
    current_act_title: data.current_act_title || 'Ato I',
    is_host: true,
    player_id: data.player_id,
    player_name: pName,
    class_name: pClass,
  };
}

export async function joinCampaign(payload: {
  campaign_id?: string;
  room_code: string;
  player_name?: string;
  class_name?: string;
  player_code?: string;
  is_single_terminal?: boolean;
}): Promise<CampaignSession> {
  const pName = payload.player_name || 'Aventureiro Convidado';
  const pClass = payload.class_name || 'Rogue';
  const res = await fetch(`${API_BASE_URL}/campaign/join`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      ...payload,
      room_code: payload.room_code.trim().toUpperCase(),
      player_name: pName,
      class_name: pClass,
    }),
  });

  if (!res.ok) {
    const err = await res.json().catch(() => ({ message: 'Erro ao entrar na sala' }));
    throw new Error(err.message || 'Falha ao ingressar na sala');
  }

  const data = await res.json();
  const stateRes = await fetch(`${API_BASE_URL}/campaign/${data.campaign_id}/state`);
  const stateData = stateRes.ok ? await stateRes.json() : {};

  return {
    campaign_id: data.campaign_id,
    room_code: data.room_code,
    premise: stateData.premise || '',
    literary_style: stateData.literary_style || '',
    host_player_id: stateData.host_player_id || '',
    player_ids: stateData.player_ids || [data.player_id],
    player_slots: data.player_slots || [],
    player_number: data.player_number,
    player_code: data.player_code,
    current_act_number: stateData.current_act_number || 1,
    current_act_title: stateData.current_act_title || 'Ato Inicial',
    is_host: data.is_host ?? false,
    player_id: data.player_id,
    player_name: pName,
    class_name: pClass,
    is_single_terminal: payload.is_single_terminal,
  };
}

export async function getCampaignSlots(campaignId: string): Promise<{
  campaign_id: string;
  room_code: string;
  player_slots: Array<{
    player_number: number;
    player_id: string;
    player_name: string;
    class_name: string;
    player_code: string;
    is_host: boolean;
  }>;
}> {
  const res = await fetch(`${API_BASE_URL}/campaign/${campaignId}/slots`);
  if (!res.ok) throw new Error('Falha ao obter slots da campanha');
  return await res.json();
}

export async function addLocalPlayerToCampaign(payload: {
  campaign_id: string;
  player_name: string;
  class_name: string;
}): Promise<any> {
  const res = await fetch(`${API_BASE_URL}/campaign/add-local-player`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });
  if (!res.ok) {
    const err = await res.json().catch(() => ({ message: 'Erro ao adicionar jogador local' }));
    throw new Error(err.message || 'Falha ao adicionar jogador local');
  }
  return await res.json();
}

export async function getCampaignState(campaignId: string): Promise<Record<string, unknown>> {
  const res = await fetch(`${API_BASE_URL}/campaign/${campaignId}/state`);
  if (!res.ok) throw new Error('Falha ao obter estado da campanha');
  return await res.json();
}

export async function getCampaignChronicle(campaignId: string): Promise<ChronicleEntryData[]> {
  const res = await fetch(`${API_BASE_URL}/campaign/${campaignId}/chronicle`);
  if (!res.ok) throw new Error('Falha ao carregar crônica');
  const data = await res.json();
  return data.entries || [];
}

export async function getCampaignEntities(campaignId: string): Promise<{
  world_entities: Array<Record<string, unknown>>;
  player_entities: Array<Record<string, unknown>>;
}> {
  const res = await fetch(`${API_BASE_URL}/campaign/${campaignId}/entities`);
  if (!res.ok) throw new Error('Falha ao carregar entidades');
  return await res.json();
}

export async function startLivingScene(
  campaignId: string,
  payload?: { title?: string; location?: string; objectives?: string[] }
): Promise<LivingSceneData> {
  const res = await fetch(`${API_BASE_URL}/campaign/${campaignId}/scene/start`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload || {}),
  });

  if (!res.ok) throw new Error('Falha ao iniciar Cena Viva');
  const data = await res.json();
  return data.scene;
}

export async function claimSceneItem(
  campaignId: string,
  itemId: string,
  playerName: string
): Promise<{
  status: string;
  message: string;
  item: any;
  active_scene: LivingSceneData;
}> {
  const res = await fetch(`${API_BASE_URL}/campaign/${campaignId}/scene/claim_item`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ item_id: itemId, player_name: playerName }),
  });

  if (!res.ok) {
    const err = await res.json().catch(() => ({ message: 'Erro ao reivindicar item' }));
    throw new Error(err.message || 'Falha ao reivindicar item');
  }

  return await res.json();
}

export async function submitCampaignTurn(
  campaignId: string,
  payload: {
    via_type: string;
    action_id?: string;
    player_text?: string;
    character_name: string;
    class_name: string;
    actor_id: string;
    target_id?: string;
    target_name?: string;
  }
): Promise<{
  status: string;
  narrative_reaction: string;
  rules_result?: Record<string, unknown>;
  scene_closed?: Record<string, unknown>;
  active_scene?: LivingSceneData;
  player_entities?: Array<Record<string, unknown>>;
}> {
  const res = await fetch(`${API_BASE_URL}/campaign/${campaignId}/action/turn`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });

  if (!res.ok) throw new Error('Falha ao processar turno');
  return await res.json();
}

export async function moveSceneParticipant(
  campaignId: string,
  payload: {
    actor_id: string;
    x: number;
    y: number;
  }
): Promise<{
  status?: string;
  message?: string;
  movement_used_ft?: number;
  active_scene?: LivingSceneData;
  scene?: LivingSceneData;
  player_entities?: Array<Record<string, unknown>>;
}> {
  let res = await fetch(`${API_BASE_URL}/campaigns/${campaignId}/scene/move`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });
  if (res.status === 404) {
    res = await fetch(`${API_BASE_URL}/campaign/${campaignId}/scene/move`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
  }
  if (!res.ok) {
    const err = await res.json().catch(() => ({ message: 'Movimento inválido ou fora do alcance' }));
    throw new Error(err.message || 'Falha ao mover participante no mapa tático');
  }
  return await res.json();
}

export async function endSceneTurn(
  campaignId: string,
  payload: {
    actor_id: string;
  }
): Promise<{
  status?: string;
  narrative_reaction?: string;
  combat_log?: string[];
  active_scene?: LivingSceneData;
  scene?: LivingSceneData;
  player_entities?: Array<Record<string, unknown>>;
}> {
  let res = await fetch(`${API_BASE_URL}/campaigns/${campaignId}/scene/end-turn`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });
  if (res.status === 404) {
    res = await fetch(`${API_BASE_URL}/campaign/${campaignId}/scene/end-turn`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
  }
  if (!res.ok) {
    const err = await res.json().catch(() => ({ message: 'Erro ao encerrar turno' }));
    throw new Error(err.message || 'Falha ao encerrar turno');
  }
  return await res.json();
}

export function connectCampaignWebSocket(
  roomCode: string,
  onMessage: (data: Record<string, unknown>) => void
): WebSocket {
  const ws = new WebSocket(`${WS_BASE_URL}/${roomCode}`);
  ws.onmessage = (event) => {
    try {
      const parsed = JSON.parse(event.data);
      onMessage(parsed);
    } catch (e) {
      console.warn('Erro ao parsear mensagem WS:', e);
    }
  };
  return ws;
}

export async function syncCharacterWithBackend(
  sheet: CharacterSheet,
  campaignId?: string,
  playerId?: string
): Promise<{ status: string; message: string }> {
  try {
    const res = await fetch(`${API_BASE_URL}/character`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        ...sheet,
        campaign_id: campaignId,
        player_id: playerId,
      }),
    });
    if (!res.ok) throw new Error('Falha HTTP');
    return await res.json();
  } catch {
    return {
      status: 'local_mode',
      message: 'Ficha carregada em modo cliente local.',
    };
  }
}

export async function syncPartyWithCampaign(
  campaignId: string,
  party: CharacterSheet[]
): Promise<{
  status: string;
  campaign_id: string;
  player_slots: Array<{
    player_number: number;
    player_id: string;
    player_name: string;
    class_name: string;
    player_code: string;
    is_host: boolean;
  }>;
} | null> {
  try {
    const res = await fetch(`${API_BASE_URL}/campaign/${campaignId}/sync-party`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ party }),
    });
    if (!res.ok) return null;
    return await res.json();
  } catch {
    return null;
  }
}

export async function submitTurnActionToBackend(
  viaType: string,
  character: CharacterSheet,
  text?: string,
  actionId?: string
): Promise<FreeformTextActionSubmission> {
  try {
    const res = await fetch(`${API_BASE_URL}/action/turn`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        via_type: viaType,
        action_id: actionId,
        player_text: text,
        character_name: character.name,
        class_name: character.className,
      }),
    });

    if (res.ok) {
      const data = await res.json();
      const rollVal = data.roll_result as { total?: number; is_success?: boolean } | undefined;
      return {
        playerText: text || actionId || 'Ação de Turno',
        submittedAt: new Date().toLocaleTimeString(),
        status: 'resolved',
        requiresRulesMechanic: Boolean(data.requires_mechanic),
        detectedIntent: data.detected_intent || 'Ação Processada',
        resolvedCheck: rollVal
          ? {
              abilityOrSkill: 'D20 + Modificador (Motor Rust)',
              rollFormula: 'd20 + bônus determinístico',
              targetDc: 13,
              result: rollVal.total ?? 15,
              success: rollVal.is_success ?? true,
            }
          : undefined,
        narrativeFeedback: data.narrative_feedback || '',
      };
    }
  } catch {
    // Graceful fallback to client-side
  }

  const isMechanic =
    Boolean(text) &&
    ['atacar', 'chutar', 'empurrar', 'pular', 'saltar', 'esconder', 'arrombar', 'magia', 'furtivo'].some(
      (w) => text?.toLowerCase().includes(w)
    );

  return {
    playerText: text || actionId || 'Ação de Turno',
    submittedAt: new Date().toLocaleTimeString(),
    status: 'resolved',
    requiresRulesMechanic: isMechanic,
    detectedIntent: isMechanic
      ? 'Manobra Tática Improvisada (#regras)'
      : 'Interação Social / Diálogo Narrativo',
    narrativeFeedback: isMechanic
      ? `Ação analisada localmente: "${text}".`
      : `Interação narrativa: "${text}".`,
  };
}

// =========================================================================
// T11: Biblioteca de Premissas
// =========================================================================

export async function savePremiseToLibrary(premiseObj: {
  title: string;
  premise: string;
  literary_style: string;
  ai_creative_freedom: string;
  custom_monsters?: any[];
}): Promise<{ status: string; id: string; message: string }> {
  const res = await fetch(`${API_BASE_URL}/premises`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(premiseObj),
  });
  if (!res.ok) throw new Error('Falha ao salvar premissa');
  return await res.json();
}

// =========================================================================
// T12: Salvar / Retomar / Exportar / Importar Campanhas
// =========================================================================

export async function fetchSavedCampaigns(): Promise<any[]> {
  try {
    const res = await fetch(`${API_BASE_URL}/campaigns/saved`);
    if (!res.ok) return [];
    const data = await res.json();
    return data.campaigns || [];
  } catch {
    return [];
  }
}

export async function exportCampaignBackup(campaignId: string): Promise<any> {
  const res = await fetch(`${API_BASE_URL}/campaign/${campaignId}/export`);
  if (!res.ok) throw new Error('Falha ao exportar backup da campanha');
  return await res.json();
}

export async function importCampaignBackup(bundle: any): Promise<{
  status: string;
  campaign_id: string;
  room_code: string;
  message: string;
}> {
  const res = await fetch(`${API_BASE_URL}/campaign/import`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(bundle),
  });
  if (!res.ok) {
    const err = await res.json().catch(() => ({ message: 'Erro ao importar campanha' }));
    throw new Error(err.message || 'Falha ao importar campanha');
  }
  return await res.json();
}

// =========================================================================
// T14: Chat da Party (HTTP fallback)
// =========================================================================

export async function sendPartyChatMessage(
  campaignId: string,
  payload: { sender_id: string; sender_name: string; text: string; is_action?: boolean }
): Promise<void> {
  await fetch(`${API_BASE_URL}/campaign/${campaignId}/chat`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });
}

// =========================================================================
// T15: Auditoria Completa
// =========================================================================

export async function fetchCampaignAudit(campaignId: string): Promise<any> {
  const res = await fetch(`${API_BASE_URL}/campaign/${campaignId}/audit`);
  if (!res.ok) throw new Error('Falha ao buscar auditoria da campanha');
  return await res.json();
}
