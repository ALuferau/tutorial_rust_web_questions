pub async fn get_questions(
    params: std::collections::HashMap<String, String>,
    store: crate::store::Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut limit = None;
    let mut offset = 20;
    if !params.is_empty() {
        let pagination = crate::types::pagination::get_pagination(params);
        limit = pagination.get_limit();
        offset = pagination.get_offset();
    }

    let res: Vec<crate::types::question::Question> = match store.get_questions(limit, offset).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&res))
}

pub async fn get_question(
    id: i32,
    store: crate::store::Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.get_question(id).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&res))
}

pub async fn add_question(
    store: crate::store::Store,
    new_question: crate::types::question::NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.add_question(new_question).await {
        return Err(warp::reject::custom(e))
    };

    Ok(warp::reply::with_status("Question added", warp::hyper::StatusCode::CREATED))
}


pub async fn update_question(
    id: i32,
    store: crate::store::Store,
    question: crate::types::question::Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.update_question(id, question).await {
        return Err(warp::reject::custom(e))
    };

    Ok(warp::reply::with_status("Question updated", warp::hyper::StatusCode::OK))
}

pub async fn delete_question(
    id: i32,
    store: crate::store::Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.delete_question(id).await {
        return Err(warp::reject::custom(e))
    };

    Ok(warp::reply::with_status("Question deleted", warp::hyper::StatusCode::OK))
}
