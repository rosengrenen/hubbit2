use super::Stat;
use crate::schema::user::User;
use chrono::{DateTime, Duration, Local, TimeZone};
use std::collections::HashMap;
use uuid::Uuid;

type DateTimeRange = (DateTime<Local>, DateTime<Local>);

pub fn calculate_stats(
  user_sessions: &HashMap<Uuid, Vec<DateTimeRange>>,
  range_start_time: DateTime<Local>,
  range_end_time: DateTime<Local>,
) -> Vec<Stat> {
  let mut stats = user_sessions
    .iter()
    .map(|(&user_id, sessions)| {
      let minutes = sessions.iter().fold(0, |prev, &(start_time, end_time)| {
        // Don't count session time outside of the range
        let session_minutes = if end_time > range_end_time {
          (range_end_time - start_time).num_minutes() as i32
        } else if start_time < range_start_time {
          (end_time - range_start_time).num_minutes() as i32
        } else {
          (end_time - start_time).num_minutes() as i32
        };

        prev + session_minutes
      });

      Stat {
        user: User { id: user_id },
        score: minutes,
        time: minutes,
      }
    })
    .collect::<Vec<_>>();
  stats.sort_by_key(|s| -s.score);
  stats
}

pub fn year_start_end(year: i32) -> (DateTime<Local>, DateTime<Local>) {
  let start_time = Local.ymd(year, 1, 1).and_hms(0, 0, 0);
  let end_time = Local.ymd(year, 12, 31).and_hms(23, 59, 59);
  (start_time, end_time)
}

pub fn month_start_end(year: i32, month: u32) -> (DateTime<Local>, DateTime<Local>) {
  let start_time = Local.ymd(year, month as u32, 1).and_hms(0, 0, 0);
  let end_time = if month == 12 {
    Local.ymd(year + 1, 1, 1).and_hms(23, 59, 59)
  } else {
    Local.ymd(year, month as u32 + 1, 1).and_hms(23, 59, 59)
  } - Duration::seconds(1);
  (start_time, end_time)
}

pub fn day_start_end(year: i32, month: u32, day: u32) -> (DateTime<Local>, DateTime<Local>) {
  let start_time = Local.ymd(year, month as u32, day as u32).and_hms(0, 0, 0);
  let end_time = Local
    .ymd(year, month as u32, day as u32)
    .and_hms(23, 59, 59);
  (start_time, end_time)
}
