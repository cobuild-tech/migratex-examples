import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import * as api from '../api/endpoints';
import type { Article, ArticleInput, Comment, Profile } from '../api/types';
import { updateArticleInCache, updateProfileInCache } from '../lib/cacheUpdates';

export function useArticles(params: { tag?: string | null; author?: string; favorited?: string; page?: number }, enabled = true) {
  return useQuery({
    queryKey: ['articles', params],
    queryFn: () => api.listArticles(params),
    enabled,
  });
}

export function useFeed(page: number, enabled = true) {
  return useQuery({ queryKey: ['feed', page], queryFn: () => api.feedArticles(page), enabled });
}

export function useArticle(slug: string) {
  return useQuery({ queryKey: ['article', slug], queryFn: () => api.getArticle(slug) });
}

export function useProfile(username: string) {
  return useQuery({ queryKey: ['profile', username], queryFn: () => api.getProfile(username) });
}

export function useTags() {
  return useQuery({ queryKey: ['tags'], queryFn: api.listTags });
}

export function useComments(slug: string) {
  return useQuery({ queryKey: ['comments', slug], queryFn: () => api.listComments(slug) });
}

export function useFavorite() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (article: Article) => api.setFavorite(article.slug, !article.favorited),
    onSuccess: (updated) => updateArticleInCache(queryClient, updated),
  });
}

export function useFollow() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (profile: Profile) => api.setFollow(profile.username, !profile.following),
    onSuccess: (updated) => updateProfileInCache(queryClient, updated),
  });
}

export function useSaveArticle(slug?: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: ArticleInput) => (slug ? api.updateArticle(slug, input) : api.createArticle(input)),
    onSuccess: (article) => {
      if (slug && article.slug !== slug) {
        // The API re-slugs on title change: drop entries under the old slug.
        queryClient.removeQueries({ queryKey: ['article', slug] });
        queryClient.removeQueries({ queryKey: ['comments', slug] });
      }
      updateArticleInCache(queryClient, article);
      queryClient.invalidateQueries({ queryKey: ['articles'] });
      queryClient.invalidateQueries({ queryKey: ['feed'] });
    },
  });
}

export function useDeleteArticle() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (slug: string) => api.deleteArticle(slug),
    onSuccess: (_, slug) => {
      queryClient.removeQueries({ queryKey: ['article', slug] });
      queryClient.invalidateQueries({ queryKey: ['articles'] });
      queryClient.invalidateQueries({ queryKey: ['feed'] });
    },
  });
}

export function useAddComment(slug: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (body: string) => api.createComment(slug, body),
    onSuccess: (comment) =>
      queryClient.setQueryData<Comment[]>(['comments', slug], (list = []) => [...list, comment]),
  });
}

export function useDeleteComment(slug: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: Comment['id']) => api.deleteComment(slug, id),
    onSuccess: (_, id) =>
      queryClient.setQueryData<Comment[]>(['comments', slug], (list = []) => list.filter((c) => c.id !== id)),
  });
}
