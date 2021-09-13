import React from 'react';

import { useQuery } from 'urql';

import ActiveGroupsList from '../components/active-group-list/ActiveGroupsList';
import ActiveUsersList from '../components/active-users-list/ActiveUsersList';
import { ActiveSessions, getCurrentSessions } from '../queries/getActiveSessions';

import styles from './index.module.scss';

const Home = () => {
  const [session] = useQuery<ActiveSessions>({
    query: getCurrentSessions,
  });

  if (session.fetching) {
    return <div>Loading...</div>;
  }

  if (session.error) {
    console.error('Error:', session.error);
    return <div style={{ color: 'red' }}>ERROR</div>;
  }

  console.log('DATA, ', session.data.currentSessions);

  return (
    <div className={styles.wrapper}>
      <div className={styles.sessionsContainer}>
        <ActiveUsersList sessions={session.data.currentSessions} />
        <ActiveGroupsList sessions={session.data.currentSessions} />
      </div>
    </div>
  );
};

export default Home;
