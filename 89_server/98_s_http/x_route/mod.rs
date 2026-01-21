use super::*;

use std::marker::PhantomData;
use std::pin::Pin;

use axum::{
    Json, Router,
    body::Bytes,
    extract::{FromRequest, Path, Query, State},
    http::{Request, StatusCode, header},
    routing::{get, post},
};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::sync::Arc;

pub struct X_Route {
    pub name: &'static str,
    pub handler: MethodRouter,
    pub json_examples: String,
}

pub trait Http_Post_Handler_T {
    type Path: Serialize + DeserializeOwned + Send + Sync + 'static;
    type Query: Serialize + DeserializeOwned + Send + Sync + 'static;
    type Body: Serialize + DeserializeOwned + Send + Sync + 'static;

    type Output: Sized + Serialize + DeserializeOwned + Send + Sync + 'static;

    fn examples() -> Vec<(Self::Path, Self::Query, Self::Body, Self::Output)>;
    fn handle(
        path: axum::extract::Path<Self::Path>,
        query: axum::extract::Query<Self::Query>,
        body: axum::Json<Self::Body>,
    ) -> Pin<Box<dyn Future<Output = Self::Output> + Send>>;
}

pub trait Http_Get_Handler_T {
    type Path: Serialize + DeserializeOwned + Send + Sync + 'static;
    type Query: Serialize + DeserializeOwned + Send + Sync + 'static;

    type Output: Sized + Serialize + DeserializeOwned + Send + Sync + 'static;

    fn examples() -> Vec<(Self::Path, Self::Query, Self::Output)>;
    fn handle(path: axum::extract::Path<Self::Path>, query: axum::extract::Query<Self::Query>) -> Pin<Box<dyn Future<Output = Self::Output> + Send>>;
}

impl X_Route {
    pub fn get<H: Http_Get_Handler_T>(name: &'static str, _: H) -> X_Route {
        X_Route {
            name,
            handler: get(|p: Path<H::Path>, q: Query<H::Query>| async {
                let resp = H::handle(p, q).await;
                Json(resp)
            }),
            json_examples: serde_json::to_string_pretty(&H::examples()).unwrap_or("error json obj -> string".to_string()),
        }
    }

    pub fn post<H: Http_Post_Handler_T>(name: &'static str, _: H) -> X_Route {
        X_Route {
            name,
            handler: post(|p: Path<H::Path>, q: Query<H::Query>, b: axum::Json<H::Body>| async {
                let resp = H::handle(p, q, b).await;
                Json(resp)
            }),
            json_examples: serde_json::to_string_pretty(&H::examples()).unwrap_or("error json obj -> string".to_string()),
        }
    }
}
