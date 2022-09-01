#[derive(Debug)]
pub struct SearchResult<'a> {
    pub line_no: usize,
    pub column: usize,
    pub line: &'a str,
}

#[derive(Debug)]
pub struct MultipleSearchResult<'a> {
    pub line_no: usize,
    pub line: &'a str
}

pub enum SearchType {
    Any,
    All
}

impl<'a> SearchResult <'a> {
    pub fn new(line_no: usize, column: usize, line: &'a str) -> Self {
        Self { line_no: line_no, column: column, line: line }
    }
}

impl<'a> MultipleSearchResult<'a> {
    pub fn new(line_no: usize, line: &'a str) -> Self {
        Self { line_no: line_no, line: line }
    }
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<SearchResult<'a>> {
    let mut result_vector: Vec<SearchResult<'a>> = Vec::new();

    for (idx, line) in content.lines().enumerate() {

        let col = line.find(query);
        match col {
            Some(col) => {
                let search_result = SearchResult::new(idx, col, line);
                result_vector.push(search_result);
            }
            None => {}
        }
    }
    result_vector
}

fn contains_any(queries: &[&str], line: &str) -> bool {
    for query in queries {
        if line.contains(query) {
            return true;
        }
    }
    false
}

fn contains_all(queries: &[&str], line: &str) -> bool {
    for query in queries {
        if !line.contains(query) {
            return false;
        }
    }
    true
}

pub fn search_multiple<'a>(queries: &[&str], content: &'a str, search_type: &SearchType) -> Vec<MultipleSearchResult<'a>> {
    let mut result_vector: Vec<MultipleSearchResult<'a>> = Vec::new();

    let cmp_fn:fn(&[&str], &str) -> bool;

    match search_type {
        SearchType::Any => {
            cmp_fn = contains_any;
        }
        &SearchType::All => {
            cmp_fn = contains_all;
        }
    }

    for(idx, line) in content.lines().enumerate() {
        if cmp_fn(queries, line) {
            result_vector.push(MultipleSearchResult::new(idx, line));
        }
    }

    result_vector
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "the quick brown fox jumps over the lazy dog";
    const MULTILINE_TEST_STR: &str = "<log data> nginx\napache <log data>\n31/08/2022: nginx: <log data>";

    #[test]
    fn test_search_result_with_matches() {
        let test_string = "<log data> nginx\napache <log data>\n31/08/2022: nginx: <log data>";
        let query = "nginx";

        let result_vector = search(query, test_string);

        let result_1 = &result_vector[0];

        assert_eq!(result_1.line_no, 0);
        assert_eq!(result_1.column, 11);
        assert_eq!(result_1.line, "<log data> nginx");

        let result_2 = &result_vector[1];

        assert_eq!(result_2.line_no, 2);
        assert_eq!(result_2.column, 12);
        assert_eq!(result_2.line, "31/08/2022: nginx: <log data>");
    }

    #[test]
    fn test_contains_any_true() {
        assert!(contains_any(vec!["the", "fast"].as_slice(), TEST_STR));
    }

    #[test]
    fn test_contains_any_false() {
        assert!(!contains_any(vec!["cat", "slow"].as_slice(), TEST_STR))
    }

    #[test]
    fn test_contains_all_true() {
        assert!(contains_all(vec!["fox", "dog"].as_slice(), TEST_STR));
    }

    #[test]
    fn test_contains_all_false() {
        assert!(!contains_all(vec!["fox", "cat"].as_slice(), TEST_STR));
    }

    #[test]
    fn test_search_with_any_multiple_queries() {

    }
}
