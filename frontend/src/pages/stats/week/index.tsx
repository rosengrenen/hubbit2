import React from 'react';

import { gql } from '@urql/core';
import { GetServerSidePropsContext, NextPage } from 'next';
import { useRouter } from 'next/router';

import { StatsMonthQuery, StatsWeekQuery } from '../../../__generated__/graphql';
import Error from '../../../components/error/Error';
import { StatsNavigation, WEEK } from '../../../components/stats-navigation/StatsNavigation';
import StatsTable, {
  STATS_TABLE_ME_FRAGMENT,
  STATS_TABLE_STAT_FRAGMENT,
} from '../../../components/stats-table/StatsTable';
import { StatsTimespanSelect } from '../../../components/stats-timespan-select/StatsTimespanSelect';
import { defaultGetServerSideProps, PageProps } from '../../../util';

const STATS_WEEK_QUERY = gql`
  query StatsWeek($input: StatsWeekInput) {
    statsWeek(input: $input) {
      stats {
        ...StatsTableStat
      }
      curr {
        year
        week
      }
      next {
        year
        week
      }
      prev {
        year
        week
      }
    }
    me {
      ...StatsTableMe
    }
  }

  ${STATS_TABLE_STAT_FRAGMENT}
  ${STATS_TABLE_ME_FRAGMENT}
`;

const StatsWeek: NextPage<PageProps<StatsWeekQuery>> = ({ data }) => {
  const { pathname: path } = useRouter();

  if (!data) {
    return <Error />;
  }

  return (
    <div className={'statsWrapper'}>
      <StatsNavigation activeFrame={WEEK} />
      <StatsTimespanSelect
        // TODO(Vidarm): Show date-span here when implemented in BE.
        current={`W${data.statsWeek.curr.week} ${data.statsWeek.curr.year}`}
        prev={`${path}?year=${data.statsWeek.prev.year}&week=${data.statsWeek.prev.week}`}
        next={`${path}?year=${data.statsWeek.next.year}&week=${data.statsWeek.next.week}`}
      />
      <StatsTable stats={data.statsWeek.stats} me={data.me} />
    </div>
  );
};

export default StatsWeek;

function getInputProps(context: GetServerSidePropsContext) {
  let year = NaN;
  const yearString = context.query['year'];
  if (yearString) {
    year = parseInt(yearString.toString(), 10);
  }

  let week = NaN;
  const weekString = context.query['week'];
  if (weekString) {
    week = parseInt(weekString.toString(), 10);
  }

  if (isNaN(year) || isNaN(week)) {
    return {};
  }

  return {
    input: {
      year: year,
      week: week,
    },
  };
}

export const getServerSideProps = defaultGetServerSideProps<StatsMonthQuery>(STATS_WEEK_QUERY, getInputProps);
