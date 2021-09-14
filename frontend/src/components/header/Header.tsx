import React from 'react';

import styles from './Header.module.scss';

enum Tab {
  ME,
  IN_HUB,
  ALL_STATS,
  MY_STATS,
}

const Header = () => {
  const activeTab: Tab = getCurrentTab();

  return (
    <header className={styles.hContainer}>
      <h1>
        <a className={styles.title} href="/">
          Who is in the Hubb?
        </a>
      </h1>
      <nav>
        <ul className={styles.menu}>
          <li className={activeTab === Tab.ME ? styles.active : ''}>
            <a href="/">ME</a>
          </li>
          <li className={activeTab === Tab.IN_HUB ? styles.active : ''}>
            <a href="/">SMURFS IN THE HUBB</a>
          </li>
          <li className={activeTab === Tab.ALL_STATS ? styles.active : ''}>
            <a href="/">STATS</a>
          </li>
          <li className={activeTab === Tab.MY_STATS ? styles.active : ''}>
            <a href="/">MY STATS</a>
          </li>
        </ul>
      </nav>
    </header>
  );
};

function getCurrentTab(): Tab {
  const path = window.location.pathname;
  switch (path) {
    case '/':
      return Tab.IN_HUB;
    default:
      return Tab.IN_HUB;
  }
}

export default Header;
