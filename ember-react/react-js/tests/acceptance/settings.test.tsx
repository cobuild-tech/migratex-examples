import { createUser, db } from '../mocks/db';
import { logInAs, visit, waitFor } from '../helpers/app';

describe('Acceptance | settings', () => {
  it('logged-out user is redirected to login', async () => {
    const app = await visit('/settings');
    expect(app.currentURL()).toBe('/login');
  });

  describe('logged-in user', () => {
    beforeEach(() => {
      createUser({ email: 'bob@example.com', password: 'password123' });
      logInAs();
    });

    it('can edit their settings', async () => {
      const app = await visit('/settings');
      const newSettings = {
        image: 'image',
        bio: 'bio',
        username: 'username',
        password: 'password',
        email: 'email@email.com',
      };
      const input = (key: string) => app.$(`[data-test-settings-form-input-${key}]`)!;

      for (const [key, value] of Object.entries(newSettings)) {
        expect(input(key)).not.toHaveValue(value);
        await app.user.clear(input(key));
        await app.user.type(input(key), value);
      }
      await app.user.click(app.$('[data-test-settings-form-button]')!);

      await waitFor(() => expect(db.users[0].username).toBe('username'));
      for (const [key, value] of Object.entries(newSettings)) {
        expect(input(key)).toHaveValue(value);
      }
      await waitFor(() => expect(app.$('[data-test-nav-username]')).toHaveTextContent('username'));
      expect(app.$('[data-test-settings-form-button]')).toBeDisabled();
    });

    it('shows settings errors from server', async () => {
      const app = await visit('/settings');
      const username = app.$('[data-test-settings-form-input-username]')!;
      await app.user.clear(username);
      await app.user.type(username, 'a'.repeat(21));
      await app.user.clear(app.$('[data-test-settings-form-input-email]')!);
      await app.user.click(app.$('[data-test-settings-form-button]')!);

      await waitFor(() => expect(app.$$('[data-test-settings-form-error-item]')).toHaveLength(2));
      expect(app.$('[data-test-settings-form-error-item="0"]')).toHaveTextContent(
        'username is too long (maximum is 20 characters)',
      );
      expect(app.$('[data-test-settings-form-error-item="1"]')).toHaveTextContent("email can't be blank");
      expect(app.currentURL()).toBe('/settings');
    });
  });
});
