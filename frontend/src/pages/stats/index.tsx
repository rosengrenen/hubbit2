import React from 'react';

import { gql } from '@urql/core';
import { GetServerSidePropsContext, NextPage } from 'next';
import Link from 'next/link';
import { useRouter } from 'next/router';

import { Period, StatsInput, StatsQuery } from '../../__generated__/graphql';
import AllStatsTable from '../../components/all-stats-table/AllStatsTable';
import { defaultGetServerSidePropsWithCallbackInput, PageProps } from '../../util';

import styles from './index.module.scss';

export const STATS_QUERY = gql`
  query Stats($input: StatsInput) {
    stats(input: $input) {
      user {
        nick
      }
      time
    }
  }
`;

const ALL_TIME = 'all_time';
const STUDY_YEAR = 'study_year';
const STUDY_PERIOD = 'study_period';
const MONTHLY = 'monthly';
const WEEKLY = 'weekly';
const DAILY = 'daily';

const AllStats: NextPage<PageProps<StatsQuery>> = ({ data }) => {
  const { pathname, query } = useRouter();
  const timeFrame = query['timeframe'];

  let activeFrame: string;
  switch (timeFrame) {
    case ALL_TIME:
      activeFrame = ALL_TIME;
      break;
    case STUDY_YEAR:
      activeFrame = STUDY_YEAR;
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

  if (!data) {
    return null;
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

function getInputProps(context: GetServerSidePropsContext) {
  const timeFrame = context.query['timeframe'];

  let statsInput: StatsInput = {};
  const currentDate = new Date(Date.now());
  switch (timeFrame) {
    case ALL_TIME:
      statsInput = {};
      break;
    case STUDY_PERIOD:
      statsInput = {
        studyPeriodStats: {
          year: currentDate.getFullYear(),
          // TODO(Vidde): Update when getting the current period is supported.
          period: Period.Lp1,
        },
      };
      break;
    case MONTHLY:
      statsInput = {
        monthStats: {
          year: currentDate.getFullYear(),
          month: currentDate.getMonth(),
        },
      };
      break;
    case WEEKLY:
      // TODO(Vidde): Implement when supported by BE
      break;
    case DAILY:
      statsInput = {
        dayStats: {
          year: currentDate.getFullYear(),
          month: currentDate.getMonth(),
          day: currentDate.getDate(),
        },
      };
      break;
    case STUDY_YEAR:
    default:
      statsInput = {
        studyYearStats: {
          year: currentDate.getFullYear(),
        },
      };
      break;
  }
  return { input: statsInput };
}

function getTimeFrameRef(pathName: string, timeFrame: string | string[]): string {
  if (typeof timeFrame === 'string') {
    return `${pathName}?timeframe=${timeFrame}`;
  }
  return pathName;
}

export default AllStats;

export const getServerSideProps = defaultGetServerSidePropsWithCallbackInput<StatsQuery>(STATS_QUERY, getInputProps);
