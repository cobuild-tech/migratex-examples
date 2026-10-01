import { setupWorker } from 'msw/browser';
import { handlers } from './handlers';

/** Used by `npm run dev:mock`. Search for "octocat". */
export const worker = setupWorker(...handlers);
