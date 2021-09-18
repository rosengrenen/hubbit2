import React from 'react';

import { gql } from '@urql/core';
import { NextPage } from 'next';

import { CurrentSessionsQuery } from '../__generated__/graphql';
import ActiveGroupsList from '../components/active-group-list/ActiveGroupsList';
import ActiveUsersList from '../components/active-users-list/ActiveUsersList';
import { defaultGetServerSideProps, PageProps } from '../util';

import styles from './index.module.scss';

const CURRENT_SESSIONS_QUERY = gql`
  query CurrentSessions {
    currentSessions {
      user {
        id
        nick
        avatarUrl
        groups
      }
      startTime
    }
  }
`;

const Home: NextPage<PageProps<CurrentSessionsQuery>> = ({ data }) => {
  if (!data) {
    return null;
  }

  return (
    <div className={styles.sessionsContainer}>
      <ActiveUsersList sessions={data.currentSessions} />
      <ActiveGroupsList sessions={data.currentSessions} />
    </div>
  );
};

export const getServerSideProps = defaultGetServerSideProps<CurrentSessionsQuery>(CURRENT_SESSIONS_QUERY);

export default Home;
