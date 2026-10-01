import { delay, http, HttpResponse } from 'msw';
import { LEGACY_USER_SEARCH_URL } from '../../src/api/github';
import { visit } from '../helpers/app';
import { user } from '../mocks/data';
import { requests, server } from '../mocks/server';

describe('home page', () => {
  it('renders the Git Info page with the footer and no Back button', async () => {
    const app = visit('/');
    expect(app.$('#home-page .ui-header h1')).toHaveTextContent('Git Info');
    expect(app.$('#back-btn')).toBeNull();
    expect(app.$('.ui-footer')).toHaveTextContent('Version - 1.0.0');
    expect(app.$('.avatar-container')).not.toHaveClass('is-open');
  });

  it('looks the user up with the legacy search and shows their avatar and buttons', async () => {
    const app = visit('/');
    await app.findUser('octocat');
    expect(requests.map((r) => r.url)).toEqual([`${LEGACY_USER_SEARCH_URL}octocat`]);
    expect(app.$('.avatar-container')).toHaveClass('is-open');
    expect(app.$('#gh-avatar')).toHaveAttribute('src', `https://www.gravatar.com/avatar/${user.gravatar_id}?s=200`);
    expect(app.$('#get-user-activity')).toHaveTextContent('Get User Activity');
    expect(app.$('#get-user-repositores')).toHaveTextContent('Get User Repositories');
  });

  it('pressing Enter in the input also looks the user up', async () => {
    const app = visit('/');
    await app.user.type(app.$('#gh-username')!, 'octocat{Enter}');
    await app.settled();
    expect(app.$('.avatar-container')).toHaveClass('is-open');
  });

  it('alerts "No User Found" when the search matches nobody', async () => {
    const alert = vi.spyOn(window, 'alert').mockImplementation(() => {});
    const app = visit('/');
    await app.findUser('nobody-here');
    await vi.waitFor(() => expect(alert).toHaveBeenCalledWith('No User Found'));
    expect(app.$('.avatar-container')).not.toHaveClass('is-open');
  });

  it('hides a found user, then alerts after the slide-up, when the next search misses', async () => {
    const alert = vi.spyOn(window, 'alert').mockImplementation(() => {});
    const app = visit('/');
    await app.findUser('octocat');
    await app.findUser('nobody-here');
    expect(app.$('.avatar-container')).not.toHaveClass('is-open');
    expect(alert).not.toHaveBeenCalled();
    await vi.waitFor(() => expect(alert).toHaveBeenCalledWith('No User Found'), { timeout: 1500 });
  });

  it('does nothing when the search request fails, like the Backbone app', async () => {
    server.use(http.get(`${LEGACY_USER_SEARCH_URL}:name`, () => new HttpResponse(null, { status: 404 })));
    const alert = vi.spyOn(window, 'alert').mockImplementation(() => {});
    const app = visit('/');
    await app.findUser('octocat');
    await new Promise((r) => setTimeout(r, 50));
    expect(alert).not.toHaveBeenCalled();
    expect(app.$('.avatar-container')).not.toHaveClass('is-open');
  });

  it('shows the "Connecting To Github" loader and disables the page during a request', async () => {
    server.use(
      http.get(`${LEGACY_USER_SEARCH_URL}:name`, async () => {
        await delay(100);
        return HttpResponse.json({ users: [user] });
      }),
    );
    const app = visit('/');
    await app.user.type(app.$('#gh-username')!, 'octocat');
    await app.user.click(app.$('#find-user')!);
    expect(await app.find('.ui-loader')).toHaveTextContent('Connecting To Github');
    expect(document.body).toHaveClass('ui-disabled');
    await app.settled();
    expect(document.body).not.toHaveClass('ui-disabled');
  });
});
