import React from 'react';

import { User } from '../../types/User';

import styles from './ActiveUsersList.module.scss';

interface props {
  users: User[];
}

const ActiveUsersList = ({ users }: props) => {
  const currTime: Date = new Date(Date.now());

  return (
    <div className={styles.activeSmurfsWrapper}>
      <div>
        There are {1} smurfs in the Hubb right now!
        <table className={'data-table card-shadow ' + styles.activeSmurfsTable}>
          <thead>
            <tr className={'header-row'}>
              <th className={styles.userRow}>User</th>
              <th className={styles.statusRow}>Current Status</th>
            </tr>
          </thead>
          <tbody>
            {users.map(user => (
              <tr key={user.nick}>
                <td className={styles.userRow}>
                  <a>{user.nick}</a>
                </td>
                <td className={styles.timeCell}>
                  {formatTime(user.activeSince)}
                  <time>{getHoursDiff(user.activeSince, currTime)}</time>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

function formatTime(time: Date): string {
  return `Since ${String(time.getHours()).padStart(2, '0')}:${String(time.getMinutes()).padStart(2, '0')} `;
}

const oneHour = 1000 * 60 * 60;
const oneMinute = 1000 * 60;
function getHoursDiff(a: Date, b: Date): string {
  console.log('Date a: ', a, ' | b: ', b);
  const diffTime = Math.abs(a.getTime() - b.getTime());
  console.log('diff: ', diffTime);
  const diffHours = Math.round(diffTime / oneHour);
  if (diffHours >= 1) {
    return `(${diffHours} hours)`;
  }

  return `(${Math.round(diffTime / oneMinute)} minutes)`;
}

export default ActiveUsersList;
