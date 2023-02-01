#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}
impl Pagination {
    pub fn new(
        params: &std::collections::HashMap<String, String>,
    ) -> Result<Self, handle_errors::Error> {
        let start = Pagination::get_value("start", params)?;
        let end = Pagination::get_value("end", params)?;
        if end >= start {
            Ok(Pagination { start, end })
        } else {
            Err(handle_errors::Error::InvalidRange)
        }
    }
    fn get_value(
        key: &str,
        params: &std::collections::HashMap<String, String>,
    ) -> Result<usize, handle_errors::Error> {
        params
            .get(key)
            .unwrap()
            .parse::<usize>()
            .map_err(handle_errors::Error::ParseError)
    }
}

pub fn get_pagination(
    params: std::collections::HashMap<String, String>,
) -> Result<Pagination, handle_errors::Error> {
    if params.contains_key("start") && params.contains_key("end") {
        Pagination::new(&params)
    } else {
        Err(handle_errors::Error::MissingParameters)
    }
}
