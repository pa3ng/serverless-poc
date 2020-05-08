use std::env;
use warp::Filter;

mod filters;
mod models;
mod handlers;

/// Provides a RESTful web server managing some Functions.
///
/// API will be:
///
/// - `GET /functions`: return a JSON list of Functions.
/// - `POST /functions`: create a new Function.
/// - `PUT /functions/:id`: update a specific Function.
/// - `DELETE /functions/:id`: delete a specific Function.
#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=functions=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "functions=info");
    }
    pretty_env_logger::init();

    let db = models::blank_db();

    let api = filters::functions(db);

    // View access logs by setting `RUST_LOG=functions`.
    let routes = api.with(warp::log("functions"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// #[cfg(test)]
// mod tests {
//     use warp::http::StatusCode;
//     use warp::test::request;

//     use super::{
//         filters,
//         models::{self, Function},
//     };

//     #[tokio::test]
//     async fn test_post() {
//         let db = models::blank_db();
//         let api = filters::functions(db);

//         let resp = request()
//             .method("POST")
//             .path("/functions")
//             .json(&Function {
//                 id: 1,
//                 text: "test 1".into(),
//                 completed: false,
//             })
//             .reply(&api)
//             .await;

//         assert_eq!(resp.status(), StatusCode::CREATED);
//     }

//     #[tokio::test]
//     async fn test_post_conflict() {
//         let db = models::blank_db();
//         db.lock().await.push(function1());
//         let api = filters::functions(db);

//         let resp = request()
//             .method("POST")
//             .path("/functions")
//             .json(&function1())
//             .reply(&api)
//             .await;

//         assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
//     }

//     #[tokio::test]
//     async fn test_put_unknown() {
//         let _ = pretty_env_logger::try_init();
//         let db = models::blank_db();
//         let api = filters::functions(db);

//         let resp = request()
//             .method("PUT")
//             .path("/functions/1")
//             .header("authorization", "Bearer admin")
//             .json(&function1())
//             .reply(&api)
//             .await;

//         assert_eq!(resp.status(), StatusCode::NOT_FOUND);
//     }

//     fn function1() -> Function {
//         Function {
//             id: 1,
//             text: "test 1".into(),
//             completed: false,
//         }
//     }
// }