#![feature(associated_type_defaults, async_fn_in_trait)]
#![allow(incomplete_features)]

pub mod error;
pub mod resources;
pub mod rpc;
pub mod snowflake_id;

use anyhow::Result;
use resource::{resource_macros, GenResourceID, Resource};
use serde::{Deserialize, Serialize};

static IDS: once_cell::sync::Lazy<tokio::sync::Mutex<snowflake_id::SnowflakeIdBucket>> =
    once_cell::sync::Lazy::new(|| {
        tokio::sync::Mutex::new(snowflake_id::SnowflakeIdBucket::new(2, 2))
    });
