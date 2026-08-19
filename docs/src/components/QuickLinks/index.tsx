import type { ReactNode } from 'react';
import Link from '@docusaurus/Link';
import Translate, { translate } from '@docusaurus/Translate';
import Heading from '@theme/Heading';

const REPO_URL = 'https://github.com/kamtoeddy/ivo';

type Row = {
  language: string;
  docs: string;
  mainDemo: string;
  examples: string;
};

const rows: Row[] = [
  {
    language: 'TypeScript',
    docs: '/docs/ts',
    // ts/examples/ doesn't exist in the repo (unlike rs/examples/) - link to the
    // full worked example in the README instead.
    mainDemo: `${REPO_URL}/blob/main/ts/README.md#defining-a-schema`,
    examples: `${REPO_URL}/tree/main/ts/tests/samples`,
  },
  {
    language: 'Rust',
    docs: '/docs/rs',
    mainDemo: `${REPO_URL}/blob/main/rs/examples/main_demo/src/main.rs`,
    examples: `${REPO_URL}/tree/main/rs/examples`,
  },
];

export default function QuickLinks(): ReactNode {
  return (
    <section>
      <div className="container">
        <Heading as="h2">
          <Translate id="homepage.quickLinks.heading">Quick links</Translate>
        </Heading>
        <table>
          <thead>
            <tr>
              <th>
                <Translate id="homepage.quickLinks.language">Language</Translate>
              </th>
              <th>
                <Translate id="homepage.quickLinks.docs">Docs</Translate>
              </th>
              <th>
                <Translate id="homepage.quickLinks.mainDemo">Main demo</Translate>
              </th>
              <th>
                <Translate id="homepage.quickLinks.examples">Examples</Translate>
              </th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row) => (
              <tr key={row.language}>
                <td>{row.language}</td>
                <td>
                  <Link to={row.docs}>
                    {translate({ id: 'homepage.quickLinks.docsLink', message: 'docs' })}
                  </Link>
                </td>
                <td>
                  <Link to={row.mainDemo}>
                    {translate({ id: 'homepage.quickLinks.demoLink', message: 'demo' })}
                  </Link>
                </td>
                <td>
                  <Link to={row.examples}>
                    {translate({ id: 'homepage.quickLinks.examplesLink', message: 'examples' })}
                  </Link>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  );
}
