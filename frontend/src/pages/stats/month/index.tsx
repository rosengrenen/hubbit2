import React from 'react';

import { gql } from '@urql/core';
import { GetServerSidePropsContext, NextPage } from 'next';
import { useRouter } from 'next/router';

import { StatsMonthQuery } from '../../../__generated__/graphql';
import Error from '../../../components/error/Error';
import { MONTH, StatsNavigation } from '../../../components/stats-navigation/StatsNavigation';
import StatsTable, { STATS_TABLE_FRAGMENT } from '../../../components/stats-table/StatsTable';
import { StatsTimespanSelect } from '../../../components/stats-timespan-select/StatsTimespanSelect';
import { defaultGetServerSidePropsWithCallbackInput, PageProps } from '../../../util';

const STATS_MONTH_QUERY = gql`
    query StatsMonth($input: StatsMonthInput!) {
        statsMonth(input: $input) {
            ...StatsTable
        }

        me {
            cid
        }

        ${STATS_TABLE_FRAGMENT}
    }
`;

const now = new Date(Date.now());

const StatsMonth: NextPage<PageProps<StatsMonthQuery>> = ({ data }) => {
  const router = useRouter();

  if (!data) {
    return <Error />;
  }

  const path = router.pathname;

  // TODO(Vidarm): Remove when backend returns the current month.
  const currMonth = queryToNumberOrDefault(router.query['month'], now.getMonth() + 1);
  const nextMonth = currMonth === 12 ? 1 : currMonth + 1;
  const prevMonth = currMonth === 1 ? 12 : currMonth - 1;

  const currYear = queryToNumberOrDefault(router.query['year'], now.getFullYear());
  const nextYear = nextMonth === 1 ? currYear + 1 : currYear;
  const prevYear = prevMonth === 12 ? currYear - 1 : currYear;

  return (
    <div className={'statsWrapper'}>
      <StatsNavigation activeFrame={MONTH} />
      <StatsTimespanSelect
        // TODO(Vidarm): Show date-span here when implemented in BE.
        current={`${new Date(currYear, currMonth, 0, 0, 0).toLocaleString('default', { month: 'long' })} ${currYear}`}
        prev={`${path}?year=${prevYear}&month=${prevMonth}`}
        next={`${path}?year=${nextYear}&month=${nextMonth}`}
      />
      <StatsTable stats={data.statsMonth} myCid={data.me.cid} />
    </div>
  );
};

export default StatsMonth;

function getInputProps(context: GetServerSidePropsContext) {
  const yearString = context.query['year'];
  const year = queryToNumberOrDefault(yearString, now.getFullYear());

  const monthString = context.query['month'];
  const month = queryToNumberOrDefault(monthString, now.getMonth() + 1);

  return {
    input: {
      year: year,
      month: month,
    },
  };
}

function queryToNumberOrDefault(query: string | string[] | undefined, alternative: number): number {
  let number = NaN;
  if (query) {
    number = parseInt(query.toString(), 10);
  }

  if (isNaN(number)) {
    return alternative;
  }

  return number;
}

export const getServerSideProps = defaultGetServerSidePropsWithCallbackInput<StatsMonthQuery>(
  STATS_MONTH_QUERY,
  getInputProps,
);
