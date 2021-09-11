import React from 'react';

import styles from './ActiveUsersList.module.scss';

const mockData = {};

const ActiveUsersList = () => {
  return (
    <div className={styles.activeSmurfsWrapper}>
      There are {1} smurfs in the Hubb right now!
      <table className={'data-table card-shadow ' + styles.activeSmurfsTable}>
        <thead>
          <tr className={'header-row'}>
            <th className={styles.userRow}>User</th>
            <th className={styles.statusRow}>Current Status</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td className={styles.userRow}>
              <a>Spurt</a>
            </td>
            <td className={styles.timeCell}>
              {'Since 12:17 '}
              <time>(4 hours)</time>
            </td>
          </tr>
          <tr>
            <td className={styles.userRow}>
              <a>Hanz</a>
            </td>
            <td className={styles.timeCell}>
              {'Since 12:17 '}
              <time>(4 hours)</time>
            </td>
          </tr>
          <tr>
            <td className={styles.userRow}>
              <a>Steget</a>
            </td>
            <td className={styles.timeCell}>
              {'Since 12:17 '}
              <time>(4 hours)</time>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  );
};

export default ActiveUsersList;
