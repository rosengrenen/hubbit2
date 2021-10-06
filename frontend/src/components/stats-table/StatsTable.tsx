import React from 'react';

import { gql } from '@urql/core';

import { StatsTableFragment } from '../../__generated__/graphql';
import { formatNick } from '../../util';

interface Props {
  stats: StatsTableFragment[];
  myCid: string;
}

export const STATS_TABLE_FRAGMENT = gql`
  fragment StatsTable on Stat {
    currentPosition
    durationSeconds
    prevPosition
    user {
      cid
      nick
    }
  }
`;

const StatsTable = ({ stats, myCid }: Props) => (
  <div>
    <a href={`#${myCid}`}>Find me!</a>
    <table className={'data-table card-shadow'}>
      <thead>
        <tr className={'header-row'}>
          <th>Change</th>
          <th>#</th>
          <th>Name</th>
          <th>Total time</th>
        </tr>
      </thead>
      <tbody>
        {stats.map((stat, index) => {
          const nick = formatNick(stat.user.cid, stat.user.nick);

          return (
            <tr
              key={stat.user.cid}
              id={stat.user.cid}
              className={`data-table-row ${stat.user.cid === myCid ? 'active-row' : ''}`}
            >
              <td>🐧</td>
              <td>{index + 1}</td>
              <td>
                <a>{nick}</a>
              </td>
              <td>{convertSecondsToString(stat.durationSeconds)}</td>
            </tr>
          );
        })}
      </tbody>
    </table>
  </div>
);

function convertSecondsToString(totalSeconds: number): string {
  const seconds = totalSeconds % 60;
  const minutes = Math.floor((totalSeconds / 60) % 60);
  const hours = Math.floor(totalSeconds / 3600);

  return `${numToStr(hours)}:${numToStr(minutes)}:${numToStr(seconds)}`;
}

function numToStr(num: number): string {
  return ('' + num).padStart(2, '0');
}

export default StatsTable;
