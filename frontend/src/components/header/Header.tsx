import React from 'react';

import styles from './Header.module.scss';

const Header = () => {
  return (
    <header>
      <h1 className={styles.hContainer}>
        <a className={styles.title}>Who is in the Hubb?</a>
      </h1>
    </header>
  );
};

export default Header;
