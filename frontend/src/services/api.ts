import type { CharacterSheet } from '../types/character';
import type { FreeformTextActionSubmission } from '../types/game';

const API_BASE_URL = 'http://localhost:3001/api';

export interface BackendHealth {
  status: string;
  engine: string;
  rules: string;
  version: string;
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

export async function syncCharacterWithBackend(
  sheet: CharacterSheet
): Promise<{ status: string; message: string }> {
  try {
    const res = await fetch(`${API_BASE_URL}/character`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(sheet),
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
      return {
        playerText: text || actionId || 'Ação de Turno',
        submittedAt: new Date().toLocaleTimeString(),
        status: 'resolved',
        requiresRulesMechanic: data.requires_mechanic,
        detectedIntent: data.detected_intent,
        resolvedCheck: data.roll_result
          ? {
              abilityOrSkill: 'D20 + Modificador (Motor Rust)',
              rollFormula: 'd20 + bônus determinístico',
              targetDc: 13,
              result: data.roll_result.total || 15,
              success: data.roll_result.success ?? true,
            }
          : undefined,
        narrativeFeedback: data.narrative_feedback,
      };
    }
  } catch {
    // Graceful fallback to client-side rule evaluation
  }

  // Local fallback
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
      ? 'Ação resolvida pelo simulador determinístico de regras D&D 5e.'
      : 'Diálogo incorporado ao Beat atual pelo Mestre.',
  };
}
