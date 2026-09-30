import { useNavigate } from 'react-router-dom';
import type { Profile } from '../api/types';
import { useFollow } from '../hooks/queries';
import { useSession } from '../session/SessionContext';

export function FollowProfile({ profile }: { profile: Profile }) {
  const { isLoggedIn } = useSession();
  const navigate = useNavigate();
  const follow = useFollow();

  const onClick = () => {
    if (isLoggedIn) {
      follow.mutate(profile);
    } else {
      navigate('/login');
    }
  };

  return (
    <button
      className={`btn btn-sm action-btn btn${profile.following ? '' : '-outline'}-secondary`}
      onClick={onClick}
      disabled={follow.isPending}
      data-test-follow-author-button
      type="button"
    >
      <i className={`ion-${profile.following ? 'minus' : 'plus'}-round`}></i>
      &nbsp;
      {profile.following ? 'Unf' : 'F'}ollow {profile.username}
    </button>
  );
}
