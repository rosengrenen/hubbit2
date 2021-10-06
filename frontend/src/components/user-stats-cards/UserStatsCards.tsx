import React from 'react';

import { UserStatsQuery } from '../../__generated__/graphql';
import { dateDiffToAgoString, dateDiffToString, formatDate, isToday, timeBetween, timeSince } from '../../dateUtil';

import styles from './UserStatsCards.module.scss';

interface props {
  userStats: UserStatsQuery['user'];
}

const UserStatsCards = ({ userStats }: props) => (
  <div className={styles.userStatsCardsWrapper}>
    <UserStatCard title={'Last Session'} content={getLastSessionText(userStats.recentSessions)} />
    <UserStatCard title={'Today'} content={getTodayText(userStats.recentSessions)} />
  </div>
);

interface userStatCardProps {
  title: string;
  content: string;
}

const UserStatCard = ({ title, content }: userStatCardProps) => (
  <div className={styles.infoContainer}>
    <h2 className={styles.infoHeader}>{title}</h2>
    <div className={styles.infoText}>{content}</div>
  </div>
);

function getLastSessionText(recentSessions: UserStatsQuery['user']['recentSessions']): any {
  if (recentSessions.length === 0) {
    return 'Never been seen in the Hubb! :o';
  }

  const lastSession = recentSessions[0];
  const lastSessionStartTime = new Date(lastSession.startTime);
  const lastSessionEndTime = new Date(lastSession.endTime);
  const timeSinceStr = dateDiffToAgoString(timeSince(lastSessionEndTime));
  const dateStr = formatDate(lastSessionEndTime);

  return (
    <>
      {timeSinceStr}
      <br />
      {dateStr}
      <br />
      {`For about ${dateDiffToString(timeBetween(lastSessionStartTime, lastSessionEndTime))}`}
    </>
  );
}

function getTodayText(recentSessions: UserStatsQuery['user']['recentSessions']): any {
  if (recentSessions.length === 0) {
    return 'Never been in the Hubb! :o';
  }

  const lastSession = recentSessions[0];
  const lastSessionEndTime = new Date(lastSession.endTime);

  if (!isToday(lastSessionEndTime)) {
    return 'Not seen today';
  }

  return `For about ${dateDiffToString(timeSince(lastSessionEndTime))}`;
}

export default UserStatsCards;
