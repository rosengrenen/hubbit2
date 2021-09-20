import React from 'react';

import { StatsQuery } from '../../__generated__/graphql';

interface props {
  stats: StatsQuery['stats'];
}

const AllStatsTable = ({ stats }: props) => (
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
        return (
          <tr key={stat.user.cid} className={'data-table-row'}>
            <td>🐧</td>
            <td>{index + 1}</td>
            <td>
              <a>{stat.user.nick}</a>
            </td>
            <td>{convertSecondsToString(stat.durationSeconds)}</td>
          </tr>
        );
      })}
    </tbody>
  </table>
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

export default AllStatsTable;
