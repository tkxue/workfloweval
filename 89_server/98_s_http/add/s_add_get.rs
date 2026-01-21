use super::*;
use axum::extract::State;

pub struct S_Add_Get {}

#[derive(Serialize, Deserialize)]
pub struct S_Add_Get__Path {
    a: i32,
    b: i32,
}

#[derive(Serialize, Deserialize)]
pub struct S_Add_Get__Output {
    res: i32,
}

impl Http_Get_Handler_T for S_Add_Get {
    type Path = S_Add_Get__Path;
    type Query = ();
    type Output = S_Add_Get__Output;

    fn examples() -> Vec<(Self::Path, Self::Query, Self::Output)> {
        vec![]
    }

    fn handle(path: Path<Self::Path>, query: Query<Self::Query>) -> Pin<Box<dyn Future<Output = Self::Output> + Send>> {
        let t = async move { S_Add_Get__Output { res: path.a + path.b } };
        Box::pin(t)
    }
}
