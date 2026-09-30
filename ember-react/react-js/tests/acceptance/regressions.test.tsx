import { http, HttpResponse } from 'msw';
import { act } from '@testing-library/react';
import { createArticle, createArticles, createUser, db, findProfile, serializeArticle } from '../mocks/db';
import { API } from '../mocks/handlers';
import { requests, server } from '../mocks/server';
import { logInAs, visit, waitFor } from '../helpers/app';

describe('Acceptance | regressions', () => {
  describe('logged-out user', () => {
    it('requests carry the token after logging in without a reload', async () => {
      const user = createUser({ email: 'bob@example.com', password: 'secret' });
      const article = createArticle();

      const app = await visit('/login');
      await app.user.type(app.$('[data-test-login-email]')!, user.email);
      await app.user.type(app.$('[data-test-login-password]')!, 'secret');
      await app.user.click(app.$('[data-test-login-button]')!);
      await app.find('[data-test-nav-username]');

      await act(() => app.router.navigate(`/articles/${article.slug}`));
      await app.find('h1[data-test-article-title]');

      const request = requests.filter((r) => r.url.endsWith(`/articles/${article.slug}`)).pop()!;
      expect(request.headers.get('Authorization')).toBe(`Token ${user.token}`);
    });

    it('?feed=your while logged out falls back to the global feed', async () => {
      createArticles(3);
      const app = await visit('/?feed=your');
      await waitFor(() => expect(app.$$('[data-test-article-preview]')).toHaveLength(3));
    });

    it('favorite button has no debug text', async () => {
      const article = createArticle();
      const app = await visit('/');
      const meta = await app.find(`[data-test-article-preview="${article.slug}"] .article-meta`);
      expect(meta).not.toHaveTextContent('article favorited');
    });

    it('article markdown is sanitized', async () => {
      const article = createArticle({
        body: '# Hi\n\n<img src=x onerror="window.__xss = true"><a href="javascript:alert(1)">x</a>',
      });
      const app = await visit(`/articles/${article.slug}`);

      expect(await app.find('[data-test-article-body] h1')).toHaveTextContent('Hi');
      expect(app.$('[data-test-article-body] img[onerror]')).toBeNull();
      expect(app.$('[data-test-article-body] a[href^="javascript"]')).toBeNull();
      expect((window as unknown as { __xss?: boolean }).__xss).toBeUndefined();
    });
  });

  describe('logged-in user', () => {
    let username: string;

    beforeEach(() => {
      username = createUser().username;
      logInAs();
    });

    it('paginating "Your Feed" stays on the feed', async () => {
      createArticles(25);
      server.use(
        http.get(`${API}/articles/feed`, ({ request }) => {
          const params = new URL(request.url).searchParams;
          const limit = parseInt(params.get('limit')!, 10);
          const offset = parseInt(params.get('offset')!, 10);
          return HttpResponse.json({
            articles: db.articles.slice(offset, offset + limit).map(serializeArticle),
            articlesCount: 15,
          });
        }),
      );

      const app = await visit('/?feed=your');
      await waitFor(() => expect(app.$$('[data-test-page-item]')).toHaveLength(2));
      await app.user.click(app.$('[data-test-page-item-link="2"]')!);

      expect(app.currentURL()).toBe('/?feed=your&page=2');
      await waitFor(() => expect(requests.some((r) => r.url.includes('offset=10'))).toBe(true));
      const feedRequest = requests.filter((r) => r.url.includes('/articles/feed')).pop()!;
      expect(new URL(feedRequest.url).searchParams.get('offset')).toBe('10');
      expect(app.$('[data-test-tab="your"]')).toHaveClass('active');
    });

    it('log out clears the user and returns home', async () => {
      const app = await visit('/settings');
      await app.user.click(app.$('[data-test-nav-log-out]')!);

      await app.find('[data-test-nav-sign-in]');
      expect(app.currentURL()).toBe('/');
      expect(app.$('[data-test-nav-username]')).toBeNull();
      expect(localStorage.getItem('ember-webapp.token')).toBeNull();
    });

    it("cannot open the editor for someone else's article", async () => {
      const article = createArticle();
      const app = await visit(`/editor/${article.slug}`);
      await waitFor(() => expect(app.currentURL()).toBe(`/articles/${article.slug}`));
      expect(article.author).not.toBe(username);
    });

    it('cancelling the unsaved-changes prompt keeps you in the editor', async () => {
      const article = createArticle({ author: findProfile(username) });
      const confirm = vi.spyOn(window, 'confirm').mockReturnValue(false);

      const app = await visit(`/editor/${article.slug}`);
      const title = await app.find('[data-test-article-form-input-title]');
      await app.user.clear(title);
      await app.user.type(title, 'Changed');
      await act(() => app.router.navigate('/'));

      expect(confirm).toHaveBeenCalledTimes(1);
      expect(app.currentURL()).toBe(`/editor/${article.slug}`);
      expect(app.$('[data-test-article-form-input-title]')).toHaveValue('Changed');
    });

    it('confirming the unsaved-changes prompt leaves the editor', async () => {
      const article = createArticle({ author: findProfile(username) });
      vi.spyOn(window, 'confirm').mockReturnValue(true);

      const app = await visit(`/editor/${article.slug}`);
      await app.user.type(await app.find('[data-test-article-form-input-title]'), '!');
      await act(() => app.router.navigate('/'));
      await waitFor(() => expect(app.currentURL()).toBe('/'));
    });

    it('tags ignore extra whitespace', async () => {
      const app = await visit('/editor');
      await app.user.type(app.$('[data-test-article-form-input-title]')!, 'T');
      await app.user.type(app.$('[data-test-article-form-input-description]')!, 'D');
      await app.user.type(app.$('[data-test-article-form-input-body]')!, 'B');
      await app.user.type(app.$('[data-test-article-form-input-tags]')!, '  one   two ');
      await app.user.click(app.$('[data-test-article-form-submit-button]')!);

      await waitFor(() => expect(app.currentURL()).toMatch(/^\/articles\//));
      const post = requests.find((r) => r.method === 'POST' && r.url.endsWith('/articles'))!;
      expect((await post.json()).article.tagList).toEqual(['one', 'two']);
    });
  });
});
