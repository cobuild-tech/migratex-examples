import type { QueryClient } from '@tanstack/react-query';
import type { Article, ArticleList, Profile } from '../api/types';

// Ember Data's identity map kept every copy of a record in sync. These helpers do the
// same for TanStack Query: patch the record wherever it appears in the cache.

function patchLists(queryClient: QueryClient, patch: (article: Article) => Article) {
  for (const key of [['articles'], ['feed']]) {
    queryClient.setQueriesData<ArticleList>({ queryKey: key }, (data) =>
      data ? { ...data, articles: data.articles.map(patch) } : data,
    );
  }
}

export function updateArticleInCache(queryClient: QueryClient, updated: Article) {
  queryClient.setQueryData(['article', updated.slug], updated);
  patchLists(queryClient, (a) => (a.slug === updated.slug ? updated : a));
}

export function updateProfileInCache(queryClient: QueryClient, updated: Profile) {
  queryClient.setQueryData(['profile', updated.username], updated);
  const patchAuthor = (a: Article) =>
    a.author.username === updated.username ? { ...a, author: { ...a.author, ...updated } } : a;
  patchLists(queryClient, patchAuthor);
  queryClient.setQueriesData<Article>({ queryKey: ['article'] }, (a) => (a ? patchAuthor(a) : a));
}
