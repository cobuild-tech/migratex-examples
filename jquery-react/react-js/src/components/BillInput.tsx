import { money } from '../lib/sanitize';

interface Props {
  value: string;
  onChange: (value: string) => void;
}

export function BillInput({ value, onChange }: Props) {
  return (
    <div className="bill-main">
      <label htmlFor="bill" className="bill">Bill</label><br />
      <input
        id="bill"
        type="text"
        inputMode="decimal"
        className="bill-input"
        placeholder="0"
        value={value}
        onChange={(e) => {
          const next = money(e.target.value);
          if (next !== null) onChange(next);
        }}
      /><br />
    </div>
  );
}
