use crate::searching::{MultipleSearchResult, SearchResult};

fn format_default(result: &MultipleSearchResult) -> String {
    format!("{}: {}\n", result.line_no, result.line)
}

fn format_minimal(result: &MultipleSearchResult) -> String {
    format!("{}\n", result.line)
}

fn format_verbose(result: &SearchResult) -> String {
    let prefix = format!("{}: ", result.line_no);
    let pad = result.column + prefix.len() + 1;
    format!("{}{}\n{:>pad$}\n", prefix, result.line, "^")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_minimal() {
        let sample_result = MultipleSearchResult::new(55, "this is log line");
        assert_eq!(format_minimal(&sample_result), "this is log line\n");
    }

    #[test]
    fn test_format_default() {
        let sample_search_result = MultipleSearchResult::new(54, "this is log line");
        assert_eq!(format_default(&sample_search_result), "54: this is log line\n");
    }

    #[test]
    fn test_format_verbose() {
        let sample_search_result = SearchResult::new(5, 12, "10/10/2020: nginx");
        assert_eq!(format_verbose(&sample_search_result), String::from("5: 10/10/2020: nginx\n               ^\n"));
    }
}