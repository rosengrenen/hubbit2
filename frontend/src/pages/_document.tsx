import React from 'react';

import Document, { Head, Html, Main, NextScript } from 'next/document';

class HubbitDocument extends Document {
  render() {
    return (
      <Html>
        <Head>
          <link rel="shortcut icon" type="image/svg" href="/hubbit_eye_logo.svg" />
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
