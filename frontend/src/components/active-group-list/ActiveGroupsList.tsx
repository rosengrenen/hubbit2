import React from 'react';

import { User } from '../../types/User';

import styles from './ActiveGroupsList.module.scss';

interface props {
  users: User[];
}

const ActiveGroupsList = ({ users }: props) => {
  const groupsMap: Map<string, string[]> = new Map<string, string[]>();
  users.forEach(user => {
    user.groups.forEach(group => {
      let users = groupsMap.get(group);
      if (users) {
        users.push(user.nick);
      } else {
        users = [user.nick];
      }
      groupsMap.set(group, users);
    });
  });

  return (
    <div className={styles.activeGroupsContainer}>
      {Array.from(groupsMap.keys()).map(group => (
        <div key={group} className={styles.groupBoxContainer}>
          <table key={group} className={'data-table card-shadow '}>
            <tbody>
              <tr className={'header-row'} id={group}>
                <th>{group}</th>
              </tr>
              {groupsMap.get(group).map(user => (
                <tr key={user}>
                  <td className={'userRow'}>
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
