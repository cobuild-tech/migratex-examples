export interface Profile {
  username: string;
  bio: string | null;
  image: string | null;
  following: boolean;
}

export interface Article {
  slug: string;
  title: string;
  description: string;
  body: string;
  tagList: string[];
  createdAt: string;
  updatedAt: string;
  favorited: boolean;
  favoritesCount: number;
  author: Profile;
}

export interface ArticleList {
  articles: Article[];
  articlesCount: number;
}

export interface Comment {
  id: number | string;
  body: string;
  createdAt: string;
  updatedAt: string;
  author: Profile;
}

export interface User {
  username: string;
  email: string;
  bio: string | null;
  image: string | null;
  token: string;
}

export interface ArticleInput {
  title: string;
  description: string;
  body: string;
  tagList: string[];
}

export interface UserUpdate {
  username: string;
  email: string;
  bio: string;
  image: string;
  password?: string;
}
