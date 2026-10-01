export function Header({ title, onBack }: { title: string; onBack?: () => void }) {
  return (
    <div className="ui-header ui-bar-a">
      {onBack && (
        <button type="button" className="ui-btn ui-btn-a ui-btn-left" id="back-btn" onClick={onBack}>
          <span className="ui-icon ui-icon-arrow-l" aria-hidden="true" />
          Back
        </button>
      )}
      <h1 className="ui-title">{title}</h1>
    </div>
  );
}
