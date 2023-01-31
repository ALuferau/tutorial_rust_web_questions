pub async fn add_answer(
    store: crate::store::Store,
    params: std::collections::HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let answer = crate::types::answer::Answer {
        id: store.get_next_answer_id().await,
        content: params.get("content").unwrap().to_string(),
        question_id: crate::types::question::QuestionId(
            params.get("questionId").unwrap().to_string(),
        ),
    };

    store
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);

    Ok(warp::reply::with_status(
        "Answer added",
        warp::hyper::StatusCode::OK,
    ))
}
