use std::collections::HashMap;

use handle_errors::Error;

/// Pagination struct that is getting extracted
/// from query params
#[derive(Debug)]
pub struct Pagination {
    /// The index of the first item that has to returned
    pub start: usize,
    /// The index of the second item that has to returned
    pub end: usize,
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
/// query.insert("start".to_string(), "1".to_string());
/// query.insert("end".to_string(), "10".to_string());
/// let p = rest_api::types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.start, 1);
/// assert_eq!(p.end, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParameters)
}
