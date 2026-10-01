import type { ReactNode } from 'react';
import { Footer } from './Footer';
import { Header } from './Header';
import { Page } from './Page';

/** The phone-only wrapper the Handlebars templates added under `{{#if isPhone}}`. */
export function PhonePage(props: { id: string; title: string; onBack: () => void; children: ReactNode }) {
  return (
    <Page id={props.id}>
      <Header title={props.title} onBack={props.onBack} />
      <div className="ui-content">{props.children}</div>
      <Footer />
    </Page>
  );
}
