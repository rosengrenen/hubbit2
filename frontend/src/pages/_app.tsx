import '../global-styles/styles.scss';
import '../global-styles/tables.scss';
import '../global-styles/groups.scss';
import '../global-styles/roboto-font.css';

import React from 'react';

import { AppProps } from 'next/app';
import { Provider } from 'urql';

import { createClient } from '../client';
import Footer from '../components/footer/Footer';
import Header from '../components/header/Header';

function HubbitApp({ Component, pageProps }: AppProps) {
  return (
    <Provider value={createClient()}>
      <div className="pageWrapper">
        <Header />
        <div className="componentWrapper">
          <div className="wrapper">
            <Component {...pageProps} />
          </div>
        </div>
        <Footer />
      </div>
    </Provider>
  );
}

export default HubbitApp;
