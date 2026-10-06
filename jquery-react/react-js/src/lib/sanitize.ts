// Each filter returns the cleaned value, or null to reject the keystroke and keep the old value.

/** Digits with at most one "." and two decimals, e.g. "142.55". */
export const money = (value: string) => (/^\d*(\.\d{0,2})?$/.test(value) ? value : null);

/** Digits with at most one ".", e.g. "12.5". */
export const percent = (value: string) => (/^\d*(\.\d*)?$/.test(value) ? value : null);

/** Digits only. */
export const wholeNumber = (value: string) => (/^\d*$/.test(value) ? value : null);
