use chrono::Timelike;
use uuid::Uuid;

use crate::{
  models::UserSession,
  repositories::UserSessionRepository,
  utils::{MAX_DATETIME, MIN_DATETIME},
};

pub struct HourStatsService {
  user_session_repo: UserSessionRepository,
}

impl HourStatsService {
  pub fn new(user_session_repo: UserSessionRepository) -> Self {
    Self { user_session_repo }
  }

  pub async fn get_for_user(&self, user_id: Uuid) -> anyhow::Result<Vec<u32>> {
    let user_sessions = self
      .user_session_repo
      .get_range_for_user(*MIN_DATETIME, *MAX_DATETIME, user_id)
      .await?;

    let hour_stats = calculate_hour_stats(&user_sessions);

    Ok(hour_stats)
  }
}

fn calculate_hour_stats(sessions: &[UserSession]) -> Vec<u32> {
  let mut hour_stats = vec![0; 24];

  for session in sessions {
    let start_hour = session.start_time.hour();

    // If session is within an hour, only get minute diff
    if start_hour == session.end_time.hour() {
      hour_stats[start_hour as usize] +=
        (session.end_time - session.start_time).num_minutes() as u32;
      continue;
    }

    let end_hour = session.end_time.hour();
    let middle_hours = start_hour + 1..end_hour;

    hour_stats[start_hour as usize] += 59 - session.start_time.minute();
    for hour in middle_hours {
      hour_stats[hour as usize] += 60;
    }

    hour_stats[end_hour as usize] += session.end_time.minute() + 1;
  }

  hour_stats
}
