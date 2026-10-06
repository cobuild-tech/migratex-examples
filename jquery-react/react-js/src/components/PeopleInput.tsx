import { wholeNumber } from '../lib/sanitize';

interface Props {
  value: string;
  invalid: boolean;
  onChange: (value: string) => void;
}

export function PeopleInput({ value, invalid, onChange }: Props) {
  return (
    <div className="people-main">
      <label className="people-label bill" htmlFor="people">
        Number of People{' '}
        {/* Inline display, as jQuery's .show() set it, so it also beats the mobile `.bill span` rule. */}
        <span className="invalid" style={invalid ? { display: 'inline' } : undefined}>
          Can't be zero
        </span>
      </label>
      <input
        id="people"
        type="text"
        inputMode="numeric"
        className={invalid ? 'people-input invalid-people' : 'people-input'}
        placeholder="0"
        aria-invalid={invalid}
        value={value}
        onChange={(e) => {
          const next = wholeNumber(e.target.value);
          if (next !== null) onChange(next);
        }}
      />
    </div>
  );
}
