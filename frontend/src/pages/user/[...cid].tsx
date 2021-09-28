import React from 'react';

import { gql } from '@urql/core';
import { GetServerSidePropsContext, NextPage } from 'next';
import { useRouter } from 'next/router';

import { UserStatsQuery } from '../../__generated__/graphql';
import LoadingData from '../../components/loading-data/LoadingData';
import UserStatsCards from '../../components/user-stats-cards/UserStatsCards';
import { defaultGetServerSidePropsWithCallbackInput, PageProps } from '../../util';

import styles from './[...cid].module.scss';

const USER_STATS_QUERY = gql`
  query UserStats($input: UserUniqueInput!) {
    me {
      cid
      nick
    }
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
    }
  }
`;

const UserStats: NextPage<PageProps<UserStatsQuery>> = ({ data }) => {
  const router = useRouter();

  if (!data) {
    return null;
  }

  if (router.query.cid && router.query.cid[0] === 'me') {
    router.replace(`${data.me.cid}`);
  }

  if (data.me.cid !== data.user.cid) {
    return <LoadingData />;
  }

  return (
    <div className={styles.showSection}>
      <h1>{data.me.nick}</h1>
      <div className={styles.showSectionF}>
        <UserStatsCards userStats={data.user} />
      </div>
    </div>
  );
};

function getInputProps(context: GetServerSidePropsContext) {
  const cids = context.query.cid || [];
  const cid = cids?.length > 0 && cids[0] ? cids[0] : '';

  return {
    input: {
      cid: cid,
    },
  };
}

export default UserStats;

export const getServerSideProps = defaultGetServerSidePropsWithCallbackInput<UserStatsQuery>(
  USER_STATS_QUERY,
  getInputProps,
);
