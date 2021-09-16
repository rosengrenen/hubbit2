import React from 'react';

import Document, { Head, Html, Main, NextScript } from 'next/document';

class HubbitDocument extends Document {
  render() {
    return (
      <Html>
        <Head>
          <link
            rel="stylesheet"
            media="all"
            href="https://fonts.googleapis.com/css?family=Roboto:400,500,300,700"
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
