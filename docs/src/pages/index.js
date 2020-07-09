import React from "react";
import clsx from "clsx";
import Layout from "@theme/Layout";
import Link from "@docusaurus/Link";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import useBaseUrl from "@docusaurus/useBaseUrl";
import styles from "./styles.module.css";

const features = [
  {
    title: <>UBL knowledge not required</>,
    imageUrl: "img/knowledge.png",
    description: (
      <>
        <code>XBuilder</code> do not require you to know the <code>UBL</code>{" "}
        details, it exposes a set of <code>POJOs</code> that are, internally,
        transpiled into XML files that follows the <code>UBL</code> standards.
      </>
    ),
  },
  {
    title: <>Math operations</>,
    imageUrl: "img/math.png",
    description: (
      <>
        <code>XBuilder</code> executes all math operations required to fill
        certain values in the <code>XML</code> files. Math operations like
        taxes, total amounts, discounts, etc. must be executed internally.
      </>
    ),
  },
  {
    title: <>Requires min data</>,
    imageUrl: "img/mindata.png",
    description: (
      <>
        <code>XBuilder</code> fills all missing data with default values.{" "}
        <code>XBuilder</code>
        requires only minimal data.
      </>
    ),
  },
];

function Feature({ imageUrl, title, description }) {
  const imgUrl = useBaseUrl(imageUrl);
  return (
    <div className={clsx("col col--4", styles.feature)}>
      {imgUrl && (
        <div className="text--center">
          <img className={styles.featureImage} src={imgUrl} alt={title} />
        </div>
      )}
      <h3>{title}</h3>
      <p>{description}</p>
    </div>
  );
}

function Home() {
  const context = useDocusaurusContext();
  const { siteConfig = {} } = context;
  return (
    <Layout
      title={`${siteConfig.title}`}
      description="Java library for creating and signing XML files based on Universal Bussiness Language (UBL) <head />"
    >
      <header className={clsx("hero hero--primary", styles.heroBanner)}>
        <div className="container">
          <h1 className="hero__title">{siteConfig.title}</h1>
          <p className="hero__subtitle">{siteConfig.tagline}</p>
          <div className={styles.buttons}>
            <Link
              className={clsx(
                "button button--outline button--secondary button--lg",
                styles.getStarted
              )}
              to={useBaseUrl("docs/")}
            >
              Get Started
            </Link>
          </div>
        </div>
      </header>
      <main>
        {features && features.length > 0 && (
          <section className={styles.features}>
            <div className="container">
              <div className="row">
                {features.map((props, idx) => (
                  <Feature key={idx} {...props} />
                ))}
              </div>
            </div>
          </section>
        )}
      </main>
    </Layout>
  );
}

export default Home;
