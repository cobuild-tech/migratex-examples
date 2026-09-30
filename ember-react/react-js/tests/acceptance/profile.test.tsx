import { createArticle, createArticles, createProfile, createUser, findProfile } from '../mocks/db';
import { logInAs, visit, waitFor } from '../helpers/app';

describe('Acceptance | profile', () => {
  describe('logged out user', () => {
    it('does not show the edit profile button', async () => {
      const owner = createProfile();
      const app = await visit(`/profile/${owner.username}`);
      await app.find('[data-test-follow-author-button]');
      expect(app.$('[data-test-edit-profile-button]')).toBeNull();
    });

    it('follow button links to login', async () => {
      const owner = createProfile();
      const app = await visit(`/profile/${owner.username}`);
      await app.user.click(await app.find('[data-test-follow-author-button]'));
      expect(app.currentURL()).toBe('/login');
    });

    it('tabs between articles written and favorited by the owner', async () => {
      const owner = createProfile();
      const other = createProfile();
      createArticles(2, { author: owner, favorited: false });
      createArticles(3, { author: other, favorited: true });

      const app = await visit(`/profile/${owner.username}`);
      await app.user.click(await app.find('[data-test-profile-tab="favorite-articles"]'));
      expect(app.currentURL()).toBe(`/profile/${owner.username}/favorites`);
      await waitFor(() => expect(app.$$('[data-test-article-title]')).toHaveLength(3));
      expect(app.$('[data-test-profile-tab="favorite-articles"]')).toHaveClass('active');

      await app.user.click(app.$('[data-test-profile-tab="my-articles"]')!);
      await waitFor(() => expect(app.$$('[data-test-article-title]')).toHaveLength(2));
      expect(app.currentURL()).toBe(`/profile/${owner.username}`);
      expect(app.$('[data-test-profile-tab="my-articles"]')).toHaveClass('active');
    });

    it("clicking an article's favorite button redirects to login", async () => {
      const owner = createProfile();
      createArticle({ author: owner, favorited: false });
      const app = await visit(`/profile/${owner.username}`);
      await app.user.click(await app.find('[data-test-favorite-article-button]'));
      expect(app.currentURL()).toBe('/login');
    });
  });

  describe('logged in user', () => {
    let username: string;

    beforeEach(() => {
      username = createUser({ email: 'bob@example.com', password: 'password123' }).username;
      logInAs();
    });

    it('own profile links to settings', async () => {
      const app = await visit(`/profile/${username}`);
      await app.user.click(await app.find('[data-test-edit-profile-button]'));
      expect(app.currentURL()).toBe('/settings');
    });

    it('can follow and unfollow another profile', async () => {
      const other = createProfile({ following: false });
      const app = await visit(`/profile/${other.username}`);
      const button = await app.find('[data-test-follow-author-button]');
      expect(button).toHaveTextContent(`Follow ${other.username}`);

      await app.user.click(button);
      await waitFor(() => expect(button).toHaveTextContent(`Unfollow ${other.username}`));
      expect(findProfile(other.username)!.following).toBe(true);

      await app.user.click(button);
      await waitFor(() => expect(button).not.toHaveTextContent('Unfollow'));
    });

    it('can favorite an article', async () => {
      const owner = createProfile();
      createArticle({ author: owner, favorited: false });
      const app = await visit(`/profile/${owner.username}`);
      await app.user.click(await app.find('[data-test-favorite-article-button]'));
      await app.find('[data-test-favorite-article-button="favorited"]');
    });
  });
});
