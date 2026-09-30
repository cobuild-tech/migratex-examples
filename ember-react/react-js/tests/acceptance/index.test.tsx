import { http, HttpResponse } from 'msw';
import { createArticles, createUser, db, serializeArticle } from '../mocks/db';
import { API } from '../mocks/handlers';
import { server } from '../mocks/server';
import { logInAs, visit, waitFor, type AppHarness } from '../helpers/app';

const previews = (app: AppHarness) => app.$$('[data-test-article-preview]');
const waitForPreviews = (app: AppHarness, count: number) =>
  waitFor(() => expect(previews(app)).toHaveLength(count));

describe('Acceptance | index', () => {
  it('visiting /', async () => {
    createArticles(20);
    const app = await visit('/');
    await waitForPreviews(app, 10);
    await waitFor(() => expect(app.$$('[data-test-tag]')).toHaveLength(7));

    expect(app.currentURL()).toBe('/');
    expect(app.$$('[data-test-page-item]')).toHaveLength(2);
    expect(app.$$('[data-test-tab]')).toHaveLength(1);
    expect(app.$('[data-test-tab="your"]')).toBeNull();
    expect(app.$('[data-test-tab="global"]')).toHaveClass('active');
    expect(app.$('[data-test-page-item="1"]')).toHaveClass('active');
  });

  it('clicking a page', async () => {
    createArticles(20);
    const app = await visit('/');
    await waitForPreviews(app, 10);

    await app.user.click(app.$('[data-test-page-item-link="2"]')!);
    await waitFor(() => expect(app.$('[data-test-page-item="2"]')).toHaveClass('active'));
    await waitForPreviews(app, 10);
    expect(app.currentURL()).toBe('/?page=2');
  });

  it('clicking a tag', async () => {
    createArticles(20);
    const app = await visit('/');
    await waitForPreviews(app, 10);

    await app.user.click(await app.find('[data-test-tag="emberjs"]'));
    await waitForPreviews(app, 10);
    expect(app.currentURL()).toBe('/?tag=emberjs');
    expect(app.$$('.feed-toggle a.nav-link')).toHaveLength(2);
    expect(app.$('[data-test-tab="tag"]')).toHaveClass('active');
    expect(app.$('[data-test-tab="tag"]')).toHaveTextContent('#emberjs');
  });

  it('resetting to the main list', async () => {
    createArticles(20);
    const app = await visit('/?page=2&tag=emberjs');
    await waitForPreviews(app, 10);

    await app.user.click(app.$('[data-test-tab="global"]')!);
    expect(app.currentURL()).toBe('/');
    await waitForPreviews(app, 10);
    expect(app.$('[data-test-tab="global"]')).toHaveClass('active');
    await waitFor(() => expect(app.$('[data-test-page-item="1"]')).toHaveClass('active'));
  });

  describe('logged in user', () => {
    beforeEach(() => {
      createUser({ email: 'bob@example.com', password: 'password123' });
      logInAs();
    });

    it('Your feed', async () => {
      createArticles(20);
      server.use(
        http.get(`${API}/articles/feed`, () =>
          HttpResponse.json({ articles: [serializeArticle(db.articles[0])], articlesCount: 1 }),
        ),
      );

      const app = await visit('/');
      expect(app.currentURL()).toBe('/');
      expect(app.$('[data-test-tab="global"]')).toHaveClass('active');
      await waitForPreviews(app, 10);

      await app.user.click(app.$('[data-test-tab="your"]')!);
      expect(app.currentURL()).toBe('/?feed=your');
      await waitForPreviews(app, 1);
      expect(app.$('[data-test-tab="your"]')).toHaveClass('active');
    });
  });
});
