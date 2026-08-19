import type { ReactNode } from 'react';
import clsx from 'clsx';
import Translate from '@docusaurus/Translate';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
  title: ReactNode;
  description: ReactNode;
};

function useFeatureList(): FeatureItem[] {
  return [
    {
      title: (
        <Translate id="homepage.feature.userStory.title">
          User-story-focused validation
        </Translate>
      ),
      description: (
        <Translate id="homepage.feature.userStory.description">
          Enforce complex, multi-field invariants that keep a domain entity from ever entering an
          invalid state - not just isolated per-field checks.
        </Translate>
      ),
    },
    {
      title: (
        <Translate id="homepage.feature.eventDriven.title">Event-driven lifecycle</Translate>
      ),
      description: (
        <Translate
          id="homepage.feature.eventDriven.description"
          values={{
            onSuccess: <code>onSuccess</code>,
            onFailure: <code>onFailure</code>,
            onDelete: <code>onDelete</code>,
          }}
        >
          {
            'Subscribe to creation, update and deletion lifecycles at both the entity and individual field level with {onSuccess}, {onFailure} and {onDelete} handlers.'
          }
        </Translate>
      ),
    },
    {
      title: (
        <Translate id="homepage.feature.oneModel.title">TypeScript & Rust, one model</Translate>
      ),
      description: (
        <Translate id="homepage.feature.oneModel.description">
          The same constant, dependent, lax, required and virtual field concepts apply in both
          implementations, so switching languages doesn't mean relearning the mental model.
        </Translate>
      ),
    },
  ];
}

function Feature({ title, description }: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className={clsx('padding-horiz--md', styles.feature)}>
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): ReactNode {
  const featureList = useFeatureList();
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {featureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
