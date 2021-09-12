import '../global-styles/styles.scss';
import '../global-styles/tables.scss';

import React from 'react';

import Footer from '../components/footer/Footer';
import Header from '../components/header/Header';

function HubbitApp({ Component, pageProps }) {
  return (
    <div className={'pageWrapper'}>
      <Header />
      <div className={'componentWrapper'}>
        <Component {...pageProps} />
      </div>
      <Footer />
    </div>
  );
}

export default HubbitApp;
