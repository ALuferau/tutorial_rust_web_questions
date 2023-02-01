use std::{collections::HashMap, ops::DerefMut, str::FromStr, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Store {
    pub questions:
        Arc<RwLock<HashMap<crate::types::question::QuestionId, crate::types::question::Question>>>,
    pub answers: Arc<RwLock<HashMap<crate::types::answer::AnswerId, crate::types::answer::Answer>>>,
    answer_id: Arc<RwLock<i32>>,
}
impl Store {
    pub fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
            answer_id: Arc::new(RwLock::new(0)),
        }
    }
    fn init() -> HashMap<crate::types::question::QuestionId, crate::types::question::Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
    pub async fn get_next_answer_id(&self) -> crate::types::answer::AnswerId {
        let mut id_ref = self.answer_id.write().await;
        let id = id_ref.deref_mut();
        *id += 1;
        crate::types::answer::AnswerId::from_str(&format!("{}", *id)).unwrap()
    }
}
