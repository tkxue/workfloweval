#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::routing::{MethodRouter, post};
use axum::{Json, Router, response::Html, routing::get};
use axum::{
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use n_msg::*;
use s_app::{_G_S_App, G_S_App, S_App};
use s_shared::*;
use std::marker::PhantomData;
use std::sync::{Arc, OnceLock};

mod add;
mod http_doc_search;
mod http_resumee;
mod s_http_;
mod x_route;
pub use add::*;
use e_jsdata::L_JsData_Util;
pub use http_doc_search::*;
pub use http_resumee::*;
pub use s_http_::*;
pub use x_route::*;

pub struct G_S_Http {}

impl G_S_Http {
    pub fn main() {
        G_S_Ffi::__init_once();
        G_S_App::__init_once();
        _G_S_Ffi.get().unwrap().tokio_rt.block_on(Self::http_main());
    }

    pub async fn post_handler(Json(body): Json<N_ToS_Full>) -> impl IntoResponse {
        let t = body.mailbox.to_client();
        let res = G_S_App::handle(body).await.map(|x| N_ToC_Full { mailbox: t, inner: x });
        Json(res)
    }

    async fn http_main() {
        let cors = tower_http::cors::CorsLayer::permissive();
        let app = Router::new()
            .route("/app-api-post", post(Self::post_handler))
            .route("/app-api-ws", get(Self::ws_handler))
            .layer(cors);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:4000").await.unwrap();
        println!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await.unwrap();
    }

    async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
        ws.on_upgrade(Self::handle_socket)
    }

    async fn handle_socket(mut socket: WebSocket) {
        while let Some(Ok(msg)) = socket.recv().await {
            match msg {
                Message::Text(t) => {
                    socket.send(Message::Text(format!("pong").into())).await.unwrap();
                }
                Message::Close(_) => break,
                Message::Binary(bytes) => {
                    match L_JsData_Util::read_obj::<N_ToS_Full>(&bytes) {
                        Err(_) => {
                            socket
                                .send(Message::Text(format!("error, not N_Root_In_Full").into()))
                                .await
                                .unwrap();
                        }
                        Ok(v) => {
                            use e_jsdata::L_JsData_Util;
                            let out = {
                                let t = v.mailbox.to_client();
                                let res = G_S_App::handle(v).await.map(|x| N_ToC_Full { mailbox: t, inner: x });
                                let bytes = L_JsData_Util::obj_to_bytes(&res);

                                bytes
                            };
                            socket
                                .send(Message::Binary(axum::body::Bytes::copy_from_slice(out.as_slice())))
                                .await
                                .unwrap();
                        }
                    };
                }
                _ => {}
            }
        }
    }
}
