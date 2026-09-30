export function ErrorMessages({ errors, itemAttr }: { errors: string[]; itemAttr?: string }) {
  if (!errors.length) return null;
  return (
    <ul className="error-messages">
      {errors.map((error, index) => (
        <li key={index} {...(itemAttr ? { [itemAttr]: index } : {})}>
          {error}
        </li>
      ))}
    </ul>
  );
}
