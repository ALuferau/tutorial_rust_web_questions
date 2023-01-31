pub async fn get_questions(
    params: std::collections::HashMap<String, String>,
    store: crate::store::Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let questions = store.questions.read().await;
    let mut start_pos = 0;
    let mut end_pos = 20;
    if !params.is_empty() {
        let pagination = crate::types::pagination::get_pagination(params)?;
        start_pos = pagination.start;
        end_pos = pagination.end;
    }

    let res: Vec<&crate::types::question::Question> = questions
        .values()
        .skip(start_pos)
        .take(end_pos - start_pos)
        .collect();

    Ok(warp::reply::json(&res))
}

pub async fn get_question(
    id: String,
    store: crate::store::Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let questions = store.questions.read().await;
    match questions.get(&crate::types::question::QuestionId(id)) {
        Some(q) => Ok(warp::reply::json(&q)),
        None => Err(warp::reject::custom(crate::error::Error::QuestionNotFound)),
    }
}

pub async fn add_question(
    store: crate::store::Store,
    question: crate::types::question::Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);

    Ok(warp::reply::with_status(
        "Question added",
        warp::hyper::StatusCode::CREATED,
    ))
}

pub async fn update_question(
    id: String,
    store: crate::store::Store,
    mut question: crate::types::question::Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    question.id = crate::types::question::QuestionId(id);
    match store.questions.write().await.get_mut(&question.id) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(crate::error::Error::QuestionNotFound)),
    };

    Ok(warp::reply::with_status(
        "Question updated",
        warp::hyper::StatusCode::OK,
    ))
}

pub async fn delete_question(
    id: String,
    store: crate::store::Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store
        .questions
        .write()
        .await
        .remove(&crate::types::question::QuestionId(id))
    {
        Some(_) => Ok(warp::reply::with_status(
            "Question deleted",
            warp::hyper::StatusCode::OK,
        )),
        None => Err(warp::reject::custom(crate::error::Error::QuestionNotFound)),
    }
}
