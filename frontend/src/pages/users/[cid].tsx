import React from 'react';

import { gql } from '@urql/core';
import { NextPage } from 'next';

import { MeCidQueryQuery, UserStatsQuery, UserStatsQueryVariables } from '../../__generated__/graphql';
import Error from '../../components/error/Error';
import UserStatsCards from '../../components/user-stats-cards/UserStatsCards';
import { defaultGetServerSideProps, PageProps } from '../../util';

import styles from './[cid].module.scss';

const USER_STATS_QUERY = gql`
  query UserStats($input: UserUniqueInput!) {
    user(input: $input) {
      longestSession {
        startTime
        endTime
      }
      recentSessions {
        startTime
        endTime
      }
      hourStats
      cid
      nick
      totalTimeSeconds
      longestSession {
        startTime
        endTime
      }
    }
  }
`;

const ME_CID_QUERY = gql`
  query MeCidQuery {
    me {
      cid
    }
  }
`;

const UserStats: NextPage<PageProps<UserStatsQuery>> = ({ data }) => {
  if (!data) {
    return <Error />;
  }

  return (
    <div className={styles.showSection}>
      <h1>{data.user.nick}</h1>
      <div className={styles.showSectionF}>
        <UserStatsCards userStats={data.user} />
      </div>
    </div>
  );
};

export default UserStats;

type Params = {
  cid: string;
};

export const getServerSideProps = defaultGetServerSideProps<UserStatsQuery, UserStatsQueryVariables, Params>(
  USER_STATS_QUERY,
  context => {
    const cid = context.params?.cid || '';
    return {
      input: {
        cid,
      },
    };
  },
  async (params, client) => {
    if (params.cid === 'me') {
      const { data } = await client.query<MeCidQueryQuery>(ME_CID_QUERY).toPromise();
      if (data) {
        return {
          destination: `/users/${data.me.cid}`,
          permanent: false,
        };
      }
    }

    return;
  },
);
