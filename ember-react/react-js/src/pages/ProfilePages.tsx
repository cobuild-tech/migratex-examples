import { NavLink, Outlet, useParams } from 'react-router-dom';
import { ArticlePreview } from '../components/ArticlePreview';
import { FollowProfile } from '../components/FollowProfile';
import { useArticles, useProfile } from '../hooks/queries';
import { useSession } from '../session/SessionContext';
import { NotFoundPage } from './NotFoundPage';
import { profileFavoritesPath, profilePath } from '../lib/paths';

export function ProfileLayout() {
  const { id = '' } = useParams();
  const { user } = useSession();
  const { data: profile, isError } = useProfile(id);

  if (isError) return <NotFoundPage />;
  if (!profile) return null;

  return (
    <div className="profile-page">
      <div className="user-info">
        <div className="container">
          <div className="row">
            <div className="col-xs-12 col-md-10 offset-md-1">
              <img src={profile.image ?? undefined} alt={profile.username} className="user-img" />
              <h4>{profile.username}</h4>
              <p>{profile.bio}</p>
              {user?.username === profile.username ? (
                <NavLink to="/settings" className="btn btn-sm btn-outline-secondary action-btn" data-test-edit-profile-button>
                  <i className="ion-gear-a"></i> Edit Profile Settings
                </NavLink>
              ) : (
                <FollowProfile profile={profile} />
              )}
            </div>
          </div>
        </div>
      </div>
      <div className="container">
        <div className="row">
          <div className="col-xs-12 col-md-10 offset-md-1">
            <div className="articles-toggle">
              <ul className="nav nav-pills outline-active">
                <li className="nav-item">
                  <NavLink to={profilePath(profile.username)} end className="nav-link" data-test-profile-tab="my-articles">
                    My Articles
                  </NavLink>
                </li>
                <li className="nav-item">
                  <NavLink
                    to={profileFavoritesPath(profile.username)}
                    className="nav-link"
                    data-test-profile-tab="favorite-articles"
                  >
                    Favorited Articles
                  </NavLink>
                </li>
              </ul>
            </div>
            <Outlet />
          </div>
        </div>
      </div>
    </div>
  );
}

function ProfileArticles({ params }: { params: { author?: string; favorited?: string } }) {
  const { data } = useArticles(params);
  return <>{data?.articles.map((article) => <ArticlePreview key={article.slug} article={article} />)}</>;
}

export function ProfileArticlesPage() {
  const { id = '' } = useParams();
  return <ProfileArticles params={{ author: id }} />;
}

export function ProfileFavoritesPage() {
  const { id = '' } = useParams();
  return <ProfileArticles params={{ favorited: id }} />;
}
