import React, { useEffect, useState } from 'react';

import Link from 'next/link';
import { useRouter } from 'next/router';

import { useAllTimeSessionsQuery } from '../../__generated__/graphql';
import AllStatsTable from '../../components/all-stats-table/AllStatsTable';
import Error from '../../components/error/Error';
import LoadingData from '../../components/loading-data/LoadingData';

import styles from './index.module.scss';

const ALL_TIME = 'all_time';
const STUDY_YEAR = 'study_year';
const STUDY_PERIOD = 'study_period';
const MONTHLY = 'monthly';
const WEEKLY = 'weekly';
const DAILY = 'daily';

const AllStats = () => {
  const [{ data, fetching, error }] = useAllTimeSessionsQuery();

  const { pathname, query } = useRouter();
  const timeFrame = query['timeframe'];

  let activeFrame = STUDY_YEAR;
  switch (timeFrame) {
    case ALL_TIME:
      activeFrame = ALL_TIME;
      break;
    case STUDY_PERIOD:
      activeFrame = STUDY_PERIOD;
      break;
    case MONTHLY:
      activeFrame = MONTHLY;
      break;
    case WEEKLY:
      activeFrame = WEEKLY;
      break;
    case DAILY:
      activeFrame = DAILY;
      break;
    default:
      activeFrame = STUDY_YEAR;
      break;
  }

  if (fetching) {
    return <LoadingData />;
  }

  if (error) {
    console.error('Error:', error);
    return <Error />;
  }

  return (
    <div className={styles.statsWrapper}>
      <ul className={styles.inlineList}>
        <li className={activeFrame === ALL_TIME ? styles.selected : ''}>
          <Link href={getTimeFrameRef(pathname, ALL_TIME)}>
            <a>All time</a>
          </Link>
        </li>
        <li className={activeFrame === STUDY_YEAR ? styles.selected : ''}>
          <Link href={pathname}>
            <a>Study year</a>
          </Link>
        </li>
        <li className={activeFrame === STUDY_PERIOD ? styles.selected : ''}>
          <Link href={getTimeFrameRef(pathname, STUDY_PERIOD)}>
            <a>Study Period</a>
          </Link>
        </li>
        <li className={activeFrame === MONTHLY ? styles.selected : ''}>
          <Link href={getTimeFrameRef(pathname, MONTHLY)}>
            <a>Monthly</a>
          </Link>
        </li>
        <li className={activeFrame === WEEKLY ? styles.selected : ''}>
          <Link href={getTimeFrameRef(pathname, WEEKLY)}>
            <a>Weekly</a>
          </Link>
        </li>
        <li className={activeFrame === DAILY ? styles.selected : ''}>
          <Link href={getTimeFrameRef(pathname, DAILY)}>
            <a>Daily</a>
          </Link>
        </li>
      </ul>
      <AllStatsTable stats={data.stats} />
    </div>
  );
};

function getTimeFrameRef(pathName: string, timeFrame: string | string[]): string {
  if (typeof timeFrame === 'string') {
    return `${pathName}?timeframe=${timeFrame}`;
  }
  return pathName;
}

export default AllStats;
