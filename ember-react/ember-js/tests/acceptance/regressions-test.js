import { module, test } from 'qunit';
import {
  visit,
  currentURL,
  currentRouteName,
  fillIn,
  click,
  triggerKeyEvent,
  waitFor,
  waitUntil,
} from '@ember/test-helpers';
import { setupApplicationTest } from 'ember-qunit';
import setupMirage from 'ember-cli-mirage/test-support/setup-mirage';
import { setupLoggedOutUser, setupLoggedInUser } from '../helpers/user';
import sinon from 'sinon';

module('Acceptance | regressions', function (hooks) {
  setupApplicationTest(hooks);
  setupMirage(hooks);

  let requests;
  hooks.beforeEach(function () {
    requests = [];
    this.server.pretender.handledRequest = (verb, path, request) => requests.push(request);
  });

  module('logged-out user', function (hooks) {
    setupLoggedOutUser(hooks);

    test('Ember Data requests carry the token after logging in without a reload', async function (assert) {
      let user = this.server.create('user', { email: 'bob@example.com', password: 'secret' });
      let article = this.server.create('article');

      // Touch the store first so the adapter is instantiated while logged out
      await visit('/');
      await visit('/login');
      await fillIn('[data-test-login-email]', user.email);
      await fillIn('[data-test-login-password]', user.password);
      await click('[data-test-login-button]');
      await waitFor('[data-test-nav-username]');

      await visit(`/articles/${article.slug}`);

      let request = requests.filter((r) => r.url.endsWith(`/articles/${article.slug}`)).pop();
      assert.equal(request.requestHeaders.Authorization, `Token ${user.token}`);
    });

    test('?feed=your while logged out falls back to the global feed', async function (assert) {
      this.server.createList('article', 3);

      await visit('/?feed=your');

      assert.dom('[data-test-article-preview]').exists({ count: 3 });
    });

    test('favorite button has no debug text', async function (assert) {
      let article = this.server.create('article');

      await visit('/');

      assert
        .dom(`[data-test-article-preview="${article.slug}"] .article-meta`)
        .doesNotContainText('article favorited');
    });

    test('article markdown is sanitized', async function (assert) {
      let article = this.server.create('article', {
        body:
          '# Hi\n\n<img src=x onerror="window.__xss = true"><a href="javascript:alert(1)">x</a>',
      });

      await visit(`/articles/${article.slug}`);

      assert.dom('[data-test-article-body] h1').hasText('Hi');
      assert.dom('[data-test-article-body] img[onerror]').doesNotExist();
      assert.dom('[data-test-article-body] a[href^="javascript"]').doesNotExist();
      assert.notOk(window.__xss);
    });
  });

  module('logged-in user', function (hooks) {
    setupLoggedInUser(hooks);

    let user;

    hooks.beforeEach(function () {
      user = this.server.create('user');
    });

    hooks.afterEach(function () {
      sinon.restore();
    });

    test('paginating "Your Feed" stays on the feed', async function (assert) {
      this.server.createList('article', 25);
      this.server.get('/articles/feed', (schema, request) => {
        let limit = parseInt(request.queryParams.limit, 10);
        let offset = parseInt(request.queryParams.offset, 10);
        return {
          articles: schema.articles.all().models.slice(offset, offset + limit),
          articlesCount: 15,
        };
      });

      await visit('/?feed=your');

      assert
        .dom('[data-test-page-item]')
        .exists({ count: 2 }, 'feed pagination uses articlesCount');
      await click('[data-test-page-item-link="2"]');

      assert.equal(currentURL(), '/?feed=your&page=2');
      await waitUntil(() => requests.some((r) => r.url.includes('offset=10')));
      let feedRequest = requests.filter((r) => r.url.includes('/articles/feed')).pop();
      assert.equal(feedRequest.queryParams.offset, '10');
      assert.dom('[data-test-tab="your"]').hasClass('active');
    });

    test('log out clears the user and returns home', async function (assert) {
      await visit('/settings');
      await click('[data-test-nav-log-out]');

      assert.equal(currentURL(), '/');
      assert.dom('[data-test-nav-sign-in]').exists();
      assert.equal(this.owner.lookup('service:session').user, null);
    });

    test("cannot open the editor for someone else's article", async function (assert) {
      let article = this.server.create('article');

      await visit(`/editor/${article.slug}`);

      assert.equal(currentRouteName(), 'articles.article');
      assert.notOk(user.username === article.author.username);
    });

    test('cancelling the unsaved-changes prompt keeps you in the editor', async function (assert) {
      let profile = this.server.schema.profiles.findBy({ username: user.username });
      let article = this.server.create('article', { author: profile });
      let confirm = sinon.stub(window, 'confirm').returns(false);

      await visit(`/editor/${article.slug}`);
      await fillIn('[data-test-article-form-input-title]', 'Changed');
      await visit('/').catch(() => {});

      assert.ok(confirm.calledOnce, 'user was prompted');
      assert.equal(currentRouteName(), 'editor.edit', 'transition was aborted');
      assert.dom('[data-test-article-form-input-title]').hasValue('Changed');
    });

    test('tags ignore extra whitespace', async function (assert) {
      await visit('/editor');
      await fillIn('[data-test-article-form-input-title]', 'T');
      await fillIn('[data-test-article-form-input-description]', 'D');
      await fillIn('[data-test-article-form-input-body]', 'B');
      await fillIn('[data-test-article-form-input-tags]', '  one   two ');
      await triggerKeyEvent('[data-test-article-form-input-tags]', 'keyup', 'Space');
      await click('[data-test-article-form-submit-button]');

      assert.equal(currentRouteName(), 'articles.article');
      // Check what the app sent; the Mirage handler strips blanks itself
      let post = requests.find((r) => r.method === 'POST' && r.url.endsWith('/articles'));
      assert.deepEqual(JSON.parse(post.requestBody).article.tagList, ['one', 'two']);
    });
  });
});
