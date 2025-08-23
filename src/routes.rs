use crate::handlers;
use sqlx::PgPool;
use warp::Filter;

pub fn routes(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let pool_filter = warp::any().map(move || pool.clone());

    let create_short_url_route = warp::path!("shorten")
        .and(warp::post())
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(handlers::create_short_url);

    let retrieve_original_url_route = warp::path!("shorten" / String)
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(handlers::retrieve_original_url);

    let update_short_url_route = warp::path!("shorten" / String)
        .and(warp::put())
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(handlers::update_short_url);

    let delete_short_url_route = warp::path!("shorten" / String)
        .and(warp::delete())
        .and(pool_filter.clone())
        .and_then(handlers::delete_short_url);

    let get_url_statistics_route = warp::path!("shorten" / String / "stats")
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(handlers::get_url_statistics);

    create_short_url_route
        .or(retrieve_original_url_route)
        .or(update_short_url_route)
        .or(delete_short_url_route)
        .or(get_url_statistics_route)
}
