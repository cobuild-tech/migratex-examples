// App URLs. Path segments are encoded so usernames/slugs containing `?`, `#`, `/` or `%`
// can't change which route (or API endpoint) a link resolves to.
const seg = (value: string) => encodeURIComponent(value);

export const articlePath = (slug: string) => `/articles/${seg(slug)}`;
export const editorPath = (slug: string) => `/editor/${seg(slug)}`;
export const profilePath = (username: string) => `/profile/${seg(username)}`;
export const profileFavoritesPath = (username: string) => `${profilePath(username)}/favorites`;
