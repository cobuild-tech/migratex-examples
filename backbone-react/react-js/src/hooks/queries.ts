import { useMutation, useQuery } from '@tanstack/react-query';
import { fetchEvents, fetchRepos, fetchUser } from '../api/github';
import type { Request } from '../state/AppState';

export function useFindUser() {
  return useMutation({ mutationFn: fetchUser });
}

// Each button click is a new request id, so it refetches like the Backbone views,
// which built a fresh collection every time.
export function useEvents(request: Request) {
  return useQuery({
    queryKey: ['events', request.user, request.id],
    queryFn: () => fetchEvents(request.user),
  });
}

export function useRepos(request: Request) {
  return useQuery({
    queryKey: ['repos', request.user, request.id],
    queryFn: () => fetchRepos(request.user),
  });
}
