import React from 'react';

import { gql } from '@urql/core';
import { GetServerSidePropsContext, NextPage } from 'next';
import { useRouter } from 'next/router';

import { StatsMonthQuery } from '../../../__generated__/graphql';
import Error from '../../../components/error/Error';
import { MONTH, StatsNavigation } from '../../../components/stats-navigation/StatsNavigation';
import StatsTable, {
  STATS_TABLE_ME_FRAGMENT,
  STATS_TABLE_STAT_FRAGMENT,
} from '../../../components/stats-table/StatsTable';
import { StatsTimespanSelect } from '../../../components/stats-timespan-select/StatsTimespanSelect';
import { defaultGetServerSideProps, PageProps } from '../../../util';

const STATS_MONTH_QUERY = gql`
  query StatsMonth($input: StatsMonthInput) {
    statsMonth(input: $input) {
      stats {
        ...StatsTableStat
      }
      curr {
        year
        month
      }
      next {
        year
        month
      }
      prev {
        year
        month
      }
    }
    me {
      ...StatsTableMe
    }
  }

  ${STATS_TABLE_STAT_FRAGMENT}
  ${STATS_TABLE_ME_FRAGMENT}
`;

const StatsMonth: NextPage<PageProps<StatsMonthQuery>> = ({ data }) => {
  const router = useRouter();

  if (!data) {
    return <Error />;
  }

  const path = router.pathname;

  return (
    <div className={'statsWrapper'}>
      <StatsNavigation activeFrame={MONTH} />
      <StatsTimespanSelect
        // TODO(Vidarm): Show date-span here when implemented in BE.
        current={`${new Date(data.statsMonth.curr.year, data.statsMonth.curr.month, 0, 0, 0).toLocaleString('default', {
          month: 'long',
        })} ${data.statsMonth.curr.year}`}
        prev={`${path}?year=${data.statsMonth.prev.year}&month=${data.statsMonth.prev.month}`}
        next={`${path}?year=${data.statsMonth.next.year}&month=${data.statsMonth.next.month}`}
      />
      <StatsTable stats={data.statsMonth.stats} me={data.me} />
    </div>
  );
};

export default StatsMonth;

function getInputProps(context: GetServerSidePropsContext) {
  let year = NaN;
  const yearString = context.query['year'];
  if (yearString) {
    year = parseInt(yearString.toString(), 10);
  }

  let month = NaN;
  const monthString = context.query['month'];
  if (monthString) {
    month = parseInt(monthString.toString(), 10);
  }

  if (isNaN(year) || isNaN(month)) {
    return {};
  }

  return {
    input: {
      year: year,
      month: month,
    },
  };
}

export const getServerSideProps = defaultGetServerSideProps<StatsMonthQuery>(STATS_MONTH_QUERY, getInputProps);
