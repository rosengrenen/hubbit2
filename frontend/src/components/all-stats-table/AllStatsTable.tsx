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
          <tr key={stat.user.nick} className={'data-table-row'}>
            <td>🐧</td>
            <td>{index + 1}</td>
            <td>
              <a>{stat.user.nick}</a>
            </td>
            <td>{convertMinutesToString(stat.time)}</td>
          </tr>
        );
      })}
    </tbody>
  </table>
);

function convertMinutesToString(totalMinutes: number): string {
  const minutes = Math.floor(totalMinutes % 60);
  const hours = Math.floor((totalMinutes - minutes) / 60);
  return `${numToStr(hours)}:${numToStr(minutes)}`;
}

function numToStr(num: number): string {
  return ('' + num).padStart(2, '0');
}

export default AllStatsTable;
