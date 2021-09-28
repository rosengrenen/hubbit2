import React from 'react';

import { gql } from '@urql/core';
import { NextPage } from 'next';

import { StatsAlltimeQuery } from '../../../__generated__/graphql';
import { ALL_TIME, StatsNavigation } from '../../../components/stats-navigation/StatsNavigation';
import StatsTable, { STATS_TABLE_FRAGMENT } from '../../../components/stats-table/StatsTable';
import { defaultGetServerSideProps, PageProps } from '../../../util';

const STATS_ALL_TIME_QUERY = gql`
    query StatsAlltime{
        statsAlltime{
            ...StatsTable
        }
        ${STATS_TABLE_FRAGMENT}

        me{
            cid
        }
    }
`;

const AllTime: NextPage<PageProps<StatsAlltimeQuery>> = ({ data }) => {
  if (!data) {
    return null;
  }

  return (
    <div className={'statsWrapper'}>
      <StatsNavigation activeFrame={ALL_TIME} />
      <StatsTable stats={data.statsAlltime} myCid={data.me.cid} />
    </div>
  );
};

export default AllTime;

export const getServerSideProps = defaultGetServerSideProps<StatsAlltimeQuery>(STATS_ALL_TIME_QUERY);
