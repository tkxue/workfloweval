use super::*;
use axum::Json;
use axum::extract::{Path, Query};
use std::pin::Pin;

pub struct HttpResumee {}

pub struct HttpResumee_Add {}

pub struct HttpResumee_Del {}

pub struct HttpResumee_GetTerm {}

impl Http_Get_Handler_T for HttpResumee_GetTerm {
    type Path = ();
    type Query = ();
    type Output = ();

    fn examples() -> Vec<(Self::Path, Self::Query, Self::Output)> {
        todo!()
    }

    fn handle(path: Path<Self::Path>, query: Query<Self::Query>) -> Pin<Box<dyn Future<Output = Self::Output> + Send>> {
        todo!()
    }
}

impl Http_Post_Handler_T for HttpResumee_Add {
    type Path = ();
    type Query = ();
    type Body = ();
    type Output = ();

    fn examples() -> Vec<(Self::Path, Self::Query, Self::Body, Self::Output)> {
        todo!()
    }

    fn handle(path: Path<Self::Path>, query: Query<Self::Query>, body: Json<Self::Body>) -> Pin<Box<dyn Future<Output = Self::Output> + Send>> {
        todo!()
    }
}

impl Http_Post_Handler_T for HttpResumee_Del {
    type Path = ();
    type Query = ();
    type Body = ();
    type Output = ();

    fn examples() -> Vec<(Self::Path, Self::Query, Self::Body, Self::Output)> {
        todo!()
    }

    fn handle(path: Path<Self::Path>, query: Query<Self::Query>, body: Json<Self::Body>) -> Pin<Box<dyn Future<Output = Self::Output> + Send>> {
        todo!()
    }
}
