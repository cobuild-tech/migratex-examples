import { marked } from 'marked';
import DOMPurify from 'dompurify';

export function renderMarkdown(body: string | null | undefined): string {
  return DOMPurify.sanitize(marked.parse(body || '', { async: false }));
}
