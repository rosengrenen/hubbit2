const path = require('path');

module.exports = {
  sassOptions: {
    includePaths: [path.join(__dirname, 'styles')],
  },
  rewrites: async () => {
    const proxy = process.env.PROXY;
    if (proxy) {
      return [
        {
          source: '/api/:path*',
          destination: `${proxy}/api/:path*`,
        },
      ];
    } else {
      return [];
    }
  },
};
