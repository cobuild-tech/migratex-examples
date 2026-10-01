import { useRepos } from '../hooks/queries';
import { filterRepos, type RepoFilter } from '../lib/filterRepos';
import type { Request } from '../state/AppState';

export function RepoTable({ request, filter }: { request: Request; filter: RepoFilter }) {
  const { data = [] } = useRepos(request);
  return (
    <table className="ui-table table-stroke" id="table-column-toggle">
      <thead>
        <tr>
          <th>Repositories</th>
          <th>Stars</th>
          <th>Forks</th>
        </tr>
      </thead>
      <tbody>
        {filterRepos(data, filter).map((repo) => (
          <tr key={repo.id}>
            <th>{repo.name}</th>
            <th>{repo.watchers}</th>
            <th>{repo.forks}</th>
          </tr>
        ))}
      </tbody>
    </table>
  );
}
