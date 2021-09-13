import React from 'react';

import { CurrentSessionsQuery } from '../../__generated__/graphql';

import styles from './ActiveGroupsList.module.scss';

interface props {
  sessions: CurrentSessionsQuery['currentSessions'];
}

const ActiveGroupsList = ({ sessions }: props) => {
  const groupsMap: Map<string, string[]> = new Map<string, string[]>();
  sessions.forEach(session => {
    session.user.groups.forEach(group => {
      let users = groupsMap.get(group);
      if (users) {
        users.push(session.user.nick);
      } else {
        users = [session.user.nick];
      }
      groupsMap.set(group, users);
    });
  });

  return (
    <div className={styles.activeGroupsContainer}>
      {Array.from(groupsMap.keys()).map(group => (
        <div key={group} className={styles.groupBoxContainer}>
          {/*TODO(vidarm): Rewrite without table */}
          <table key={group} className={'data-table card-shadow '}>
            <tbody>
              <tr className={'header-row'} id={group}>
                <th>{group}</th>
              </tr>
              {groupsMap.get(group).map(user => (
                <tr key={user}>
                  <td className={styles.userRow}>
                    <a href={'google.com'}>{user}</a>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      ))}
    </div>
  );
};

export default ActiveGroupsList;
