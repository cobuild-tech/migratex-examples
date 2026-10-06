import { setupWorker } from 'msw/browser';
import { handlers } from './handlers';

/** Used by `npm run dev:mock`. Try "London", "Oslo", "Atlantis" (forecast fails) or anything else (not found). */
export const worker = setupWorker(...handlers);
