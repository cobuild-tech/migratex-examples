export const PRESETS = [5, 10, 15, 25, 50] as const;

export type Tip = { kind: 'none' } | { kind: 'preset'; pct: number } | { kind: 'custom' };

export interface State {
  bill: string;
  tip: Tip;
  customTip: string;
  people: string;
}

export type Action =
  | { type: 'setBill'; value: string }
  | { type: 'selectPreset'; pct: number }
  | { type: 'setCustomTip'; value: string }
  | { type: 'setPeople'; value: string }
  | { type: 'reset' };

export const initialState: State = { bill: '', tip: { kind: 'none' }, customTip: '', people: '' };

export function reducer(state: State, action: Action): State {
  switch (action.type) {
    case 'setBill':
      return { ...state, bill: action.value };
    case 'selectPreset':
      return { ...state, tip: { kind: 'preset', pct: action.pct }, customTip: '' };
    case 'setCustomTip':
      return { ...state, tip: { kind: 'custom' }, customTip: action.value };
    case 'setPeople':
      return { ...state, people: action.value };
    case 'reset':
      return initialState;
  }
}

export function tipPercent(state: State): number {
  if (state.tip.kind === 'preset') return state.tip.pct;
  if (state.tip.kind === 'custom') return parseFloat(state.customTip) || 0;
  return 0;
}

/** "0", "00" and so on: the people field holds a number, and it is zero. */
export const peopleIsZero = (state: State) => state.people !== '' && Number(state.people) === 0;

export const isDirty = (state: State) =>
  state.bill !== '' || state.customTip !== '' || state.people !== '' || state.tip.kind !== 'none';
