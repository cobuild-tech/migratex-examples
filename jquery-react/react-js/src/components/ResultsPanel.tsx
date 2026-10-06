import { formatMoney, type Split } from '../lib/split';

interface Props {
  split: Split | null;
  canReset: boolean;
  onReset: () => void;
}

export function ResultsPanel({ split, canReset, onReset }: Props) {
  return (
    <div className="result-main">
      <div className="results-div">
        <div className="results">
          <div className="labels">
            <h2>Tip Amount</h2>
            <h3>/ person</h3>
          </div>
          <div className="result-display">
            <h1>$<span id="tip-display">{formatMoney(split?.tipPerPerson ?? 0)}</span></h1>
          </div>
        </div>

        <div className="results">
          <div className="labels">
            <h2>Total</h2>
            <h3>/ person</h3>
          </div>
          <div className="result-display">
            <h1>$<span id="total-display">{formatMoney(split?.totalPerPerson ?? 0)}</span></h1>
          </div>
        </div>
      </div>

      <div className="result-btn-div">
        <button type="button" className="reset-btn" disabled={!canReset} onClick={onReset}>
          RESET
        </button>
      </div>
    </div>
  );
}
