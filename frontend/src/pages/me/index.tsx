import React from 'react';

import { gql } from '@urql/core';
import { NextPage } from 'next';
import Head from 'next/head';

import { MeQuery } from '../../__generated__/graphql';
import DeviceList, { DEVICE_FRAGMENT } from '../../components/device-list/DeviceList';
import Error from '../../components/error/Error';
import { createTitle, defaultGetServerSideProps, PageProps } from '../../util';

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
    <>
      <Head>
        <title>{createTitle('My devices')}</title>
      </Head>
      <div className={styles.meWrapper}>
        <h1>{data.me.nick}</h1>
        <DeviceList initialDevices={data.me.devices} />
      </div>
    </>
  );
};

export default Index;

export const getServerSideProps = defaultGetServerSideProps<MeQuery>(ME_QUERY);
