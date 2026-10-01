import { useRepos } from '../hooks/queries';
import type { RepoFilter } from '../lib/filterRepos';
import type { Request } from '../state/AppState';

export function RepoCategories({ request, onFilter }: { request: Request; onFilter: (filter: RepoFilter) => void }) {
  // Starts the fetch as soon as the categories show, like RepoCategoryView did.
  useRepos(request);
  return (
    <fieldset className="ui-grid-b">
      <div className="ui-block-a">
        <button type="button" className="ui-btn ui-btn-b" id="all-repos" onClick={() => onFilter('all')}>
          All
        </button>
      </div>
      <div className="ui-block-b">
        <button type="button" className="ui-btn ui-btn-b" id="source-repos" onClick={() => onFilter('source')}>
          Source's
        </button>
      </div>
      <div className="ui-block-c">
        <button type="button" className="ui-btn ui-btn-b" id="fork-repos" onClick={() => onFilter('fork')}>
          Fork's
        </button>
      </div>
    </fieldset>
  );
}
