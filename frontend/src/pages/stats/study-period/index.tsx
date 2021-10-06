import React from 'react';

import { gql } from '@urql/core';
import { GetServerSidePropsContext, NextPage } from 'next';
import { useRouter } from 'next/router';

import { Period, StatsStudyPeriodQuery } from '../../../__generated__/graphql';
import Error from '../../../components/error/Error';
import { StatsNavigation, STUDY_PERIOD } from '../../../components/stats-navigation/StatsNavigation';
import StatsTable, { STATS_TABLE_FRAGMENT } from '../../../components/stats-table/StatsTable';
import { StatsTimespanSelect } from '../../../components/stats-timespan-select/StatsTimespanSelect';
import { defaultGetServerSidePropsWithCallbackInput, PageProps } from '../../../util';

const STATS_STUDY_PERIOD_QUERY = gql`
    query StatsStudyPeriod($input: StatsStudyPeriodInput) {
        statsStudyPeriod(input: $input) {
            stats {
                ...StatsTable
            }
            period
            year
        }

        me {
            cid
        }

        ${STATS_TABLE_FRAGMENT}
    }
`;

const LP1 = 'LP1';
const LP2 = 'LP2';
const LP3 = 'LP3';
const LP4 = 'LP4';
const SUMMER = 'SUMMER';

const ALL_PERIODS = [Period.Summer, Period.Lp1, Period.Lp2, Period.Lp3, Period.Lp4];

const StudyPeriod: NextPage<PageProps<StatsStudyPeriodQuery>> = ({ data }) => {
  const router = useRouter();

  if (!data) {
    return <Error />;
  }

  const path = router.pathname;

  const currPeriod = data.statsStudyPeriod.period;
  const periodIndex = ALL_PERIODS.indexOf(currPeriod);
  // Add ALL_PERIOD.length because javascript is stupid and can otherwise give use negative values.
  const nextPeriod = ALL_PERIODS[(periodIndex + 1) % ALL_PERIODS.length];
  const prevPeriod = ALL_PERIODS[(periodIndex - 1 + ALL_PERIODS.length) % ALL_PERIODS.length];

  const currYear = data.statsStudyPeriod.year;
  const prevYear = prevPeriod === Period.Lp4 ? currYear - 1 : currYear;
  const nextYear = nextPeriod === Period.Summer ? currYear + 1 : currYear;

  return (
    <div className={'statsWrapper'}>
      <StatsNavigation activeFrame={STUDY_PERIOD} />
      <StatsTimespanSelect
        // TODO(Vidarm): Show date-span here when implemented in BE.
        current={`${currPeriod} ${formatYear(currYear)}/${formatYear(currYear + 1)}`}
        prev={`${path}?year=${prevYear}&period=${prevPeriod.toUpperCase()}`}
        next={`${path}?year=${nextYear}&period=${nextPeriod.toUpperCase()}`}
      />
      <StatsTable stats={data.statsStudyPeriod.stats} myCid={data.me.cid} />
    </div>
  );
};

export default StudyPeriod;

function getInputProps(context: GetServerSidePropsContext) {
  let year = NaN;
  const yearString = context.query['year'];
  const periodString = context.query['period'];
  if (yearString) {
    year = parseInt(yearString.toString(), 10);
  }

  const period = periodString ? parseStudyPeriod(periodString.toString()) : undefined;

  if (isNaN(year) || !period) {
    return {};
  }

  return {
    input: {
      year,
      period,
    },
  };
}

function parseStudyPeriod(studyPeriodString: string): Period | undefined {
  switch (studyPeriodString) {
    case SUMMER:
      return Period.Summer;
    case LP1:
      return Period.Lp1;
    case LP2:
      return Period.Lp2;
    case LP3:
      return Period.Lp3;
    case LP4:
      return Period.Lp4;
    default:
      return undefined;
  }
}

export const getServerSideProps = defaultGetServerSidePropsWithCallbackInput<StatsStudyPeriodQuery>(
  STATS_STUDY_PERIOD_QUERY,
  getInputProps,
);

const formatYear = (year: number) => {
  return year.toString().substring(2, 4);
};
