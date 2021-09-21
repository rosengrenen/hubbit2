import React from 'react';

import { GetServerSideProps } from 'next';

const Stats = () => null;

export default Stats;

export const getServerSideProps: GetServerSideProps = async () => {
  return {
    redirect: {
      destination: '/stats/study-year',
      permanent: true,
    },
  };
};
