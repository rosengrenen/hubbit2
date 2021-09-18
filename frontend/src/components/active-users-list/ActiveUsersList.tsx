import React from 'react';

import { CurrentSessionsQuery } from '../../__generated__/graphql';

import styles from './ActiveUsersList.module.scss';

interface props {
  sessions: CurrentSessionsQuery['currentSessions'];
}

const ActiveUsersList = ({ sessions }: props) => {
  const currTime: Date = new Date(Date.now());

  return (
    <div className={styles.activeSmurfsWrapper}>
      <div>
        There are {sessions.length} smurfs in the Hubb right now!
        <table className={'data-table card-shadow ' + styles.activeSmurfsTable}>
          <thead>
            <tr className={'header-row'}>
              <th className={styles.userRow}>User</th>
              <th className={styles.statusRow}>Current Status</th>
            </tr>
          </thead>
          <tbody>
            {sessions.map(session => {
              const startTime = new Date(session.startTime);

              return (
                <tr key={session.user.nick} className={'data-table-row'}>
                  <td className={styles.userRow}>
                    <a href={'google.com'}>{session.user.nick}</a>
                  </td>
                  <td className={styles.timeCell}>
                    {formatTime(startTime)}
                    <time>{getHoursDiff(startTime, currTime)}</time>
                  </td>
                </tr>
              );
            })}
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
  const diffTime = Math.abs(a.getTime() - b.getTime());
  const diffHours = Math.round(diffTime / oneHour);
  if (diffHours >= 1) {
    return `(${diffHours} hours)`;
  }

  return `(${Math.round(diffTime / oneMinute)} minutes)`;
}

export default ActiveUsersList;
