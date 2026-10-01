import { Navigate, useNavigate, useParams } from 'react-router-dom';
import { ActivityPanel } from '../components/ActivityPanel';
import type { PageState } from '../components/Page';
import { PhonePage } from '../components/PhonePage';
import { RepoCategories } from '../components/RepoCategories';
import { RepoTable } from '../components/RepoTable';
import { useIsPhone } from '../hooks/useIsPhone';
import { isRepoFilter } from '../lib/filterRepos';
import { useAppState } from '../state/AppState';

const BACK: PageState = { reverse: true };
const NO_TRANSITION: PageState = { transition: 'none' };

/**
 * Phone pages exist only on phones and only after their button was pressed. Otherwise
 * (a deep link, a refresh, or the window widening to desktop) go home, where desktop
 * shows the same content as panels.
 */
function useHomeRedirect(ready: boolean) {
  const isPhone = useIsPhone();
  return !isPhone || !ready ? <Navigate to="/" replace state={NO_TRANSITION} /> : null;
}

export function ActivityPage() {
  const { activity } = useAppState();
  const navigate = useNavigate();
  const redirect = useHomeRedirect(!!activity);
  if (redirect || !activity) return redirect;
  return (
    <PhonePage id="activity-page" title="Git Activity" onBack={() => navigate('/', { state: BACK })}>
      <ActivityPanel key={activity.id} request={activity} />
    </PhonePage>
  );
}

export function RepoCategoryPage() {
  const { repos, setRepoFilter } = useAppState();
  const navigate = useNavigate();
  const redirect = useHomeRedirect(!!repos);
  if (redirect || !repos) return redirect;
  return (
    <PhonePage id="repo-category-page" title="Git Repo Categories" onBack={() => navigate('/', { state: BACK })}>
      <RepoCategories
        key={repos.id}
        request={repos}
        onFilter={(filter) => {
          setRepoFilter(filter);
          navigate(`/repos/${filter}`);
        }}
      />
    </PhonePage>
  );
}

export function RepoPage() {
  const { repos } = useAppState();
  const { filter } = useParams();
  const navigate = useNavigate();
  const redirect = useHomeRedirect(!!repos);
  if (redirect || !repos) return redirect;
  if (!isRepoFilter(filter)) return <Navigate to="/repos" replace state={NO_TRANSITION} />;
  return (
    <PhonePage id="repo-page" title="Git Repos" onBack={() => navigate('/repos', { state: BACK })}>
      <RepoTable key={repos.id} request={repos} filter={filter} />
    </PhonePage>
  );
}
