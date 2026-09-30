import { act } from '@testing-library/react';
import { http, HttpResponse } from 'msw';
import { ApiError } from '../../src/api/client';
import { shouldRetry } from '../../src/App';
import { createArticle, createUser, findArticle, findProfile, serializeArticle } from '../mocks/db';
import { API } from '../mocks/handlers';
import { requests, server } from '../mocks/server';
import { logInAs, visit, waitFor } from '../helpers/app';

describe('Acceptance | code review fixes (round 3)', () => {
  it('a title edit that changes the slug does not leave stale data under the old slug', async () => {
    const user = createUser();
    logInAs();
    const article = createArticle({ author: findProfile(user.username), slug: 'old-slug', title: 'Old' });
    // Mimic the real API, which re-slugs on title change.
    server.use(
      http.put(`${API}/articles/:slug`, async ({ request }) => {
        const { article: attrs } = (await request.json()) as { article: { title: string } };
        Object.assign(findArticle('old-slug')!, { ...attrs, slug: 'new-slug' });
        return HttpResponse.json({ article: serializeArticle(findArticle('new-slug')!) });
      }),
    );

    const app = await visit(`/articles/${article.slug}`);
    await app.user.click(await app.find('[data-test-edit-article-button]'));
    const title = await app.find('[data-test-article-form-input-title]');
    await app.user.clear(title);
    await app.user.type(title, 'New');
    await app.user.click(app.$('[data-test-article-form-submit-button]')!);
    await waitFor(() => expect(app.currentURL()).toBe('/articles/new-slug'));

    // Visiting the old URL must hit the server (now 404) rather than show cached "Old"/"New" data.
    await act(() => app.router.navigate('/articles/old-slug'));
    await app.find('[data-test-error-page]');
  });

  it('retries transient failures but not client errors', () => {
    expect(shouldRetry(0, new TypeError('Failed to fetch'))).toBe(true);
    expect(shouldRetry(0, new ApiError(503, []))).toBe(true);
    expect(shouldRetry(2, new ApiError(503, []))).toBe(false);
    expect(shouldRetry(0, new ApiError(404, []))).toBe(false);
    expect(shouldRetry(0, new ApiError(422, ['title is bad']))).toBe(false);
  });

  it('renaming yourself refreshes cached author data', async () => {
    const user = createUser({ image: null });
    logInAs();
    const article = createArticle({ author: findProfile(user.username) });

    const app = await visit(`/articles/${article.slug}`);
    await app.find(`.article-meta a.author`);
    await act(() => app.router.navigate('/settings'));
    const username = await app.find('[data-test-settings-form-input-username]');
    await app.user.clear(username);
    await app.user.type(username, 'renamed');
    await app.user.click(app.$('[data-test-settings-form-button]')!);
    await waitFor(() => expect(app.$('[data-test-nav-username]')).toHaveTextContent('renamed'));

    requests.length = 0;
    await act(() => app.router.navigate(`/articles/${article.slug}`));
    await waitFor(() => expect(app.$('.article-meta a.author')).toHaveTextContent('renamed'));
    expect(app.$('.article-meta a.author')).toHaveAttribute('href', '/profile/renamed');
    expect(app.$('[data-test-edit-article-button]')).toBeInTheDocument();
  });
});
