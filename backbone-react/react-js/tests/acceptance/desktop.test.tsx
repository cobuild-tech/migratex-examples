import { visit } from '../helpers/app';
import { requests } from '../mocks/server';

const EVENT_ROWS = [
  'PushEvent |  | octocat/hello-world',
  'IssuesEvent | opened | octocat/spoon-knife',
  'WatchEvent | started | github/linguist',
];

describe('desktop: everything renders as panels on the home page', () => {
  it('shows the activity table in the events panel without leaving home', async () => {
    const app = visit('/');
    await app.findUser();
    await app.click('#get-user-activity');
    await app.settled();
    expect(app.currentURL()).toBe('/');
    expect(app.$$('#events-container th').slice(0, 3).map((th) => th.textContent)).toEqual([
      'Event',
      'Action',
      'Repository',
    ]);
    expect(app.rows('#events-container')).toEqual(EVENT_ROWS);
    expect(app.$('#events-container #back-btn')).toBeNull();
  });

  it('refetches the activity on every click, replacing the panel', async () => {
    const app = visit('/');
    await app.findUser();
    await app.click('#get-user-activity');
    await app.settled();
    await app.click('#get-user-activity');
    await app.settled();
    expect(requests.filter((r) => r.url.endsWith('/events'))).toHaveLength(2);
    expect(app.$$('#events-container table')).toHaveLength(1);
    expect(app.rows('#events-container')).toEqual(EVENT_ROWS);
  });

  it('shows repo categories, then the filtered repo table', async () => {
    const app = visit('/');
    await app.findUser();
    await app.click('#get-user-repositores');
    await app.settled();
    expect(app.$('#repos-category-container #all-repos')).toHaveTextContent('All');
    expect(app.$('#repos-container table')).toBeNull();

    await app.click('#all-repos');
    expect(app.rows('#repos-container')).toEqual([
      'hello-world | 1800 | 1700',
      'spoon-knife | 12000 | 140000',
      'linguist | 40 | 12',
    ]);

    await app.click('#source-repos');
    expect(app.rows('#repos-container')).toEqual(['hello-world | 1800 | 1700', 'spoon-knife | 12000 | 140000']);

    await app.click('#fork-repos');
    expect(app.rows('#repos-container')).toEqual(['linguist | 40 | 12']);
    expect(requests.filter((r) => r.url.endsWith('/repos'))).toHaveLength(1);
  });

  it('a second "Get User Repositories" clears the table and refetches', async () => {
    const app = visit('/');
    await app.findUser();
    await app.click('#get-user-repositores');
    await app.settled();
    await app.click('#all-repos');
    await app.click('#get-user-repositores');
    await app.settled();
    expect(app.$('#repos-container table')).toBeNull();
    expect(app.$('#repos-category-container #all-repos')).not.toBeNull();
    expect(requests.filter((r) => r.url.endsWith('/repos'))).toHaveLength(2);
  });

  it('uses the username currently in the input', async () => {
    const app = visit('/');
    await app.findUser();
    await app.user.clear(app.$('#gh-username')!);
    await app.user.type(app.$('#gh-username')!, 'someone-else');
    await app.click('#get-user-activity');
    await app.settled();
    expect(requests[requests.length - 1].url).toBe('https://api.github.com/users/someone-else/events');
    expect(app.rows('#events-container')).toEqual([]);
  });
});
