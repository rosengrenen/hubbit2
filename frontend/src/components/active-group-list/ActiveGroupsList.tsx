import React from 'react';

import { CurrentSessionsQuery } from '../../__generated__/graphql';
import { formatNick } from '../../util';

import styles from './ActiveGroupsList.module.scss';

interface props {
  sessions: CurrentSessionsQuery['currentSessions'];
}

interface User {
  cid: string;
  nick: string;
}

const ActiveGroupsList = ({ sessions }: props) => {
  const groupsMap: Map<string, User[]> = new Map<string, User[]>();
  sessions.forEach(session => {
    session.user.groups.forEach(group => {
      let users = groupsMap.get(group);
      const user = {
        nick: formatNick(session.user.cid, session.user.nick),
        cid: session.user.cid,
      };
      if (users) {
        users.push(user);
      } else {
        users = [user];
      }
      groupsMap.set(group, users);
    });
  });

  return (
    <div className={styles.activeGroupsContainer}>
      {Array.from(groupsMap.keys())
        .sort((a, b) => compareGroups(a, b, groupsMap))
        .map(group => (
          <div key={group} className={styles.groupBoxContainer}>
            {/*TODO(vidarm): Rewrite without table */}
            <table key={group} className={'data-table card-shadow '}>
              <tbody>
                <tr className={'header-row'} id={group}>
                  <th>{group}</th>
                </tr>
                {groupsMap
                  .get(group)
                  ?.sort((a, b) => compareUsers(a, b))
                  .map(user => (
                    <tr key={user.cid} className={'data-table-row'}>
                      <td className={styles.userRow}>
                        <a href={`user/${user.cid}`}>{user.nick}</a>
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

function compareGroups(groupNameA: string, groupNameB: string, groupsMap: Map<string, User[]>): number {
  const groupA = groupsMap.get(groupNameA);
  const groupB = groupsMap.get(groupNameB);

  if (!groupB || !groupA) {
    return 0;
  }

  if (groupA.length === groupB.length) {
    return groupNameA.localeCompare(groupNameB);
  }

  return groupB.length - groupA.length;
}

function compareUsers(userA: User, userB: User): number {
  if (userA.nick === userB.nick) {
    return 0;
  }

  return userA.nick.localeCompare(userB.nick);
}

export default ActiveGroupsList;
