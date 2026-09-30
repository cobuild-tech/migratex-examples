import { apiFetch } from './client';
import type { Article, ArticleInput, ArticleList, Comment, Profile, User, UserUpdate } from './types';

export const PER_PAGE = 10;

const seg = (value: string) => encodeURIComponent(value);

const offsetFor = (page: number) => (page - 1) * PER_PAGE;

function query(params: Record<string, string | number | undefined | null>) {
  const search = new URLSearchParams();
  for (const [key, value] of Object.entries(params)) {
    if (value !== undefined && value !== null && value !== '') {
      search.set(key, String(value));
    }
  }
  return search.toString();
}

// Articles
export const listArticles = (params: { tag?: string | null; author?: string; favorited?: string; page?: number }) =>
  apiFetch<ArticleList>(
    `/articles?${query({
      limit: PER_PAGE,
      offset: offsetFor(params.page ?? 1),
      tag: params.tag,
      author: params.author,
      favorited: params.favorited,
    })}`,
  );

export const feedArticles = (page: number) =>
  apiFetch<ArticleList>(`/articles/feed?${query({ limit: PER_PAGE, offset: offsetFor(page) })}`);

export const getArticle = (slug: string) =>
  apiFetch<{ article: Article }>(`/articles/${seg(slug)}`).then((r) => r.article);

export const createArticle = (article: ArticleInput) =>
  apiFetch<{ article: Article }>('/articles', { method: 'POST', body: { article } }).then((r) => r.article);

export const updateArticle = (slug: string, article: ArticleInput) =>
  apiFetch<{ article: Article }>(`/articles/${seg(slug)}`, { method: 'PUT', body: { article } }).then(
    (r) => r.article,
  );

export const deleteArticle = (slug: string) => apiFetch<unknown>(`/articles/${seg(slug)}`, { method: 'DELETE' });

export const setFavorite = (slug: string, favorite: boolean) =>
  apiFetch<{ article: Article }>(`/articles/${seg(slug)}/favorite`, {
    method: favorite ? 'POST' : 'DELETE',
  }).then((r) => r.article);

// Comments
export const listComments = (slug: string) =>
  apiFetch<{ comments: Comment[] }>(`/articles/${seg(slug)}/comments`).then((r) => r.comments);

export const createComment = (slug: string, body: string) =>
  apiFetch<{ comment: Comment }>(`/articles/${seg(slug)}/comments`, {
    method: 'POST',
    body: { comment: { body } },
  }).then((r) => r.comment);

export const deleteComment = (slug: string, id: Comment['id']) =>
  apiFetch<unknown>(`/articles/${seg(slug)}/comments/${seg(String(id))}`, { method: 'DELETE' });

// Profiles
export const getProfile = (username: string) =>
  apiFetch<{ profile: Profile }>(`/profiles/${seg(username)}`).then((r) => r.profile);

export const setFollow = (username: string, follow: boolean) =>
  apiFetch<{ profile: Profile }>(`/profiles/${seg(username)}/follow`, {
    method: follow ? 'POST' : 'DELETE',
  }).then((r) => r.profile);

// Tags
export const listTags = () => apiFetch<{ tags: string[] }>('/tags').then((r) => r.tags);

// User
export const getCurrentUser = () => apiFetch<{ user: User }>('/user').then((r) => r.user);

export const login = (email: string, password: string) =>
  apiFetch<{ user: User }>('/users/login', { method: 'POST', body: { user: { email, password } } }).then(
    (r) => r.user,
  );

export const register = (username: string, email: string, password: string) =>
  apiFetch<{ user: User }>('/users', {
    method: 'POST',
    body: { user: { username, email, password } },
  }).then((r) => r.user);

export const updateUser = (user: UserUpdate) =>
  apiFetch<{ user: User }>('/user', { method: 'PUT', body: { user } }).then((r) => r.user);
