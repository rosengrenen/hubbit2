import React from 'react';

import { gql } from '@urql/core';
import { NextPage } from 'next';

import { MeQuery } from '../../__generated__/graphql';
import MacAddressesList from '../../components/mac-addresses-list/MacAddressesList';
import { defaultGetServerSideProps, PageProps } from '../../util';

import styles from './index.module.scss';

const ME_QUERY = gql`
  query Me {
    me {
      cid
      nick
      devices {
        address
        name
      }
    }
  }
`;

const Index: NextPage<PageProps<MeQuery>> = ({ data }) => {
  if (!data) {
    return null;
  }

  return (
    <div className={styles.meWrapper}>
      <h1>{data.me.nick}</h1>
      <MacAddressesList initialDevices={data.me.devices} />
    </div>
  );
};

export default Index;

export const getServerSideProps = defaultGetServerSideProps<MeQuery>(ME_QUERY);