pub async fn get_questions(
    params: std::collections::HashMap<String, String>,
    store: crate::store::Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut limit = None;
    let mut offset = 0;
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
    session: crate::types::account::Session,
    store: crate::store::Store,
    new_question: crate::types::question::NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
     let (title, content) = tokio::join!(
         crate::profanity::check_profanity(new_question.title),
         crate::profanity::check_profanity(new_question.content),
     );
     let (title, content) = (
         match title {
             Ok(res) => res,
             Err(e) => return Err(warp::reject::custom(e)),
         },
         match content {
             Ok(res) => res,
             Err(e) => return Err(warp::reject::custom(e)),
         }
     );

    if let Err(e) = store.add_question(
        crate::types::question::NewQuestion {
            title,
            content,
            tags: new_question.tags,
        },
        &session.account_id,
    ).await {
        return Err(warp::reject::custom(e))
    };
    Ok(warp::reply::with_status("Question added", warp::hyper::StatusCode::CREATED))
}


pub async fn update_question(
    id: i32,
    session: crate::types::account::Session,
    store: crate::store::Store,
    question: crate::types::question::Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    if store.is_question_owner(id, &session.account_id).await? {
        let title = match crate::profanity::check_profanity(question.title).await {
            Ok(res) => res,
            Err(e) => return Err(warp::reject::custom(e)),
        };
        let content = match crate::profanity::check_profanity(question.content).await {
            Ok(res) => res,
            Err(e) => return Err(warp::reject::custom(e)),
        };
        if let Err(e) = store.update_question(
            id,
            crate::types::question::Question {
                id: question.id,
                title,
                content,
                tags: question.tags,
            },
            &session.account_id,
        ).await {
            return Err(warp::reject::custom(e))
        };

        Ok(warp::reply::with_status("Question updated", warp::hyper::StatusCode::OK))
    } else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
}

pub async fn delete_question(
    id: i32,
    session: crate::types::account::Session,
    store: crate::store::Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if store.is_question_owner(id, &session.account_id).await? {
        if let Err(e) = store.delete_question(id, &session.account_id).await {
            return Err(warp::reject::custom(e))
        };

        Ok(warp::reply::with_status("Question deleted", warp::hyper::StatusCode::OK))
    } else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
}
