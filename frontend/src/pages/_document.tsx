import React from 'react';

import Document, { Head, Html, Main, NextScript } from 'next/document';

class HubbitDocument extends Document {
  static async getInitialProps(ctx) {
    const initialProps = await Document.getInitialProps(ctx);
    return { ...initialProps };
  }

  render() {
    return (
      <Html>
        <Head>
          <link
            rel="stylesheet"
            media="all"
            href="//fonts.googleapis.com/css?family=Roboto:400,500,300,700"
            data-turbolinks-track="true"
          />
        </Head>
        <body>
          <Main />
          <NextScript />
        </body>
      </Html>
    );
  }
}

export default HubbitDocument;
