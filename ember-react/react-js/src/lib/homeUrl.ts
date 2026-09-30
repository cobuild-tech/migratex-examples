/** Builds a home-page URL, dropping default params (mirrors Ember's query-param serialization). */
export function homeUrl({ feed, tag, page }: { feed?: string | null; tag?: string | null; page?: number }) {
  const params = new URLSearchParams();
  if (feed) params.set('feed', feed);
  if (tag) params.set('tag', tag);
  if (page && page !== 1) params.set('page', String(page));
  const search = params.toString();
  return search ? `/?${search}` : '/';
}
