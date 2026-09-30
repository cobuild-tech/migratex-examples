export function formatDate(date: string | Date | null | undefined): string {
  if (!date) {
    return '';
  }
  return new Date(date).toLocaleDateString('en-US', {
    month: 'long',
    day: 'numeric',
    year: 'numeric',
  });
}
