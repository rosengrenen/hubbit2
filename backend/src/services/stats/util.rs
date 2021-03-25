use super::DateTimeRange;
use crate::{
  models::UserSession,
  schema::{stats::Stat, user::User},
};
use chrono::{DateTime, Duration, Local, TimeZone};
use std::collections::HashMap;
use uuid::Uuid;

pub fn map_sessions(sessions: Vec<UserSession>) -> HashMap<Uuid, Vec<DateTimeRange>> {
  let mut sessions_map = HashMap::new();
  for session in sessions {
    sessions_map
      .entry(session.user_id)
      .or_insert_with(Vec::new)
      .push((
        session.start_time.with_timezone(&Local),
        session.end_time.with_timezone(&Local),
      ));
  }

  sessions_map
}

pub fn calculate_stats(
  user_sessions: &HashMap<Uuid, Vec<DateTimeRange>>,
  range_start_time: DateTime<Local>,
  range_end_time: DateTime<Local>,
) -> HashMap<Uuid, Stat> {
  user_sessions
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

      (
        user_id,
        Stat {
          user: User { id: user_id },
          score: minutes,
          time: minutes,
        },
      )
    })
    .collect()
}

pub fn join_stats(stats: &mut HashMap<Uuid, Stat>, other_stats: &HashMap<Uuid, Stat>) {
  for (user_id, stat) in other_stats {
    stats
      .entry(*user_id)
      .and_modify(|s| {
        s.score += stat.score;
        s.time += stat.time;
      })
      .or_insert(stat.clone());
  }
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
    Local.ymd(year, month as u32 + 1, 1).and_hms(0, 0, 0)
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
