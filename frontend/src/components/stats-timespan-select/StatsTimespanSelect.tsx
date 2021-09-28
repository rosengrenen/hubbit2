import React from 'react';

import Link from 'next/link';

import styles from './StatsTimespanSelect.module.scss';

interface Props {
  current: string;
  prev: string;
  next: string;
}

export const StatsTimespanSelect = ({ current, prev, next }: Props) => {
  return (
    <div className={styles.row}>
      <Link href={prev}>
        <a>Prev</a>
      </Link>
      <div className={styles.currentText}>{current}</div>
      <Link href={next}>
        <a>Next</a>
      </Link>
    </div>
  );
};
