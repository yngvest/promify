use wasm_bindgen::prelude::*;
use promql_parser::parser::{self, Prettier};

#[wasm_bindgen]
pub fn promify(query: &str) -> String {
    match parser::parse(query) {
        Ok(expr) => {
            return expr.pretty(0, 1);
        }
        Err(_err) => {
            return query.to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        name: &'static str,
        input: &'static str,
        expected: &'static str,
    }

    #[test]
    fn test_promify() {
        let test_cases = vec![
            TestCase {
                name: "simple query",
                input: "http_requests_total[5m]",
                expected: "http_requests_total[5m]",
            },
            TestCase {
                name: "simple query 1",
                input: "min_over_time(rate(http_requests_total[5m])[30m:1m])",
                expected: "min_over_time(\n  rate(\n    http_requests_total[5m]\n  )[30m:1m]\n)", 
            },
            TestCase {
                name: "complex query 1",
                input: r#"histogram_quantile(0.9, rate(demo_api_request_duration_seconds_bucket{job="demo"}[5m])) > 0.05 and rate(demo_api_request_duration_seconds_count{job="demo"}[5m]) > 1"#,
                expected: "    histogram_quantile(\n      0.9,\n      rate(\n        demo_api_request_duration_seconds_bucket{job=\"demo\"}[5m]\n      )\n    )\n  >\n    0.05\nand\n    rate(\n      demo_api_request_duration_seconds_count{job=\"demo\"}[5m]\n    )\n  >\n    1", 
            },
            TestCase {
                name: "invalid query 1",
                input: "min_over_time(rate(",
                expected: "min_over_time(rate(", 
            },
        ];
        for case in test_cases {
            let output = promify(case.input);
            assert_eq!(
                output,
                case.expected,
                "Test case '{}' failed. Expected '{}', got '{}'",
                case.name,
                case.expected,
                output,
            );
        }
    }
}