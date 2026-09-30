import { faker } from '@faker-js/faker';
import { createUser } from '../mocks/db';
import { visit, waitFor } from '../helpers/app';

describe('Acceptance | register', () => {
  it('successful registration', async () => {
    const user = createUser({ username: 'test_user', email: faker.internet.email(), password: 'password123' });
    const app = await visit('/register');

    await app.user.type(app.$('[data-test-register-username]')!, user.username);
    await app.user.type(app.$('[data-test-register-email]')!, user.email);
    await app.user.type(app.$('[data-test-register-password]')!, 'password123');
    await app.user.click(app.$('[data-test-register-button]')!);

    await waitFor(() => expect(app.currentURL()).toBe('/'));
    expect(await app.find('[data-test-nav-username]')).toHaveTextContent(user.username);
    expect(app.$('[data-test-nav-new-post]')).toBeInTheDocument();
    expect(app.$('[data-test-nav-sign-up]')).toBeNull();
  });

  it('has a link to /login', async () => {
    const app = await visit('/register');
    await app.user.click(app.$('[data-test-login-link]')!);
    expect(app.currentURL()).toBe('/login');
  });
});
