export interface Split {
  tipPerPerson: number;
  totalPerPerson: number;
}

const round2 = (n: number) => Math.round((n + Number.EPSILON) * 100) / 100;

/**
 * Splits a bill plus tip evenly. Returns null until there is something valid to show:
 * a bill, and a whole number of people of at least one. Rounds once, at the end.
 */
export function calculateSplit(bill: number, tipPct: number, people: number): Split | null {
  if (!Number.isFinite(bill) || bill < 0) return null;
  if (!Number.isInteger(people) || people < 1) return null;
  const pct = Number.isFinite(tipPct) && tipPct > 0 ? tipPct : 0;
  return {
    tipPerPerson: round2((bill * pct) / 100 / people),
    totalPerPerson: round2((bill * (1 + pct / 100)) / people),
  };
}

export const formatMoney = (n: number) => n.toFixed(2);
