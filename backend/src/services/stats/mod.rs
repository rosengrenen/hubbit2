mod service;
mod util;

use chrono::{DateTime, Local};
pub use service::StatsService;

pub type DateTimeRange = (DateTime<Local>, DateTime<Local>);
