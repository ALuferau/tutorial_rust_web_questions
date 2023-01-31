#[derive(Debug, serde::Serialize, serde::Deserialize, Eq, Hash, PartialEq, Clone)]
pub struct AnswerId(String);
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
            false => Ok(AnswerId(id.to_string())),
            true => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No id provided",
            )),
        }
    }
}
