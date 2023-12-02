use axum::{
    routing::get,
    Router,
    extract::Path,
    response::IntoResponse,
};

async fn exclusive_cube(Path((num1, num2)): Path<(i32, i32)>) -> impl IntoResponse {
    (num1 ^ num2).pow(3).to_string()

}
async fn hello_world() -> &'static str {
    "Hello, world!"
}


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/get", get(hello_world))
        .route("/1/:num1/:num2", get(exclusive_cube));

    Ok(router.into())
}
