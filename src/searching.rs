#[derive(Debug)]
pub struct SearchResult<'a> {
    pub line_no: usize,
    pub column: usize,
    pub line: &'a str,
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

#[test]
fn test_search_result_with_matches() {
    let test_string = "<log data> nginx
apache <log data>
31/08/2022: nginx: <log data>";
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