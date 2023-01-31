#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}
impl Pagination {
    pub fn new(
        params: &std::collections::HashMap<String, String>,
    ) -> Result<Self, crate::error::Error> {
        let start = Pagination::get_value("start", &params)?;
        let end = Pagination::get_value("end", &params)?;
        if end >= start {
            Ok(Pagination {
                start: start,
                end: end,
            })
        } else {
            Err(crate::error::Error::InvalidRange)
        }
    }
    fn get_value(
        key: &str,
        params: &std::collections::HashMap<String, String>,
    ) -> Result<usize, crate::error::Error> {
        params
            .get(key)
            .unwrap()
            .parse::<usize>()
            .map_err(crate::error::Error::ParseError)
    }
}

pub fn get_pagination(
    params: std::collections::HashMap<String, String>,
) -> Result<Pagination, crate::error::Error> {
    if params.contains_key("start") && params.contains_key("end") {
        Pagination::new(&params)
    } else {
        Err(crate::error::Error::MissingParameters)
    }
}
