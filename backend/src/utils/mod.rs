use chrono::{DateTime, Local, TimeZone};
use lazy_static::lazy_static;

pub mod gamma;

lazy_static! {
  pub static ref MIN_DATETIME: DateTime<Local> = Local.ymd(2000, 1, 1).and_hms(0, 0, 0);
  pub static ref MAX_DATETIME: DateTime<Local> = Local.ymd(2099, 12, 31).and_hms(23, 59, 59);
}
