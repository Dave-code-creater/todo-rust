
use std::sync::Arc;
use crate::models::user::{User};
use crate::dto::auth::RegisterRequest;
use anyhow::Result;
use mongodb::bson::oid::ObjectId;
