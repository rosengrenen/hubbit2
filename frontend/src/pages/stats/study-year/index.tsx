import React from 'react';

import { gql } from '@urql/core';
import { NextPage } from 'next';

import { StatsAlltimeQuery } from '../../../__generated__/graphql';
import { ALL_TIME, StatsNavigation } from '../../../components/stats-navigation/StatsNavigation';
import StatsTable, { STATS_TABLE_FRAGMENT } from '../../../components/stats-table/StatsTable';
import { defaultGetServerSideProps, PageProps } from '../../../util';

const STATS_STUDY_YEAR_QUERY = gql`
    query StatsStudyYear{
        statsStudyYear{
            stats{
                ...StatsTable
            }
            year
        }
        ${STATS_TABLE_FRAGMENT}
    }
`;

const StudyYear: NextPage<PageProps<StatsStudyYear>> = ({ data }) => {
  if (!data) {
    return null;
  }

  return (
    <div className={'statsWrapper'}>
      <StatsNavigation activeFrame={ALL_TIME} />
      <StatsTable stats={data} />
    </div>
  );
};

export default StatsStudyYearQuery;

export const getServerSideProps = defaultGetServerSideProps<StatsAlltimeQuery>(STATS_ALL_TIME_QUERY);
rom 'react';
