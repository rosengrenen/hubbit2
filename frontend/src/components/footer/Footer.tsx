import React from 'react';

import styles from './Footer.module.scss';

const Footer = () => (
  <footer className={styles.footer}>
    <div className={styles.divider} />
    <div className={styles.footerText}>
      <span>
        Created with 💙 by <a href="https://github.com/rosengrenen">🌹 Rosen 🌹</a> &{' '}
        <a href="https://github.com/viddem">✌ Vidde ✌</a>
      </span>
    </div>
  </footer>
);

export default Footer;
