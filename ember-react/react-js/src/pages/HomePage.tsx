import { Link, useSearchParams } from 'react-router-dom';
import { ArticleList } from '../components/ArticleList';
import { TagList } from '../components/TagList';
import { homeUrl } from '../lib/homeUrl';
import { useSession } from '../session/SessionContext';

export function HomePage() {
  const { isLoggedIn } = useSession();
  const [params] = useSearchParams();
  const feed = params.get('feed');
  const tag = params.get('tag');
  const page = Math.max(1, parseInt(params.get('page') ?? '1', 10) || 1);
  const isGlobalFeed = !tag && !feed;

  return (
    <div className="home-page">
      <div className="banner">
        <div className="container">
          <h1 className="logo-font">inkwell</h1>
          <p>A place to share your knowledge.</p>
        </div>
      </div>
      <div className="container page">
        <div className="row">
          <div className="col-md-9">
            <div className="feed-toggle">
              <ul className="nav nav-pills outline-active">
                {isLoggedIn && (
                  <li className="nav-item">
                    <Link
                      to={homeUrl({ feed: 'your' })}
                      className={`nav-link ${feed === 'your' ? 'active' : ''}`}
                      data-test-tab="your"
                    >
                      Your Feed
                    </Link>
                  </li>
                )}
                <li className="nav-item">
                  <Link to="/" className={`nav-link ${isGlobalFeed ? 'active' : ''}`} data-test-tab="global">
                    Global Feed
                  </Link>
                </li>
                {tag && (
                  <li className="nav-item">
                    <Link to={homeUrl({ tag })} className="nav-link active" data-test-tab="tag">
                      #{tag}
                    </Link>
                  </li>
                )}
              </ul>
            </div>
            <ArticleList feed={feed} tag={tag} page={page} />
          </div>
          <div className="col-md-3">
            <div className="sidebar">
              <TagList />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
