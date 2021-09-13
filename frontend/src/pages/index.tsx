import React from 'react';

import ActiveUsersList from '../components/active-users-list/ActiveUsersList';
import { User } from '../types/User';

import styles from './index.module.scss';
import ActiveGroupsList from '../components/active-group-list/ActiveGroupsList';

const mockData: User[] = [
  {
    nick: 'Simpen',
    activeSince: new Date(2021, 8, 12, 20, 0, 0, 111),
    groups: ['sexit', 'prit'],
  },
  {
    nick: 'Hanz',
    activeSince: new Date(2021, 8, 12, 19, 21, 0, 5),
    groups: ['sexit', 'styrit'],
  },
  {
    nick: 'Kaffe',
    activeSince: new Date(2021, 8, 12, 18, 3, 0, 873),
    groups: ['talperson', '8-bit'],
  },
  {
    nick: 'Hoidi',
    activeSince: new Date(2021, 8, 12, 17, 9, 0, 111),
    groups: ['prit', 'styrit'],
  },
  {
    nick: 'Rille',
    activeSince: new Date(2021, 8, 9, 20, 0, 0, 111),
    groups: [],
  },
  {
    nick: 'Champis',
    activeSince: new Date(2021, 8, 9, 20, 0, 0, 111),
    groups: ['nollkit'],
  },
  {
    nick: 'Pang',
    activeSince: new Date(2021, 8, 9, 20, 0, 0, 111),
    groups: ['sexit'],
  },
  {
    nick: 'Snek',
    activeSince: new Date(2021, 8, 9, 20, 0, 0, 111),
    groups: ['hookit'],
  },
  {
    nick: 'SÄPO',
    activeSince: new Date(2021, 8, 9, 20, 0, 0, 111),
    groups: ['drawit'],
  },
  {
    nick: 'Bieber',
    activeSince: new Date(2021, 8, 9, 20, 0, 0, 111),
    groups: ['snit'],
  },
];

const Home = () => {
  return (
    <div className={styles.wrapper}>
      <div className={styles.sessionsContainer}>
        <ActiveUsersList users={mockData} />
        <ActiveGroupsList users={mockData} />
      </div>
    </div>
  );
};

export default Home;
