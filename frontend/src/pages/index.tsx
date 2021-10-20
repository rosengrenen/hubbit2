import React, { useState } from 'react';

import { gql } from '@urql/core';
import { NextPage } from 'next';
import { useSubscription } from 'urql';

import {
  CurrentSessionFragment,
  CurrentSessionsQuery,
  UserJoinSubscription,
  UserLeaveSubscription,
} from '../__generated__/graphql';
import ActiveGroupList, { ACTIVE_GROUP_FRAGMENT } from '../components/active-group-list/ActiveGroupsList';
import ActiveUserList, { ACTIVE_USER_FRAGMENT } from '../components/active-users-list/ActiveUserList';
import Error from '../components/error/Error';
import { defaultGetServerSideProps, PageProps } from '../util';

import styles from './index.module.scss';

const CURRENT_SESSION_FRAGMENT = gql`
  fragment CurrentSession on ActiveSession {
    ...ActiveUser
    ...ActiveGroup
    user {
      id
    }
  }

  ${ACTIVE_USER_FRAGMENT}
  ${ACTIVE_GROUP_FRAGMENT}
`;

const CURRENT_SESSIONS_QUERY = gql`
  query CurrentSessions {
    currentSessions {
      ...CurrentSession
    }
  }

  ${CURRENT_SESSION_FRAGMENT}
`;

const USER_JOIN_SUBSCRIPTION = gql`
  subscription UserJoin {
    userJoin {
      ...CurrentSession
    }
  }

  ${CURRENT_SESSION_FRAGMENT}
`;

const USER_LEAVE_SUBSCRIPTION = gql`
  subscription UserLeave {
    userLeave {
      id
    }
  }
`;

const Home: NextPage<PageProps<CurrentSessionsQuery>> = ({ data }) => {
  const [sessions, setSessions] = useState<CurrentSessionFragment[]>(data?.currentSessions || []);
  useSubscription<UserJoinSubscription>(
    {
      query: USER_JOIN_SUBSCRIPTION,
    },
    (_prev, data) => {
      if (data && !sessions.find(session => session.user.id === data.userJoin.user.id)) {
        setSessions(sessions => [...sessions, data.userJoin]);
      }
      return data;
    },
  );
  useSubscription<UserLeaveSubscription>(
    {
      query: USER_LEAVE_SUBSCRIPTION,
    },
    (_prev, data) => {
      if (data) {
        setSessions(sessions => sessions.filter(session => session.user.id !== data.userLeave.id));
      }
      return data;
    },
  );

  if (!data) {
    return <Error />;
  }

  return (
    <div className={styles.sessionsContainer}>
      <ActiveUserList sessions={sessions} />
      <ActiveGroupList sessions={sessions} />
    </div>
  );
};

export const getServerSideProps = defaultGetServerSideProps<CurrentSessionsQuery>(CURRENT_SESSIONS_QUERY);

export default Home;
