/* eslint-disable no-console */
const next = require('next');
const { createProxyMiddleware } = require('http-proxy-middleware');
const express = require('express');

const port = process.env.PORT || 3000;
const dev = process.env.NODE_ENV !== 'production';
const app = next({ dev });
const handle = app.getRequestHandler();

app.prepare().then(() => {
  const app = express();

  app.use(
    '/api',
    createProxyMiddleware({
      target: process.env.BACKEND_ADDRESS,
      pathRewrite: {
        '^/api': '/api',
      },
      ws: true,
    }),
  );

  app.use((req, res) => {
    return handle(req, res);
  });

  app.listen(port, err => {
    if (err) throw err;
    console.log(`> Ready on http://localhost:${port}`);
  });
});
