import React from 'react';

import { gql } from '@urql/core';
import { NextPage } from 'next';

import { CurrentSessionsQuery } from '../__generated__/graphql';
import ActiveGroupList, { ACTIVE_GROUP_FRAGMENT } from '../components/active-group-list/ActiveGroupsList';
import ActiveUserList, { ACTIVE_USER_FRAGMENT } from '../components/active-users-list/ActiveUserList';
import Error from '../components/error/Error';
import { defaultGetServerSideProps, PageProps } from '../util';

import styles from './index.module.scss';

const CURRENT_SESSIONS_QUERY = gql`
  query CurrentSessions {
    currentSessions {
      ...ActiveUser
      ...ActiveGroup
    }
  }

  ${ACTIVE_USER_FRAGMENT}
  ${ACTIVE_GROUP_FRAGMENT}
`;

const Home: NextPage<PageProps<CurrentSessionsQuery>> = ({ data }) => {
  if (!data) {
    return <Error />;
  }

  return (
    <div className={styles.sessionsContainer}>
      <ActiveUserList sessions={data.currentSessions} />
      <ActiveGroupList sessions={data.currentSessions} />
    </div>
  );
};

export const getServerSideProps = defaultGetServerSideProps<CurrentSessionsQuery>(CURRENT_SESSIONS_QUERY);

export default Home;
