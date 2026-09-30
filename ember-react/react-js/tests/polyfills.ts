// React Router's data router builds `new Request(url, { signal })` with jsdom's AbortSignal,
// which Node's undici Request brand-checks and rejects. Tests don't need router aborts, so
// drop the signal.
const NodeRequest = globalThis.Request;
globalThis.Request = class extends NodeRequest {
  constructor(input: RequestInfo | URL, init?: RequestInit) {
    if (init?.signal) {
      const { signal: _signal, ...rest } = init;
      init = rest;
    }
    super(input, init);
  }
} as typeof Request;

export {};
