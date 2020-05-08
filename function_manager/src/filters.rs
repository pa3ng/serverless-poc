use super::handlers;
use super::models::{Db, ListOptions, Function};
use warp::Filter;

/// The 4 TODOs filters combined.
pub fn functions(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    functions_list(db.clone())
        .or(functions_create(db.clone()))
        .or(functions_update(db.clone()))
        .or(functions_delete(db))
}

/// GET /functions?offset=3&limit=5
pub fn functions_list(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("functions")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_db(db))
        .and_then(handlers::list_functions)
}

/// POST /functions with JSON body
pub fn functions_create(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("functions")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::create_function)
}

/// PUT /functions/:id with JSON body
pub fn functions_update(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("functions" / u64)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::update_function)
}

/// DELETE /functions/:id
pub fn functions_delete(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // We'll make one of our endpoints admin-only to show how authentication filters are used
    let admin_only = warp::header::exact("authorization", "Bearer admin");

    warp::path!("functions" / u64)
        // It is important to put the auth check _after_ the path filters.
        // If we put the auth check before, the request `PUT /functions/invalid-string`
        // would try this filter and reject because the authorization header doesn't match,
        // rather because the param is wrong for that other path.
        .and(admin_only)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::delete_function)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Function,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}