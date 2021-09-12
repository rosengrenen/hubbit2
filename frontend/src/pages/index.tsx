import React from 'react';

import ActiveUsersList from '../components/active-users-list/ActiveUsersList';

import styles from './index.module.scss';

const Home = () => {
  return (
    <div className={styles.wrapper}>
      <div className={styles.sessionsContainer}>
        <ActiveUsersList />
      </div>
    </div>
  );
};

export default Home;
