use super::*;
use axum::Json;

pub struct S_Add_Post {}

#[derive(Serialize, Deserialize)]
pub struct S_Add_Post__Body {
    a: i32,
    b: i32,
}

#[derive(Serialize, Deserialize)]
pub struct S_Add_Post__Output {
    res: i32,
}

impl Http_Post_Handler_T for S_Add_Post {
    type Path = ();
    type Query = ();
    type Body = S_Add_Post__Body;
    type Output = S_Add_Post__Output;

    fn examples() -> Vec<(Self::Path, Self::Query, Self::Body, Self::Output)> {
        vec![]
    }

    fn handle(path: Path<Self::Path>, query: Query<Self::Query>, body: Json<Self::Body>) -> Pin<Box<dyn Future<Output = Self::Output> + Send>> {
        let t = async move { S_Add_Post__Output { res: body.a + body.b } };
        Box::pin(t)
    }

    /*
    fn handle(path: Path<Self::Path>, query: Query<Self::Query>) -> Pin<Box<dyn Future<Output = Self::Output> + Send>> {
    }
    */
}
