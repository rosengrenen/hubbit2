import React from 'react';

import { gql } from '@urql/core';

import { ActiveGroupFragment } from '../../__generated__/graphql';
import { formatNick } from '../../util';

import styles from './ActiveGroupList.module.scss';

export const ACTIVE_GROUP_FRAGMENT = gql`
  fragment ActiveGroup on ActiveSession {
    user {
      cid
      nick
      groups
    }
  }
`;

interface Props {
  sessions: ActiveGroupFragment[];
}

const ActiveGroupList = ({ sessions }: Props) => {
  const groupsMap = new Map<string, ActiveGroupFragment['user'][]>();
  sessions.forEach(session => {
    session.user.groups.forEach(group => {
      const users = groupsMap.get(group);
      const user = {
        ...session.user,
        nick: formatNick(session.user.cid, session.user.nick),
      };
      if (users) {
        groupsMap.set(group, [...users, user]);
      } else {
        groupsMap.set(group, [user]);
      }
    });
  });
  const groups = Array.from(groupsMap)
    .map(([group, users]) => {
      return {
        name: group,
        users: users.sort((left, right) => left.nick.localeCompare(right.nick)),
      };
    })
    .sort((left, right) => left.name.localeCompare(right.name));

  return (
    <div className={styles.activeGroupsContainer}>
      {groups.map(group => (
        <div key={group.name} className={styles.groupBoxContainer}>
          {/*TODO(vidarm): Rewrite without table */}
          <table className="data-table card-shadow">
            <tbody>
              <tr className="header-row" id={group.name}>
                <th>{group.name}</th>
              </tr>
              {group.users.map(user => (
                <tr key={user.cid} className="data-table-row">
                  <td className={styles.userRow}>
                    <a href={`/users/${user.cid}`}>{user.nick}</a>
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

export default ActiveGroupList;
