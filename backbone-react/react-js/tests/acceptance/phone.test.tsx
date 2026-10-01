import { act } from '@testing-library/react';
import { visit } from '../helpers/app';
import { resizeTo } from '../helpers/media';

describe('phone: each view is its own page', () => {
  it('opens the activity page with a Back header and the footer', async () => {
    const app = visit('/', { phone: true });
    await app.findUser();
    await app.click('#get-user-activity');
    await app.settled();
    expect(app.currentURL()).toBe('/activity');
    expect(app.$('#home-page')).toBeNull();
    expect(app.$('#activity-page .ui-header h1')).toHaveTextContent('Git Activity');
    expect(app.$('#activity-page #back-btn')).toHaveTextContent('Back');
    expect(app.$('#activity-page .ui-footer')).toHaveTextContent('Version - 1.0.0');
    expect(app.rows('#activity-page')).toHaveLength(3);
    expect(app.$('#activity-page')).toHaveClass('slide-in');
  });

  it('Back from activity returns home with the username and avatar kept', async () => {
    const app = visit('/', { phone: true });
    await app.findUser();
    await app.click('#get-user-activity');
    await app.click('#back-btn');
    expect(app.currentURL()).toBe('/');
    expect(app.$('#home-page')).toHaveClass('slide-in', 'reverse');
    expect(app.$('#gh-username')).toHaveValue('octocat');
    expect(app.$('.avatar-container')).toHaveClass('is-open');
    expect(app.$('#events-container table')).toBeNull();
  });

  it('walks repos → categories → filtered table → back to categories → home', async () => {
    const app = visit('/', { phone: true });
    await app.findUser();
    await app.click('#get-user-repositores');
    await app.settled();
    expect(app.currentURL()).toBe('/repos');
    expect(app.$('#repo-category-page h1')).toHaveTextContent('Git Repo Categories');

    await app.click('#fork-repos');
    expect(app.currentURL()).toBe('/repos/fork');
    expect(app.$('#repo-page h1')).toHaveTextContent('Git Repos');
    expect(app.rows('#repo-page')).toEqual(['linguist | 40 | 12']);

    await app.click('#back-btn');
    expect(app.currentURL()).toBe('/repos');
    await app.click('#source-repos');
    expect(app.rows('#repo-page')).toHaveLength(2);

    await app.click('#back-btn');
    await app.click('#back-btn');
    expect(app.currentURL()).toBe('/');
  });

  it('redirects deep links to home when nothing was requested', async () => {
    for (const url of ['/activity', '/repos', '/repos/all', '/nope']) {
      const app = visit(url, { phone: true });
      await app.find('#home-page');
      expect(app.currentURL()).toBe('/');
      app.unmount();
    }
  });

  it('redirects an unknown repo filter to the categories page', async () => {
    const app = visit('/', { phone: true });
    await app.findUser();
    await app.click('#get-user-repositores');
    await app.settled();
    await act(() => app.router.navigate('/repos/forks'));
    await app.find('#repo-category-page');
    expect(app.currentURL()).toBe('/repos');
  });
});

describe('resizing across the breakpoint', () => {
  it('widening on a phone page goes home, where the panel shows the same data', async () => {
    const app = visit('/', { phone: true });
    await app.findUser();
    await app.click('#get-user-activity');
    await app.settled();
    resizeTo('desktop');
    await app.find('#home-page');
    expect(app.currentURL()).toBe('/');
    expect(app.rows('#events-container')).toHaveLength(3);
  });

  it('narrowing on desktop hides the panels; the buttons then open pages', async () => {
    const app = visit('/');
    await app.findUser();
    await app.click('#get-user-repositores');
    await app.settled();
    resizeTo('phone');
    expect(app.$('#repos-category-container #all-repos')).toBeNull();
    await app.click('#get-user-repositores');
    expect(app.currentURL()).toBe('/repos');
  });
});
