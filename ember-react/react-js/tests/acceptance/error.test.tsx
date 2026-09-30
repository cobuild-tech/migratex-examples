import { visit } from '../helpers/app';

describe('Acceptance | error', () => {
  it('displays error page content for invalid URLs', async () => {
    const app = await visit('/some-BODY-once-told-me');
    expect(app.$('[data-test-error-page]')).toBeInTheDocument();
  });
});
