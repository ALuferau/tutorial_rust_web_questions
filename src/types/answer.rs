#[derive(Debug, serde::Serialize, serde::Deserialize, Eq, Hash, PartialEq, Clone)]
pub struct AnswerId(pub i32);
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: crate::types::question::QuestionId,
}

impl std::str::FromStr for AnswerId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(AnswerId(id.parse::<i32>().unwrap())),
            true => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No id provided",
            )),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct NewAnswer {
    pub content: String,
    pub question_id: crate::types::question::QuestionId,
}
