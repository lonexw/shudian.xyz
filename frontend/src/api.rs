use serde::{Serialize, Deserialize};
use gloo_net::http::{Request, Response};

use crate::types::Shop;

#[derive(Serialize)]
pub struct EdgedbQuery {
    query: String,
    variables: Option<String>
}

// #[derive(Deserialize)]
// pub struct QueryError {
//     message: String,
//     r#type: String,
//     code: u16
// }

#[derive(Deserialize)]
pub struct QueryResponse<T> {
    pub data: Vec<T>,
    // error: QueryError,
}

pub async fn get_shops() -> Result<QueryResponse<Shop>, String> {
    let query = "select Shop { id, name, cover_image, address, open_time, telephone, desc, tags }".to_string();
    let query = EdgedbQuery { query, variables: None };

    query_edgedb(query)
        .await
        .unwrap()
        .json()
        .await
        .map_err(|err| err.to_string())
}

async fn query_edgedb(query: EdgedbQuery) -> Result<Response, String> {
    let resp = Request::post("/db/edgedb/edgeql")
        .json(&query)
        .unwrap()
        .send()
        .await
        .unwrap();

    if !resp.ok() {
        Err(format!("Error fetching data {} ({})", resp.status(), resp.status_text()))
    } else {
        Ok(resp)
    }
}