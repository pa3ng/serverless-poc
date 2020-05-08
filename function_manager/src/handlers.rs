/// These are our API handlers, the ends of each filter chain.
/// Notice how thanks to using `Filter::and`, we can define a function
/// with the exact arguments we'd expect from each filter in the chain.
/// No tuples are needed, it's auto flattened for the functions.

use super::models::{Db, ListOptions, Function};
use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn list_functions(opts: ListOptions, db: Db) -> Result<impl warp::Reply, Infallible> {
    // Just return a JSON array of functions, applying the limit and offset.
    let functions = db.lock().await;
    let functions: Vec<Function> = functions
        .clone()
        .into_iter()
        .skip(opts.offset.unwrap_or(0))
        .take(opts.limit.unwrap_or(std::usize::MAX))
        .collect();
    Ok(warp::reply::json(&functions))
}

pub async fn create_function(create: Function, db: Db) -> Result<impl warp::Reply, Infallible> {
    log::debug!("create_function: {:?}", create);

    let mut vec = db.lock().await;

    for function in vec.iter() {
        if function.id == create.id {
            log::debug!("    -> id already exists: {}", create.id);
            // Function with id already exists, return `400 BadRequest`.
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    // No existing Function with id, so insert and return `201 Created`.
    vec.push(create);

    Ok(StatusCode::CREATED)
}

pub async fn update_function(
    id: u64,
    update: Function,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    log::debug!("update_function: id={}, function={:?}", id, update);
    let mut vec = db.lock().await;

    // Look for the specified Function...
    for function in vec.iter_mut() {
        if function.id == id {
            *function = update;
            return Ok(StatusCode::OK);
        }
    }

    log::debug!("    -> function id not found!");

    // If the for loop didn't return OK, then the ID doesn't exist...
    Ok(StatusCode::NOT_FOUND)
}

pub async fn delete_function(id: u64, db: Db) -> Result<impl warp::Reply, Infallible> {
    log::debug!("delete_function: id={}", id);

    let mut vec = db.lock().await;

    let len = vec.len();
    vec.retain(|function| {
        // Retain all Functions that aren't this id...
        // In other words, remove all that *are* this id...
        function.id != id
    });

    // If the vec is smaller, we found and deleted a Function!
    let deleted = vec.len() != len;

    if deleted {
        // respond with a `204 No Content`, which means successful,
        // yet no body expected...
        Ok(StatusCode::NO_CONTENT)
    } else {
        log::debug!("    -> function id not found!");
        Ok(StatusCode::NOT_FOUND)
    }
}