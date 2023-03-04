#[derive(Debug)]
pub struct Pagination {
    limit: Option<u32>,
    offset: u32,
}
impl Pagination {
    pub fn new(
        params: &std::collections::HashMap<String, String>,
    ) -> Self {
        let limit = Pagination::get_value("limit", params, None);
        let offset = Pagination::get_value("offset", params, Some(0_u32));

        Pagination { limit: limit, offset: offset.unwrap() }
    }
    pub fn get_limit(&self) -> Option<i32> {
        match self.limit {
            Some(limit) => Some(limit as i32),
            None => None,
        }
    }
    pub fn get_offset(&self) -> i32 {
        self.offset as i32
    }
    fn get_value(
        key: &str,
        params: &std::collections::HashMap<String, String>,
        default: Option<u32>,
    ) -> Option<u32> {
        if params.contains_key(key) {
            match params
            .get(key)
            .unwrap()
            .parse::<u32>() {
                Ok(val) => Some(val),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "Parsing {} error: {:?}", key, e);
                    default
                },
            }
        } else {
            default
        }
    }
}

pub fn get_pagination(
    params: std::collections::HashMap<String, String>,
) -> Pagination {
    Pagination::new(&params)
}


#[cfg(test)]
mod pagination_tests {
    use super::{Pagination, get_pagination};

    #[test]
    fn valid_pagination() {
        // arrange
        let params = std::collections::HashMap::from([
            (String::from("limit"), String::from("1")),
            (String::from("offset"), String::from("1")),
        ]);

        // act
        let pagination = Pagination::new(&params);

        // assert
        assert_eq!(pagination.get_limit(), Some(1));
        assert_eq!(pagination.get_offset(), 1);
    }

    #[test]
    fn get_valid_pagination() {
        // arrange
        let params = std::collections::HashMap::from([
            (String::from("limit"), String::from("1")),
            (String::from("offset"), String::from("1")),
        ]);

        // act
        let pagination = get_pagination(params);

        // assert
        assert_eq!(pagination.get_limit(), Some(1));
        assert_eq!(pagination.get_offset(), 1);
    }

    #[test]
    fn missing_offset_pagination() {
        // arrange
        let params = std::collections::HashMap::from([
            (String::from("limit"), String::from("1")),
        ]);

        // act
        let pagination = Pagination::new(&params);

        // assert
        assert_eq!(pagination.get_limit(), Some(1));
        assert_eq!(pagination.get_offset(), 0);
    }

    #[test]
    fn missing_limit_pagination() {
        // arrange
        let params = std::collections::HashMap::from([
            (String::from("offset"), String::from("1")),
        ]);

        // act
        let pagination = Pagination::new(&params);

        // assert
        assert_eq!(pagination.get_limit(), None);
        assert_eq!(pagination.get_offset(), 1);
    }

    #[test]
    fn missing_both_pagination() {
        // arrange
        let params = std::collections::HashMap::new();

        // act
        let pagination = Pagination::new(&params);

        // assert
        assert_eq!(pagination.get_limit(), None);
        assert_eq!(pagination.get_offset(), 0);
    }

    #[test]
    fn get_invalid_pagination() {
        // arrange
        let params = std::collections::HashMap::from([
            (String::from("limit"), String::from("one")),
            (String::from("offset"), String::from("one")),
        ]);

        // act
        let pagination = get_pagination(params);

        // assert
        assert_eq!(pagination.get_limit(), None);
        assert_eq!(pagination.get_offset(), 0);
    }
}