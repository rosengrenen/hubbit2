import React from 'react';

import { gql } from '@urql/core';
import { NextPage } from 'next';
import Head from 'next/head';

import { StatsAlltimeQuery } from '../../../__generated__/graphql';
import Error from '../../../components/error/Error';
import { ALL_TIME, StatsNavigation } from '../../../components/stats-navigation/StatsNavigation';
import StatsTable, {
  STATS_TABLE_ME_FRAGMENT,
  STATS_TABLE_STAT_FRAGMENT,
} from '../../../components/stats-table/StatsTable';
import { createTitle, defaultGetServerSideProps, PageProps } from '../../../util';

const STATS_ALL_TIME_QUERY = gql`
  query StatsAlltime {
    statsAlltime {
      ...StatsTableStat
    }
    me {
      ...StatsTableMe
    }
  }

  ${STATS_TABLE_STAT_FRAGMENT}
  ${STATS_TABLE_ME_FRAGMENT}
`;

const AllTime: NextPage<PageProps<StatsAlltimeQuery>> = ({ data }) => {
  if (!data) {
    return <Error />;
  }

  return (
    <>
      <Head>
        <title>{createTitle(`Stats since beginning`)}</title>
      </Head>
      <div className="statsWrapper">
        <StatsNavigation activeFrame={ALL_TIME} />
        <StatsTable stats={data.statsAlltime} me={data.me} hideChange={true} />
      </div>
    </>
  );
};

export default AllTime;

export const getServerSideProps = defaultGetServerSideProps<StatsAlltimeQuery>(STATS_ALL_TIME_QUERY);
