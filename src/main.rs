use axum::{
    routing::get,
    Router,
    extract::Path,
    response::IntoResponse,
};

async fn exclusive_cube(Path(path): Path<String>) -> impl IntoResponse {
    let mut all_nums: Vec<u32> = Vec::new();

    for candidate_num in path.split("/") {
        let validated_num = match candidate_num.parse::<u32>() {
            Ok(is_digit) => is_digit,
            Err(_) => panic!("package id must be a digit")
        };
        all_nums.push(validated_num)
    }

    let exclusive_or = all_nums.into_iter().reduce(|a, b| a ^ b).unwrap();
    exclusive_or.pow(3).to_string()
}
async fn hello_world() -> &'static str {
    "Hello, world!"
}


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/get", get(hello_world))
        .route("/1/*path", get(exclusive_cube));

    Ok(router.into())
}
