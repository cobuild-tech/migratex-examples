import { faker } from '@faker-js/faker';
import type { Article, Comment, Profile, User } from '../../src/api/types';

// In-memory backend state, a port of the Mirage models/factories in ember-webapp/mirage.

export const TAGS = ['emberjs', 'tomster', 'wycats', 'tomdale', 'ember-cli', 'training', 'dragons'];

type DbUser = User & { password?: string };
type DbArticle = Omit<Article, 'author'> & { author: string };
type DbComment = Omit<Comment, 'author'> & { author: string; article: string };

export const db = {
  users: [] as DbUser[],
  profiles: [] as Profile[],
  articles: [] as DbArticle[],
  comments: [] as DbComment[],
  nextCommentId: 1,
};

export function resetDb() {
  db.users = [];
  db.profiles = [];
  db.articles = [];
  db.comments = [];
  db.nextCommentId = 1;
}

let sequence = 0;
const unique = (value: string) => `${value}${++sequence}`;

export function createProfile(attrs: Partial<Profile> = {}): Profile {
  const profile: Profile = {
    username: unique(faker.internet.username().replace(/\W/g, '')),
    bio: faker.lorem.sentence(),
    image: faker.image.avatar(),
    following: faker.datatype.boolean(),
    ...attrs,
  };
  db.profiles.push(profile);
  return profile;
}

export function createUser(attrs: Partial<DbUser> = {}): DbUser {
  const user: DbUser = {
    token: 'auth-token',
    image: null,
    email: faker.internet.email(),
    username: unique(faker.internet.username().replace(/\W/g, '')),
    bio: faker.lorem.paragraph(),
    ...attrs,
  };
  db.users.push(user);
  createProfile({ username: user.username, bio: user.bio, image: user.image, following: false });
  return user;
}

export function createArticle(attrs: Partial<Omit<DbArticle, 'author'>> & { author?: Profile } = {}): DbArticle {
  const { author = createProfile(), ...rest } = attrs;
  const title = rest.title ?? faker.lorem.words();
  const article: DbArticle = {
    title,
    description: faker.lorem.paragraphs(),
    body: faker.lorem.paragraphs(),
    tagList: faker.datatype.boolean() ? [faker.helpers.arrayElement(TAGS), faker.helpers.arrayElement(TAGS)] : [],
    createdAt: faker.date.recent().toISOString(),
    updatedAt: faker.date.recent().toISOString(),
    favorited: faker.datatype.boolean(),
    favoritesCount: faker.number.int(100),
    slug: unique(faker.helpers.slugify(title)),
    ...rest,
    author: author.username,
  };
  db.articles.push(article);
  return article;
}

export function createArticles(count: number, attrs: Parameters<typeof createArticle>[0] = {}) {
  return Array.from({ length: count }, () => createArticle(attrs));
}

export function createComment(article: DbArticle, author: Profile, attrs: Partial<DbComment> = {}): DbComment {
  const comment: DbComment = {
    id: db.nextCommentId++,
    body: faker.lorem.paragraphs(),
    createdAt: faker.date.recent().toISOString(),
    updatedAt: faker.date.recent().toISOString(),
    ...attrs,
    author: author.username,
    article: article.slug,
  };
  db.comments.push(comment);
  return comment;
}

export const findProfile = (username: string) => db.profiles.find((p) => p.username === username);
export const findArticle = (slug: string) => db.articles.find((a) => a.slug === slug);

export const serializeArticle = (article: DbArticle): Article => ({
  ...article,
  author: { ...findProfile(article.author)! },
});

export const serializeComment = ({ article: _article, ...comment }: DbComment): Comment => ({
  ...comment,
  author: { ...findProfile(comment.author)! },
});

export const serializeUser = ({ password: _password, ...user }: DbUser): User => ({ ...user });
