import React from 'react';

import { gql } from '@urql/core';
import { GetServerSidePropsContext, NextPage } from 'next';
import { useRouter } from 'next/router';

import { StatsStudyYearQuery } from '../../../__generated__/graphql';
import Error from '../../../components/error/Error';
import { StatsNavigation, STUDY_YEAR } from '../../../components/stats-navigation/StatsNavigation';
import StatsTable, { STATS_TABLE_FRAGMENT } from '../../../components/stats-table/StatsTable';
import { StatsTimespanSelect } from '../../../components/stats-timespan-select/StatsTimespanSelect';
import { defaultGetServerSidePropsWithCallbackInput, PageProps } from '../../../util';

const STATS_STUDY_YEAR_QUERY = gql`
    query StatsStudyYear($input: StatsStudyYearInput) {
        statsStudyYear(input: $input) {
            stats {
                ...StatsTable
            }
            year
        }

        me {
            cid
        }

        ${STATS_TABLE_FRAGMENT}
    }
`;

const StudyYear: NextPage<PageProps<StatsStudyYearQuery>> = ({ data }) => {
  const router = useRouter();

  if (!data) {
    return <Error />;
  }

  const path = router.pathname;
  const currYear = data.statsStudyYear.year;
  const prevYear = currYear - 1;
  const nextYear = currYear + 1;

  return (
    <div className={'statsWrapper'}>
      <StatsNavigation activeFrame={STUDY_YEAR} />
      <StatsTimespanSelect
        // TODO(Vidarm): Show date-span here when implemented in BE.
        current={`${currYear} - ${nextYear}`}
        prev={`${path}?year=${prevYear}`}
        next={`${path}?year=${nextYear}`}
      />
      <StatsTable stats={data.statsStudyYear.stats} myCid={data.me.cid} />
    </div>
  );
};

export default StudyYear;

function getInputProps(context: GetServerSidePropsContext) {
  let year = NaN;
  const yearString = context.query['year'];
  if (yearString) {
    year = parseInt(yearString.toString(), 10);
  }

  if (isNaN(year)) {
    return {};
  }

  return {
    input: {
      year,
    },
  };
}

export const getServerSideProps = defaultGetServerSidePropsWithCallbackInput<StatsStudyYearQuery>(
  STATS_STUDY_YEAR_QUERY,
  getInputProps,
);
