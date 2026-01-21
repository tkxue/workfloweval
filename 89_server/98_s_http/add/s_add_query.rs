use super::*;

pub struct S_Add_Query {}

#[derive(Serialize, Deserialize)]
pub struct S_Add_Query__Query {
    a: i32,
    b: i32,
}

#[derive(Serialize, Deserialize)]
pub struct S_Add_Query__Output {
    res: i32,
}

impl Http_Get_Handler_T for S_Add_Query {
    type Path = ();
    type Query = S_Add_Query__Query;
    type Output = S_Add_Query__Output;

    fn examples() -> Vec<(Self::Path, Self::Query, Self::Output)> {
        vec![]
    }

    fn handle(path: Path<Self::Path>, query: Query<Self::Query>) -> Pin<Box<dyn Future<Output = Self::Output> + Send>> {
        let t = async move { S_Add_Query__Output { res: query.a + query.b } };
        Box::pin(t)
    }
}
