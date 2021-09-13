import React from 'react';

import { useCurrentSessionsQuery } from '../__generated__/graphql';
import ActiveGroupsList from '../components/active-group-list/ActiveGroupsList';
import ActiveUsersList from '../components/active-users-list/ActiveUsersList';

import styles from './index.module.scss';

const Home = () => {
  const [{ data, fetching, error }] = useCurrentSessionsQuery();

  if (fetching) {
    return <div>Loading...</div>;
  }

  if (error) {
    return <div>Error...</div>;
  }

  return (
    <div className={styles.wrapper}>
      <div className={styles.sessionsContainer}>
        <ActiveUsersList sessions={data.currentSessions} />
        <ActiveGroupsList sessions={data.currentSessions} />
      </div>
    </div>
  );
};

export default Home;
