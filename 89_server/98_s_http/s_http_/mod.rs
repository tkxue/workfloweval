use super::*;

use axum::{
    Router,
    body::Bytes,
    extract::FromRequest,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
};

/*
pub static _G_S_Http: OnceLock<S_Http> = OnceLock::new();

pub struct G_S_Http {}

impl G_S_Http {
    pub fn __init_once() {
        if _G_S_Http.get().is_none() {
            let _ = _G_S_Http.set(S_Http::new());
        }
    }

    pub async fn all_routes() -> AxumRon<Vec<String>> {
        let t = _G_S_Http.get().unwrap().routes.clone();
        AxumRon(t)
    }
}

impl S_Http {
    pub fn new() -> S_Http {
        let s = Self::routes_axum();
        let routes = s.into_iter().map(|(k, v)| k.to_string()).collect::<Vec<_>>();
        S_Http { routes: routes }
    }

}

// 1. Data Structures with nanoserde traits
#[derive(DeJson)]
struct AddRequest {
    a: i32,
    b: i32,
}

#[derive(SerJson)]
struct AddResponse {
    result: i32,
}

// 2. Custom nanoserde JSON Extractor
struct NanoserdeJson<T>(T);

impl<S, T> FromRequest<S> for NanoserdeJson<T>
where
    T: DeJson,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<axum::body::Body>, state: &S) -> Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let body_str = std::str::from_utf8(&bytes).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UTF-8".into()))?;

        // Use nanoserde's JSON parser
        let data = DeJson::deserialize_json(body_str).map_err(|e| (StatusCode::BAD_REQUEST, format!("JSON Parse Error: {}", e)))?;

        Ok(NanoserdeJson(data))
    }
}

// 3. Handler
async fn add_handler(NanoserdeJson(payload): NanoserdeJson<AddRequest>) -> impl IntoResponse {
    let sum = payload.a + payload.b;
    let response_body = SerJson::serialize_json(&AddResponse { result: sum });

    Response::builder().header("content-type", "application/json").body(response_body).unwrap()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/add", post(add_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("JSON Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}


 */
