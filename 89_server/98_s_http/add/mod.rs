use super::*;

use axum::extract::{Path, Query};
use serde::{Deserialize, Serialize};
use std::pin::Pin;

mod s_add_get;
mod s_add_post;
mod s_add_query;
pub use s_add_get::*;
pub use s_add_post::*;
pub use s_add_query::*;
