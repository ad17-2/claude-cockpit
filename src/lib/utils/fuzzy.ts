export interface FuzzyResult<T> {
  item: T;
  score: number;
}

export function fuzzyMatch(query: string, target: string): number {
  const q = query.toLowerCase();
  const t = target.toLowerCase();

  if (t === q) return 100;
  if (t.startsWith(q)) return 90;
  if (t.includes(q)) return 80;

  let qi = 0;
  let score = 0;
  let lastMatchIndex = -1;

  for (let ti = 0; ti < t.length && qi < q.length; ti++) {
    if (t[ti] === q[qi]) {
      score += 10;
      if (lastMatchIndex === ti - 1) score += 5;
      lastMatchIndex = ti;
      qi++;
    }
  }

  if (qi < q.length) return 0;
  return score;
}

export function fuzzySearch<T>(
  query: string,
  items: T[],
  getText: (item: T) => string,
  minScore: number = 1,
): FuzzyResult<T>[] {
  if (!query.trim()) return items.map((item) => ({ item, score: 0 }));

  return items
    .map((item) => ({ item, score: fuzzyMatch(query, getText(item)) }))
    .filter((r) => r.score >= minScore)
    .sort((a, b) => b.score - a.score);
}
