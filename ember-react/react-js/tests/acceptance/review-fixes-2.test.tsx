import { http, HttpResponse, delay } from 'msw';
import { createArticle, createArticles, createComment, createProfile, createUser, findProfile } from '../mocks/db';
import { API } from '../mocks/handlers';
import { requests, server } from '../mocks/server';
import { logInAs, visit, waitFor } from '../helpers/app';

describe('Acceptance | code review fixes (round 2)', () => {
  it('encodes usernames and slugs in links and API paths', async () => {
    const author = createProfile({ username: 'a?b#c' });
    createArticle({ author, slug: 'x?y' });

    const app = await visit('/');
    const authorLink = await app.find('.article-preview a.author');
    expect(authorLink).toHaveAttribute('href', '/profile/a%3Fb%23c');
    expect(app.$('.preview-link')).toHaveAttribute('href', '/articles/x%3Fy');

    await app.user.click(authorLink);
    await app.find('.profile-page h4');
    expect(app.$('.profile-page h4')).toHaveTextContent('a?b#c');
    expect(requests.some((r) => r.url === `${API}/profiles/a%3Fb%23c`)).toBe(true);
  });

  it('a failed delete is reported once', async () => {
    const user = createUser();
    logInAs();
    const article = createArticle({ author: findProfile(user.username) });
    server.use(http.delete(`${API}/articles/:slug`, () => HttpResponse.json({}, { status: 500 })));

    const app = await visit(`/articles/${article.slug}`);
    await app.user.click((await app.find('[data-test-delete-article-button]')));
    await app.find('[data-test-delete-article-error]');
    expect(app.$$('[data-test-delete-article-error]')).toHaveLength(1);
  });

  it('clamps a negative page to 1', async () => {
    createArticles(3);
    const app = await visit('/?page=-1');
    await waitFor(() => expect(app.$$('[data-test-article-preview]')).toHaveLength(3));
    const list = requests.find((r) => r.url.includes('/articles?'))!;
    expect(new URL(list.url).searchParams.get('offset')).toBe('0');
  });

  it('a page past the end still shows pagination to get back', async () => {
    createArticles(3);
    const app = await visit('/?page=5');
    await app.find('[data-test-page-item-link="1"]');
    expect(app.$('.article-preview')).toHaveTextContent('No articles are here... yet.');
  });

  it('a failed comment delete keeps the comment and reports it', async () => {
    const user = createUser();
    logInAs();
    const article = createArticle();
    createComment(article, findProfile(user.username)!);
    server.use(http.delete(`${API}/articles/:slug/comments/:id`, () => HttpResponse.json({}, { status: 500 })));

    const app = await visit(`/articles/${article.slug}`);
    await app.user.click(await app.find('[data-test-article-comment-delete-button]'));
    await app.find('[data-test-comment-error]');
    expect(app.$$('[data-test-article-comment]')).toHaveLength(1);
  });

  it('a failed comment post keeps the draft', async () => {
    createUser();
    logInAs();
    const article = createArticle();
    server.use(http.post(`${API}/articles/:slug/comments`, () => HttpResponse.json({}, { status: 500 })));

    const app = await visit(`/articles/${article.slug}`);
    const textarea = await app.find('[data-test-article-comment-textarea]');
    await app.user.type(textarea, 'my draft');
    await app.user.click(app.$('[data-test-article-comment-button]')!);
    await app.find('[data-test-comment-error]');
    expect(textarea).toHaveValue('my draft');
  });

  it('favorite and follow buttons are disabled while a request is in flight', async () => {
    createUser();
    logInAs();
    const article = createArticle({ author: createProfile({ following: false }), favorited: false });
    server.use(
      http.post(`${API}/articles/:slug/favorite`, async () => {
        await delay('infinite');
        return HttpResponse.json({});
      }),
      http.post(`${API}/profiles/:username/follow`, async () => {
        await delay('infinite');
        return HttpResponse.json({});
      }),
    );

    const app = await visit(`/articles/${article.slug}`);
    const favorite = await app.find('[data-test-favorite-article-button]');
    const follow = app.$('[data-test-follow-author-button]')!;
    await app.user.click(favorite);
    await app.user.click(follow);
    await waitFor(() => expect(favorite).toBeDisabled());
    expect(follow).toBeDisabled();
    await app.user.click(favorite);
    expect(requests.filter((r) => r.url.endsWith('/favorite'))).toHaveLength(1);
  });
});
