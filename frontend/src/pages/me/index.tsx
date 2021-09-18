import React from 'react';

import { useMeQuery } from '../../__generated__/graphql';
import Error from '../../components/error/Error';
import LoadingData from '../../components/loading-data/LoadingData';
import MacAddressesList from '../../components/mac-addresses-list/MacAddressesList';

import styles from './index.module.scss';

const Index = () => {
  const [{ data, fetching, error }] = useMeQuery();

  if (fetching) {
    return <LoadingData />;
  }

  if (error) {
    console.error('Error:', error);
    return <Error />;
  }

  return (
    <div className={styles.meWrapper}>
      <h1>{data.me.nick}</h1>
      <MacAddressesList initialDevices={data.me.devices} />
    </div>
  );
};

export default Index;
