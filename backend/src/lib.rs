pub mod broker;
pub mod config;
pub mod error;
pub mod event;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod schema;
pub mod services;
pub mod utils;

use mobc::{Connection, Pool};
use mobc_redis::RedisConnectionManager;

pub type RedisPool = Pool<RedisConnectionManager>;
pub type RedisConnection = Connection<RedisConnectionManager>;
