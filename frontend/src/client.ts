import { createClient } from 'urql';

export const clientSideClient = createClient({
  url: '/api/graphql',
});

export const serverSideClient = (headers: Record<string, string>) =>
  createClient({
    url: `${process.env.BACKEND_ADDRESS}/api/graphql`,
    fetchOptions: {
      headers,
    },
  });
