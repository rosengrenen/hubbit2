import { Client, CombinedError } from '@urql/core';
import { DocumentNode } from 'graphql';
import { GetServerSideProps, GetServerSidePropsContext, Redirect } from 'next';
import { ParsedUrlQuery } from 'querystring';

import { serverSideClient } from './client';

export const isServerSide = typeof window === 'undefined';

export const rawHeadersToDict = (rawHeaders: string[]): Record<string, string> => {
  const headers: Record<string, string> = rawHeaders.reduce((prev, curr, i, arr) => {
    if (i + 1 === rawHeaders.length) {
      return prev;
    }

    if (i % 2 === 0) {
      return {
        ...prev,
        [curr]: arr[i + 1],
      };
    }

    return prev;
  }, {});

  return headers;
};

export enum GqlError {
  NOT_LOGGED_IN,
  OTHER,
}

export const parseGqlError = (error: CombinedError): GqlError => {
  if (error) {
    for (const gqlError of error.graphQLErrors) {
      if (gqlError.extensions && gqlError.extensions['code'] === 'NOT_LOGGED_IN') {
        return GqlError.NOT_LOGGED_IN;
      }
    }
  }

  return GqlError.OTHER;
};

export const authRedirect = (path: string): Redirect => ({
  destination: `/api/auth/gamma/login?from=${path}`,
  permanent: false,
});

export interface PageProps<T> {
  data: T | null;
}

export const defaultGetServerSideProps = <
  Result,
  // eslint-disable-next-line @typescript-eslint/ban-types
  Variables extends object = {},
  Params extends ParsedUrlQuery = ParsedUrlQuery,
>(
  query: DocumentNode,
  inputCallback?: (context: GetServerSidePropsContext<Params>) => Variables,
  preDataHook?: (params: Params, client: Client) => Promise<Redirect | void>,
) => {
  const getServerSideProps: GetServerSideProps<PageProps<Result>, Params> = async context => {
    const headers = rawHeadersToDict(context.req.rawHeaders);
    const client = serverSideClient(headers);

    if (preDataHook) {
      const redirect = await preDataHook(context.params as Params, client);
      if (redirect) {
        return {
          redirect,
        };
      }
    }
    const variables = (inputCallback && inputCallback(context)) || undefined;

    const { data, error } = await client.query<Result, Variables>(query, variables).toPromise();

    let redirect: Redirect | undefined = undefined;
    if (error) {
      switch (parseGqlError(error)) {
        case GqlError.NOT_LOGGED_IN:
          redirect = authRedirect(context.resolvedUrl);
          break;
        default:
          console.log(error);
      }
    }

    return {
      props: {
        data: data ? data : null,
      },
      redirect,
    };
  };

  return getServerSideProps;
};

export const formatNick = (cid: string, nick: string) => {
  switch (cid) {
    // DON'T QUESTION THIS!
    case 'mvidar':
      return `✌ ${nick} ✌`;
    case 'rasros':
      return `🌹 ${nick} 🌹`;
    case 'dahida':
      return `💤 ${nick} 💤`;
    case 'hulthe':
      return `🎩${nick}🪄`;
    case 'jenhallb':
      return `❤ ${nick} ❤`;
    case 'erijohns':
      return `🍔${nick}🦙`;
    case 'caeric':
      return `Loppan :dab:`;
    case 'lahtig':
      return `🥛(☕)🥛 ${nick}`;
    default:
      return nick;
  }
};
