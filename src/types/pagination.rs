use std::collections::HashMap;

use handle_errors::Error;

/// Pagination struct that is getting extracted
/// from query params
#[derive(Debug, Default, PartialEq)]
pub struct Pagination {
    /// The index of the last item which has to be returned
    pub limit: Option<i32>,
    /// The index of the second item that has to returned
    pub offset: i32,
}

/// Extract query parameter from `/questions` route
/// # Example query
/// GET requests to this route can have a pagination attached so we just
/// return the questions we need
/// `/questions?start=1&end=10`
/// # Example usage
/// ```rust
/// use std::collections::HashMap;
///
/// let mut query = HashMap::new();
/// query.insert("limit".to_string(), "1".to_string());
/// query.insert("offset".to_string(), "10".to_string());
/// let p = rustwebdev::types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.limit.unwrap(), 1);
/// assert_eq!(p.offset, 10);
/// ```
pub fn extract_pagination(
    params: HashMap<String, String>,
) -> Result<Pagination, handle_errors::Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<i32>()
                    .map_err(Error::ParseError)?,
            ),
            offset: params
                .get("offset")
                .unwrap()
                .parse::<i32>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParameters)
}

#[cfg(test)]
mod pagination_tests {
    use std::num::ParseIntError;

    use super::{extract_pagination, Error, HashMap, Pagination};

    #[test]
    fn valid_paginations() {
        let mut params = HashMap::new();
        params.insert("limit".to_string(), "1".to_string());
        params.insert("offset".to_string(), "1".to_string());
        let pagination_result = extract_pagination(params);
        let expected = Pagination {
            limit: Some(1),
            offset: 1,
        };
        assert_eq!(pagination_result.unwrap(), expected);
    }
    #[test]
    fn missing_offset_parameter() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("1"));
        let pagination_result =
            format!("{}", extract_pagination(params).unwrap_err());
        let expected = format!("{}", Error::MissingParameters);
        assert_eq!(pagination_result, expected);
    }
    
}
