import { act } from '@testing-library/react';
import { http, HttpResponse } from 'msw';
import { createArticle, createProfile, createUser, findArticle, findProfile } from '../mocks/db';
import { API } from '../mocks/handlers';
import { server } from '../mocks/server';
import { logInAs, visit, waitFor } from '../helpers/app';
import { STORAGE_KEY } from '../../src/session/SessionContext';

describe('Acceptance | code review fixes', () => {
  it('login refreshes viewer-specific data fetched while logged out', async () => {
    const user = createUser({ email: 'bob@example.com', password: 'pw' });
    const article = createArticle({ favorited: false });
    const app = await visit('/');
    await app.find(`[data-test-article-preview="${article.slug}"] [data-test-favorite-article-button="unfavorited"]`);

    // The server now reports this article as favorited for the logged-in viewer.
    findArticle(article.slug)!.favorited = true;
    await act(() => app.router.navigate('/login'));
    await app.user.type(await app.find('[data-test-login-email]'), user.email);
    await app.user.type(app.$('[data-test-login-password]')!, 'pw');
    await app.user.click(app.$('[data-test-login-button]')!);

    await app.find(`[data-test-article-preview="${article.slug}"] [data-test-favorite-article-button="favorited"]`);
  });

  it('logout drops cached personal data', async () => {
    createUser();
    logInAs();
    const author = createProfile({ following: true });
    const article = createArticle({ author });
    const app = await visit(`/articles/${article.slug}`);
    await waitFor(() => expect(app.$('[data-test-follow-author-button]')).toHaveTextContent('Unfollow'));

    findProfile(author.username)!.following = false;
    await app.user.click(app.$('[data-test-nav-log-out]')!);
    await app.find('[data-test-nav-sign-in]');
    await act(() => app.router.navigate(`/articles/${article.slug}`));
    await waitFor(() => expect(app.$('[data-test-follow-author-button]')).not.toHaveTextContent('Unfollow'));
  });

  it('a transient /user failure keeps the stored token', async () => {
    createUser();
    logInAs();
    server.use(http.get(`${API}/user`, () => HttpResponse.json({}, { status: 503 })));
    const app = await visit('/');
    expect(app.$('[data-test-nav-sign-in]')).toBeInTheDocument();
    expect(localStorage.getItem(STORAGE_KEY)).toBe('auth-token');
  });

  it('an invalid token (401) is cleared', async () => {
    logInAs('stale-token');
    await visit('/');
    expect(localStorage.getItem(STORAGE_KEY)).toBeNull();
  });

  it('cancelling the unsaved-changes prompt during logout stays logged in', async () => {
    const user = createUser();
    logInAs();
    const article = createArticle({ author: findProfile(user.username) });
    vi.spyOn(window, 'confirm').mockReturnValue(false);

    const app = await visit(`/editor/${article.slug}`);
    await app.user.type(await app.find('[data-test-article-form-input-title]'), '!');
    await app.user.click(app.$('[data-test-nav-log-out]')!);

    await waitFor(() => expect(window.confirm).toHaveBeenCalledTimes(1));
    expect(app.currentURL()).toBe(`/editor/${article.slug}`);
    expect(app.$('[data-test-nav-username]')).toBeInTheDocument();
    expect(app.$('[data-test-article-form-input-title]')).toHaveValue(`${article.title}!`);
  });

  it('editing a missing article shows the not-found page', async () => {
    createUser();
    logInAs();
    const app = await visit('/editor/does-not-exist');
    await app.find('[data-test-error-page]');
  });

  it('a non-JSON error response surfaces as a normal error', async () => {
    server.use(
      http.post(`${API}/users/login`, () => new HttpResponse('<html>Bad Gateway</html>', { status: 502 })),
    );
    const app = await visit('/login');
    await app.user.type(app.$('[data-test-login-email]')!, 'a@b.c');
    await app.user.type(app.$('[data-test-login-password]')!, 'x');
    await app.user.click(app.$('[data-test-login-button]')!);
    const error = await app.find('.error-messages li');
    expect(error).toHaveTextContent('Request failed with status 502');
    expect(error).not.toHaveTextContent('SyntaxError');
  });

  it('a failed delete shows an error and stays on the article', async () => {
    const user = createUser();
    logInAs();
    const article = createArticle({ author: findProfile(user.username) });
    server.use(http.delete(`${API}/articles/:slug`, () => HttpResponse.json({}, { status: 500 })));

    const app = await visit(`/articles/${article.slug}`);
    await app.user.click((await app.find('[data-test-delete-article-button]')));
    expect(await app.find('[data-test-delete-article-error]')).toHaveTextContent('Could not delete');
    expect(app.currentURL()).toBe(`/articles/${article.slug}`);
  });
});
