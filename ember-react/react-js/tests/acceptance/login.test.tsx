import { createUser } from '../mocks/db';
import { visit, waitFor } from '../helpers/app';

describe('Acceptance | login', () => {
  it('visiting /login', async () => {
    const user = createUser({ email: 'bob@example.com', password: 'password123' });
    const app = await visit('/login');

    await app.user.type(app.$('[data-test-login-email]')!, user.email);
    await app.user.type(app.$('[data-test-login-password]')!, 'password123');
    await app.user.click(app.$('[data-test-login-button]')!);

    await waitFor(() => expect(app.currentURL()).toBe('/'));
    expect(await app.find('[data-test-nav-username]')).toHaveTextContent(user.username);
    expect(app.$('[data-test-nav-new-post]')).toBeInTheDocument();
    expect(app.$('[data-test-nav-sign-up]')).toBeNull();
  });

  it('shows login errors from the server', async () => {
    const app = await visit('/login');
    await app.user.type(app.$('[data-test-login-email]')!, 'nobody@example.com');
    await app.user.type(app.$('[data-test-login-password]')!, 'nope');
    await app.user.click(app.$('[data-test-login-button]')!);

    expect(await app.find('.error-messages li')).toHaveTextContent('email or password is invalid');
    expect(app.currentURL()).toBe('/login');
  });

  it('has a link to /register', async () => {
    const app = await visit('/login');
    await app.user.click(app.$('[data-test-register-link]')!);
    expect(app.currentURL()).toBe('/register');
  });
});
