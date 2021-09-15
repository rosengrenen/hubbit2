import React from 'react';

import Link from 'next/link';
import { useRouter } from 'next/router';

import { Period, StatsInput, useStatsQuery } from '../../__generated__/graphql';
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
  const { pathname, query } = useRouter();
  const timeFrame = query['timeframe'];

  let activeFrame = STUDY_YEAR;
  let statsInput: StatsInput = {};
  const currentDate = new Date(Date.now());
  switch (timeFrame) {
    case ALL_TIME:
      activeFrame = ALL_TIME;
      break;
    case STUDY_YEAR:
      statsInput = {
        studyYearStats: {
          year: currentDate.getFullYear(),
        },
      };
      activeFrame = STUDY_YEAR;
      break;
    case STUDY_PERIOD:
      statsInput = {
        studyPeriodStats: {
          year: currentDate.getFullYear(),
          // TODO(Vidde): Update when getting the current period is supported.
          period: Period.Lp1,
        },
      };
      activeFrame = STUDY_PERIOD;
      break;
    case MONTHLY:
      statsInput = {
        monthStats: {
          year: currentDate.getFullYear(),
          month: currentDate.getMonth(),
        },
      };
      activeFrame = MONTHLY;
      break;
    case WEEKLY:
      // TODO(Vidde): Implement when supported by BE
      activeFrame = WEEKLY;
      break;
    case DAILY:
      statsInput = {
        dayStats: {
          year: currentDate.getFullYear(),
          month: currentDate.getMonth(),
          day: currentDate.getDate(),
        },
      };
      activeFrame = DAILY;
      break;
    default:
      activeFrame = STUDY_YEAR;
      break;
  }

  const [{ data, fetching, error }] = useStatsQuery({
    variables: {
      input: statsInput,
    },
  });

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