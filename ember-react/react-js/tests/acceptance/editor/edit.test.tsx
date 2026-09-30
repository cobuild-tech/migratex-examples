import { createArticle, createUser, findProfile } from '../../mocks/db';
import { logInAs, visit, waitFor, type AppHarness } from '../../helpers/app';

async function fill(app: AppHarness, field: string, value: string) {
  const el = app.$(`[data-test-article-form-input-${field}]`)!;
  await app.user.clear(el);
  if (value) await app.user.type(el, value);
}

describe('Acceptance | editor (edit)', () => {
  beforeEach(() => {
    vi.spyOn(window, 'confirm').mockReturnValue(true);
  });

  it('anonymous user is transitioned to login', async () => {
    const app = await visit('/editor/foo');
    expect(app.currentURL()).toBe('/login');
  });

  describe('logged-in user', () => {
    let slug: string;

    beforeEach(() => {
      const user = createUser();
      slug = createArticle({ author: findProfile(user.username) }).slug;
      logInAs();
    });

    it('can edit their own article', async () => {
      const app = await visit(`/editor/${slug}`);
      await app.find('[data-test-article-form-input-title]');
      await fill(app, 'title', 'Test Title');
      await fill(app, 'description', 'Test Description');
      await fill(app, 'body', 'Test Body');
      await fill(app, 'tags', 'test-tag{Enter}');
      await app.user.click(app.$('[data-test-article-form-submit-button]')!);

      await waitFor(() => expect(app.currentURL()).toBe(`/articles/${slug}`));
      expect(window.confirm).not.toHaveBeenCalled();
      expect(await app.find('h1[data-test-article-title]')).toHaveTextContent('Test Title');
      expect(app.$('[data-test-article-body]')).toHaveTextContent('Test Body');
    });

    it('shows article errors from server', async () => {
      const app = await visit(`/editor/${slug}`);
      await app.find('[data-test-article-form-input-title]');
      await fill(app, 'title', 'Test Title');
      await fill(app, 'description', 'Test Description');
      await fill(app, 'body', '');
      await app.user.click(app.$('[data-test-article-form-submit-button]')!);

      await waitFor(() => expect(app.$$('[data-test-article-form-error-item]')).toHaveLength(1));
      expect(app.$('[data-test-article-form-error-item]')).toHaveTextContent("body can't be blank");
      expect(app.currentURL()).toBe(`/editor/${slug}`);
    });
  });
});
