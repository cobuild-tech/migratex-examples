import { percent } from '../lib/sanitize';
import { PRESETS, type Tip } from '../state';

interface Props {
  tip: Tip;
  customTip: string;
  onSelectPreset: (pct: number) => void;
  onCustomTipChange: (value: string) => void;
}

export function TipSelector({ tip, customTip, onSelectPreset, onCustomTipChange }: Props) {
  return (
    <div className="tip-main">
      <label htmlFor="custom-tip" className="bill">Select Tip %</label><br />
      <div className="tips">
        {PRESETS.map((pct) => {
          const selected = tip.kind === 'preset' && tip.pct === pct;
          return (
            <button
              key={pct}
              type="button"
              className={selected ? 'share tip-selected' : 'share'}
              value={pct}
              aria-pressed={selected}
              onClick={() => onSelectPreset(pct)}
            >
              <span>{pct}%</span>
            </button>
          );
        })}
        <input
          id="custom-tip"
          type="text"
          inputMode="decimal"
          className={tip.kind === 'custom' ? 'shareinput tip-selected' : 'shareinput'}
          placeholder="Custom"
          value={customTip}
          onChange={(e) => {
            const next = percent(e.target.value);
            if (next !== null) onCustomTipChange(next);
          }}
        />
      </div>
    </div>
  );
}
