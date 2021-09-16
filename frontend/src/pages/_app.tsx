import '../global-styles/styles.scss';
import '../global-styles/tables.scss';
import '../global-styles/groups.scss';

import React from 'react';

import { AppProps } from 'next/app';

import Footer from '../components/footer/Footer';
import Header from '../components/header/Header';

function HubbitApp({ Component, pageProps }: AppProps) {
  return (
    <div className="pageWrapper">
      <Header />
      <div className="componentWrapper">
        <div className="wrapper">
          <Component {...pageProps} />
        </div>
      </div>
      <Footer />
    </div>
  );
}

export default HubbitApp;
