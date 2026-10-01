import type { FormEvent } from 'react';
import { useNavigate } from 'react-router-dom';
import { gravatarUrl } from '../api/github';
import { ActivityPanel } from '../components/ActivityPanel';
import { Footer } from '../components/Footer';
import { Header } from '../components/Header';
import { Page } from '../components/Page';
import { RepoCategories } from '../components/RepoCategories';
import { RepoTable } from '../components/RepoTable';
import { useFindUser } from '../hooks/queries';
import { useIsPhone } from '../hooks/useIsPhone';
import { useAppState } from '../state/AppState';

/** Matches jQuery's `slideUp('slow')`, after which the original alerted. */
const SLIDE_MS = 600;

export function HomePage() {
  const isPhone = useIsPhone();
  const navigate = useNavigate();
  const state = useAppState();
  const findUser = useFindUser();

  function onFindUser(event: FormEvent) {
    event.preventDefault();
    // A failed request does nothing, as Backbone's `sync` listener never fired on errors.
    findUser.mutate(state.username, {
      onSuccess: (user) => {
        if (user.username) {
          state.showAvatar(gravatarUrl(user.gravatar_id ?? ''));
        } else {
          const wasOpen = state.avatarOpen;
          state.hideAvatar();
          setTimeout(() => window.alert('No User Found'), wasOpen ? SLIDE_MS : 0);
        }
      },
    });
  }

  function onGetActivity() {
    state.requestActivity();
    if (isPhone) navigate('/activity');
  }

  function onGetRepos() {
    state.requestRepos();
    if (isPhone) navigate('/repos');
  }

  return (
    <Page id="home-page">
      <Header title="Git Info" />
      <div className="ui-content">
        <div className="home-container">
          <div className="ui-grid-a home-top-container">
            <div className="ui-block-a inner-container left">
              <form className="form-container" onSubmit={onFindUser}>
                <input
                  type="text"
                  name="gh-name"
                  id="gh-username"
                  className="ui-input-text"
                  placeholder="Github Username"
                  value={state.username}
                  onChange={(e) => state.setUsername(e.target.value)}
                />
                <button type="submit" className="ui-btn ui-btn-c ui-btn-icon-notext" id="find-user" title="Find User">
                  <span className="ui-icon ui-icon-arrow-r" aria-hidden="true" />
                  <span className="ui-hidden-accessible">Find User</span>
                </button>
              </form>
              <div className={`avatar-container${state.avatarOpen ? ' is-open' : ''}`} aria-hidden={!state.avatarOpen}>
                <div className="avatar-inner">
                  <img src={state.avatar} className="avatar" id="gh-avatar" alt="" />
                  <fieldset className="ui-grid-a">
                    <div className="ui-block-a">
                      <button type="button" className="ui-btn ui-btn-c" id="get-user-activity" onClick={onGetActivity}>
                        Get User Activity
                      </button>
                    </div>
                    <div className="ui-block-b">
                      <button type="button" className="ui-btn ui-btn-b" id="get-user-repositores" onClick={onGetRepos}>
                        Get User Repositories
                      </button>
                    </div>
                  </fieldset>
                </div>
              </div>
            </div>
            <div className="ui-block-b inner-container right" id="events-container">
              {!isPhone && state.activity && <ActivityPanel key={state.activity.id} request={state.activity} />}
            </div>
          </div>
          <div id="repos-category-container">
            {!isPhone && state.repos && (
              <RepoCategories key={state.repos.id} request={state.repos} onFilter={state.setRepoFilter} />
            )}
          </div>
          <div id="repos-container">
            {!isPhone && state.repos && state.repoFilter && (
              <RepoTable key={state.repos.id} request={state.repos} filter={state.repoFilter} />
            )}
          </div>
        </div>
      </div>
      <Footer />
    </Page>
  );
}
