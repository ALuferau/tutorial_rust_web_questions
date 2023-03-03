pub async fn add_answer(
    session: crate::types::account::Session,
    store: crate::store::Store,
    new_answer: crate::types::answer::NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    let content = match crate::profanity::check_profanity(new_answer.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    if let Err(e) = store.add_answer(
        crate::types::answer::NewAnswer {
            content,
            question_id: new_answer.question_id,
        },
        &session.account_id,
    ).await {
        return Err(warp::reject::custom(e))
    };

    Ok(warp::reply::with_status("Answer added", warp::hyper::StatusCode::CREATED))
}
