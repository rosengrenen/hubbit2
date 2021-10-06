import React from 'react';

import { gql } from '@urql/core';
import { NextPage } from 'next';

import { MeQuery } from '../../__generated__/graphql';
import DeviceList, { DEVICE_FRAGMENT } from '../../components/device-list/DeviceList';
import Error from '../../components/error/Error';
import { defaultGetServerSideProps, PageProps } from '../../util';

import styles from './index.module.scss';

const ME_QUERY = gql`
  query Me {
    me {
      cid
      nick
      devices {
        ...Device
      }
    }
  }

  ${DEVICE_FRAGMENT}
`;

const Index: NextPage<PageProps<MeQuery>> = ({ data }) => {
  if (!data) {
    return <Error />;
  }

  return (
    <div className={styles.meWrapper}>
      <h1>{data.me.nick}</h1>
      <DeviceList initialDevices={data.me.devices} />
    </div>
  );
};

export default Index;

export const getServerSideProps = defaultGetServerSideProps<MeQuery>(ME_QUERY);
