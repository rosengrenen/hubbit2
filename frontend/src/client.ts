import { createClient } from 'urql';

export const clientSideClient = createClient({
  url: '/api/graphql',
});

export const serverSideClient = (headers: Record<string, string>) =>
  createClient({
    url: 'http://localhost:8080/api/graphql',
    fetchOptions: {
      headers,
    },
  });
