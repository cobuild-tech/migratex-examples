import { createArticle, createComment, createProfile, createUser, findArticle, findProfile } from '../mocks/db';
import { logInAs, visit, waitFor } from '../helpers/app';

describe('Acceptance | article', () => {
  let username: string;

  beforeEach(() => {
    username = createUser({ email: 'bob@example.com', password: 'password123' }).username;
    logInAs();
  });

  it('visiting /articles/:slug', async () => {
    const article = createArticle({ author: createProfile() });
    const app = await visit(`/articles/${article.slug}`);
    expect(await app.find('[data-test-article-title]')).toHaveTextContent(article.title);
    expect(app.currentURL()).toBe(`/articles/${article.slug}`);
  });

  it('favorite article', async () => {
    const article = createArticle({ author: createProfile(), favorited: false });
    const app = await visit(`/articles/${article.slug}`);

    await app.user.click(await app.find('[data-test-favorite-article-button]'));
    await waitFor(() => expect(findArticle(article.slug)!.favorited).toBe(true));
    await app.find('[data-test-favorite-article-button="favorited"]');

    await app.user.click(app.$('[data-test-favorite-article-button]')!);
    await waitFor(() => expect(findArticle(article.slug)!.favorited).toBe(false));
  });

  it('follow author', async () => {
    const article = createArticle({ author: createProfile({ following: false }), favorited: false });
    const app = await visit(`/articles/${article.slug}`);

    await app.user.click(await app.find('[data-test-follow-author-button]'));
    await waitFor(() => expect(app.$('[data-test-follow-author-button]')).toHaveTextContent('Unfollow'));

    await app.user.click(app.$('[data-test-follow-author-button]')!);
    await waitFor(() => expect(app.$('[data-test-follow-author-button]')).not.toHaveTextContent('Unfollow'));
    expect(app.$('[data-test-follow-author-button]')).toHaveTextContent('Follow');
  });

  it('edit article', async () => {
    const article = createArticle({ author: findProfile(username) });
    const app = await visit(`/articles/${article.slug}`);

    await app.user.click(await app.find('[data-test-edit-article-button]'));
    expect(app.currentURL()).toBe(`/editor/${article.slug}`);
    expect(await app.find('[data-test-article-form-input-title]')).toHaveValue(article.title);
  });

  it('delete article', async () => {
    const article = createArticle({ author: findProfile(username) });
    const app = await visit(`/articles/${article.slug}`);

    await app.user.click(await app.find('[data-test-delete-article-button]'));
    await waitFor(() => expect(app.currentURL()).toBe('/'));
    expect(findArticle(article.slug)).toBeUndefined();
  });

  it('post comment', async () => {
    const article = createArticle({ author: createProfile() });
    const app = await visit(`/articles/${article.slug}`);

    const textarea = await app.find('[data-test-article-comment-textarea]');
    expect(app.$('[data-test-article-comment]')).toBeNull();
    await app.user.type(textarea, 'foo!');
    await app.user.click(app.$('[data-test-article-comment-button]')!);

    await waitFor(() => expect(app.$$('[data-test-article-comment]')).toHaveLength(1));
    expect(app.$('[data-test-article-comment-body]')).toHaveTextContent('foo!');
    expect(textarea).toHaveValue('');
  });

  it('delete comment', async () => {
    const article = createArticle({ author: createProfile() });
    createComment(article, findProfile(username)!);
    const app = await visit(`/articles/${article.slug}`);

    await app.find('[data-test-article-comment]');
    expect(app.$$('[data-test-article-comment]')).toHaveLength(1);
    await app.user.click(app.$('[data-test-article-comment-delete-button]')!);
    await waitFor(() => expect(app.$('[data-test-article-comment]')).toBeNull());
  });
});
