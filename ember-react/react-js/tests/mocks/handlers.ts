import { http, HttpResponse } from 'msw';
import {
  db,
  findArticle,
  findProfile,
  serializeArticle,
  serializeComment,
  serializeUser,
  TAGS,
} from './db';

export const API = 'http://api.test';

// Validators ported from ember-webapp/mirage/config.js
const required = (value = '') => (value.trim() ? [] : ["can't be blank"]);
const maxLength = (max: number) => (value = '') =>
  value.trim().length > max ? [`is too long (maximum is ${max} characters)`] : [];

function validate(fields: Record<string, string[]>) {
  const errors = Object.fromEntries(Object.entries(fields).filter(([, messages]) => messages.length));
  return Object.keys(errors).length ? HttpResponse.json({ errors }, { status: 422 }) : null;
}

function currentUser(request: Request) {
  const [type, token] = (request.headers.get('Authorization') ?? '').split(' ');
  return type === 'Token' && token ? db.users.find((u) => u.token === token) : undefined;
}

const notFound = () => HttpResponse.json({ errors: { resource: ['not found'] } }, { status: 404 });

type ArticleBody = { article: { title: string; description: string; body: string; tagList: string[] } };

function validateArticle({ article }: ArticleBody) {
  return validate({
    title: [...required(article.title), ...maxLength(200)(article.title)],
    description: [...required(article.description), ...maxLength(500)(article.description)],
    body: required(article.body),
  });
}

export const handlers = [
  http.post(`${API}/users/login`, async ({ request }) => {
    const { user: attrs } = (await request.json()) as { user: { email: string; password: string } };
    const user = db.users.find((u) => u.email === attrs.email && (!u.password || u.password === attrs.password));
    if (!user) {
      return HttpResponse.json({ errors: { 'email or password': ['is invalid'] } }, { status: 422 });
    }
    return HttpResponse.json({ user: serializeUser(user) });
  }),

  http.post(`${API}/users`, async ({ request }) => {
    const { user: attrs } = (await request.json()) as { user: { email: string } };
    const user = db.users.find((u) => u.email === attrs.email);
    if (!user) {
      return HttpResponse.json({ errors: { email: ['is invalid'] } }, { status: 422 });
    }
    return HttpResponse.json({ user: serializeUser(user) });
  }),

  http.get(`${API}/user`, ({ request }) => {
    const user = currentUser(request);
    return user ? HttpResponse.json({ user: serializeUser(user) }) : HttpResponse.json({}, { status: 401 });
  }),

  http.put(`${API}/user`, async ({ request }) => {
    const user = currentUser(request);
    if (!user) return HttpResponse.json({}, { status: 401 });
    const { user: attrs } = (await request.json()) as { user: Record<string, string> };
    const invalid = validate({
      username: [...required(attrs.username), ...maxLength(20)(attrs.username)],
      email: required(attrs.email),
    });
    if (invalid) return invalid;

    const profile = findProfile(user.username)!;
    Object.assign(profile, { username: attrs.username, bio: attrs.bio, image: attrs.image });
    db.articles.filter((a) => a.author === user.username).forEach((a) => (a.author = attrs.username));
    Object.assign(user, attrs);
    return HttpResponse.json({ user: serializeUser(user) });
  }),

  http.get(`${API}/articles/feed`, () => HttpResponse.json({ articles: [], articlesCount: 0 })),

  http.get(`${API}/articles`, ({ request }) => {
    const params = new URL(request.url).searchParams;
    const author = params.get('author');
    const favorited = params.get('favorited');
    let articles = db.articles;
    if (author) {
      articles = articles.filter((a) => a.author === author);
    } else if (favorited) {
      // Same simplification as Mirage: no per-user favorites, so "favorited by X" means
      // any favorited article not written by X.
      articles = articles.filter((a) => a.favorited && a.author !== favorited);
    }
    // Like the Mirage handler, `tag` is not filtered on.
    const limit = parseInt(params.get('limit') ?? '20', 10);
    const offset = parseInt(params.get('offset') ?? '0', 10);
    return HttpResponse.json({
      articles: articles.slice(offset, offset + limit).map(serializeArticle),
      articlesCount: articles.length,
    });
  }),

  http.post(`${API}/articles`, async ({ request }) => {
    const body = (await request.json()) as ArticleBody;
    const invalid = validateArticle(body);
    if (invalid) return invalid;
    const user = currentUser(request);
    const now = new Date().toISOString();
    const article = {
      ...body.article,
      tagList: body.article.tagList.map((t) => t.trim()).filter(Boolean),
      slug: body.article.title.toLowerCase().replace(/\W+/g, '-'),
      createdAt: now,
      updatedAt: now,
      favorited: false,
      favoritesCount: 0,
      author: user?.username ?? db.profiles[0].username,
    };
    db.articles.push(article);
    return HttpResponse.json({ article: serializeArticle(article) });
  }),

  http.get(`${API}/articles/:slug`, ({ params }) => {
    const article = findArticle(params.slug as string);
    return article ? HttpResponse.json({ article: serializeArticle(article) }) : notFound();
  }),

  http.put(`${API}/articles/:slug`, async ({ request, params }) => {
    const article = findArticle(params.slug as string);
    if (!article) return notFound();
    const body = (await request.json()) as ArticleBody;
    const invalid = validateArticle(body);
    if (invalid) return invalid;
    Object.assign(article, {
      ...body.article,
      tagList: body.article.tagList.map((t) => t.trim()).filter(Boolean),
      updatedAt: new Date().toISOString(),
    });
    return HttpResponse.json({ article: serializeArticle(article) });
  }),

  http.delete(`${API}/articles/:slug`, ({ params }) => {
    db.articles = db.articles.filter((a) => a.slug !== params.slug);
    return HttpResponse.json({});
  }),

  http.post(`${API}/articles/:slug/favorite`, ({ params }) => {
    const article = findArticle(params.slug as string);
    if (!article) return notFound();
    Object.assign(article, { favorited: true, favoritesCount: article.favoritesCount + 1 });
    return HttpResponse.json({ article: serializeArticle(article) });
  }),

  http.delete(`${API}/articles/:slug/favorite`, ({ params }) => {
    const article = findArticle(params.slug as string);
    if (!article) return notFound();
    Object.assign(article, { favorited: false, favoritesCount: article.favoritesCount - 1 });
    return HttpResponse.json({ article: serializeArticle(article) });
  }),

  http.get(`${API}/articles/:slug/comments`, ({ params }) =>
    HttpResponse.json({
      comments: db.comments.filter((c) => c.article === params.slug).map(serializeComment),
    }),
  ),

  http.post(`${API}/articles/:slug/comments`, async ({ request, params }) => {
    const { comment: attrs } = (await request.json()) as { comment: { body: string } };
    const user = currentUser(request) ?? db.users[0];
    const now = new Date().toISOString();
    const comment = {
      id: db.nextCommentId++,
      body: attrs.body,
      createdAt: now,
      updatedAt: now,
      author: user.username,
      article: params.slug as string,
    };
    db.comments.push(comment);
    return HttpResponse.json({ comment: serializeComment(comment) });
  }),

  http.delete(`${API}/articles/:slug/comments/:id`, ({ params }) => {
    db.comments = db.comments.filter((c) => String(c.id) !== params.id);
    return HttpResponse.json({});
  }),

  http.get(`${API}/tags`, () => HttpResponse.json({ tags: TAGS })),

  http.get(`${API}/profiles/:username`, ({ params }) => {
    const profile = findProfile(params.username as string);
    return profile ? HttpResponse.json({ profile }) : notFound();
  }),

  http.post(`${API}/profiles/:username/follow`, ({ params }) => {
    const profile = findProfile(params.username as string);
    if (!profile) return notFound();
    profile.following = true;
    return HttpResponse.json({ profile });
  }),

  http.delete(`${API}/profiles/:username/follow`, ({ params }) => {
    const profile = findProfile(params.username as string);
    if (!profile) return notFound();
    profile.following = false;
    return HttpResponse.json({ profile });
  }),
];
