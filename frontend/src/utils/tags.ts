/**
 * Utilitário de normalização e manipulação de tags do RPG Engine.
 *
 * Requisito: As tags precisam ser estritamente de UMA ÚNICA PALAVRA cada (sem espaços,
 * minúsculas, sem pontuações supérfluas).
 */

export const normalizeTags = (tags?: string[] | null): string[] => {
  if (!tags || !Array.isArray(tags)) return [];
  const normalized: string[] = [];

  for (const rawTag of tags) {
    if (typeof rawTag !== 'string') continue;
    const lower = rawTag.toLowerCase().trim();
    if (!lower) continue;

    // Mapeamentos diretos de tags legadas multi-palavras para tokens únicos
    let mapped = lower;
    if (lower === 'anos 80/90' || lower === 'anos 80' || lower === 'anos 90') {
      mapped = 'anos80';
    } else if (lower === 'ficção científica' || lower === 'ficcao cientifica' || lower === 'sci-fi') {
      mapped = 'scifi';
    } else if (lower === 'he-man & she-ra' || lower === 'he-man' || lower === 'she-ra') {
      mapped = 'heman';
    } else if (lower === 'dungeon crawl') {
      mapped = 'dungeon';
    } else if (lower === 'conto de fadas sombrio') {
      mapped = 'sombria';
    } else if (lower === 'infiltração & espionagem' || lower === 'infiltracao & espionagem') {
      mapped = 'espionagem';
    }

    // Divide qualquer tag multi-palavras por espaços, barras ou pontuações
    const words = mapped
      .split(/[\s,/_&\\-]+/)
      .map((w) => w.replace(/^[^a-z0-9áàâãéèêíïóôõöúçñ]+|[^a-z0-9áàâãéèêíïóôõöúçñ]+$/gi, '').trim())
      .filter(Boolean);

    for (const word of words) {
      if (word && !normalized.includes(word)) {
        normalized.push(word);
      }
    }
  }

  return normalized;
};
