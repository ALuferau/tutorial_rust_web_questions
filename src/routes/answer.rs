pub async fn add_answer(
    store: crate::store::Store,
    new_answer: crate::types::answer::NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.add_answer(new_answer).await {
        return Err(warp::reject::custom(e))
    };

    Ok(warp::reply::with_status("Answer added", warp::hyper::StatusCode::CREATED))
}
