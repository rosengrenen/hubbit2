export interface DateDiff {
  daysSince: number;
  hoursSince: number;
  minutesSince: number;
  secondsSince: number;
}

export function dateDiffToAgoString(dateDiff: DateDiff) {
  if (dateDiff.daysSince === 0 && dateDiff.hoursSince === 0 && dateDiff.minutesSince === 0) {
    return 'Just now';
  }

  return `${dateDiffToString(dateDiff)} ago`;
}

export function dateDiffToString(dateDiff: DateDiff) {
  return dateDiff.daysSince > 0
    ? `${dateDiff.daysSince} day${dateDiff.daysSince > 1 ? 's' : ''}`
    : dateDiff.hoursSince > 0
    ? `${dateDiff.hoursSince} hour${dateDiff.hoursSince > 1 ? 's' : ''}`
    : dateDiff.minutesSince > 0
    ? `${dateDiff.minutesSince} minute${dateDiff.minutesSince === 1 ? '' : 's'}`
    : `${dateDiff.secondsSince} second${dateDiff.secondsSince === 1 ? '' : 's'}`;
}

const ONE_SECOND = 1000;
const SECONDS_PER_MINUTE = 60;
const MINUTES_PER_HOUR = 60;
const HOURS_PER_DAY = 24;

const now = new Date(Date.now());

export function timeSince(date: Date): DateDiff {
  return timeBetween(date, now);
}

export function timeBetween(dateA: Date, dateB: Date): DateDiff {
  const milliSecDiff = Math.abs(dateA.getTime() - dateB.getTime());
  const secondsAgo = Math.round(milliSecDiff / ONE_SECOND);
  const minutesAgo = Math.round(secondsAgo / SECONDS_PER_MINUTE);
  const hoursAgo = Math.round(minutesAgo / MINUTES_PER_HOUR);
  const daysAgo = (hoursAgo - (hoursAgo % HOURS_PER_DAY)) / HOURS_PER_DAY;

  return {
    daysSince: daysAgo,
    hoursSince: hoursAgo,
    minutesSince: minutesAgo,
    secondsSince: secondsAgo,
  };
}

export function isToday(date: Date): boolean {
  return (
    date.getUTCFullYear() === now.getUTCFullYear() &&
    date.getMonth() === now.getMonth() &&
    date.getUTCDate() === now.getUTCDate()
  );
}

export function formatDate(date: Date): string {
  return `${date.getDay()} ${date.toLocaleString('default', {
    month: 'short',
  })} ${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}`;
}
