use financial_api::SUPPORTED_ENDPOINTS;
use std::fs;

const ENDPOINT_MODULES: &[(&str, &str)] = &[
    ("src/meta.rs", include_str!("../src/meta.rs")),
    ("src/market_dump.rs", include_str!("../src/market_dump.rs")),
    ("src/index.rs", include_str!("../src/index.rs")),
    (
        "src/a_share/auction.rs",
        include_str!("../src/a_share/auction.rs"),
    ),
    (
        "src/a_share/calendar.rs",
        include_str!("../src/a_share/calendar.rs"),
    ),
    (
        "src/a_share/corporate_actions.rs",
        include_str!("../src/a_share/corporate_actions.rs"),
    ),
    (
        "src/a_share/financials.rs",
        include_str!("../src/a_share/financials.rs"),
    ),
    (
        "src/a_share/prices.rs",
        include_str!("../src/a_share/prices.rs"),
    ),
    (
        "src/a_share/special_data.rs",
        include_str!("../src/a_share/special_data.rs"),
    ),
    (
        "src/a_share/valuations.rs",
        include_str!("../src/a_share/valuations.rs"),
    ),
    (
        "src/fund/company.rs",
        include_str!("../src/fund/company.rs"),
    ),
    (
        "src/fund/corporate_actions.rs",
        include_str!("../src/fund/corporate_actions.rs"),
    ),
    (
        "src/fund/diagnostics.rs",
        include_str!("../src/fund/diagnostics.rs"),
    ),
    (
        "src/fund/financials.rs",
        include_str!("../src/fund/financials.rs"),
    ),
    (
        "src/fund/holders.rs",
        include_str!("../src/fund/holders.rs"),
    ),
    (
        "src/fund/managers.rs",
        include_str!("../src/fund/managers.rs"),
    ),
    ("src/fund/market.rs", include_str!("../src/fund/market.rs")),
    ("src/fund/news.rs", include_str!("../src/fund/news.rs")),
    (
        "src/fund/offerings.rs",
        include_str!("../src/fund/offerings.rs"),
    ),
    (
        "src/fund/performance.rs",
        include_str!("../src/fund/performance.rs"),
    ),
    (
        "src/fund/portfolio.rs",
        include_str!("../src/fund/portfolio.rs"),
    ),
    (
        "src/fund/profile.rs",
        include_str!("../src/fund/profile.rs"),
    ),
];

#[test]
fn every_supported_endpoint_has_chinese_rustdoc_and_an_example() {
    for endpoint in SUPPORTED_ENDPOINTS {
        let marker = format!("pub async fn {}(", endpoint.name);
        let matches = ENDPOINT_MODULES
            .iter()
            .filter_map(|(path, source)| source.find(&marker).map(|index| (*path, *source, index)))
            .collect::<Vec<_>>();

        assert_eq!(
            matches.len(),
            1,
            "{} must have exactly one public client method",
            endpoint.name
        );

        let (path, source, method_index) = matches[0];
        let documentation = documentation_before(source, method_index);
        let summary = documentation
            .lines()
            .filter_map(|line| line.trim_start().strip_prefix("///"))
            .map(str::trim)
            .find(|line| !line.is_empty() && !line.starts_with('#'))
            .unwrap_or_default();
        let example_path = format!("/examples/{}.rs", endpoint.name);
        let example = fs::read_to_string(format!(
            "{}/examples/{}.rs",
            env!("CARGO_MANIFEST_DIR"),
            endpoint.name
        ))
        .unwrap_or_else(|error| panic!("failed to read {example_path}: {error}"));

        assert!(
            summary.chars().any(is_chinese),
            "{} in {path} must start with a Chinese summary",
            endpoint.name
        );
        assert!(
            documentation.contains("# 示例"),
            "{} in {path} must contain a `# 示例` section",
            endpoint.name
        );
        assert!(
            documentation.contains(&example_path),
            "{} in {path} must embed its executable example",
            endpoint.name
        );
        assert!(
            example.contains("from_env()?"),
            "{example_path} must show how to construct the client",
        );
        assert!(
            example.contains(&format!(".{}(", endpoint.name)),
            "{example_path} must call the documented method",
        );
    }
}

fn documentation_before(source: &str, method_index: usize) -> &str {
    let line_start = source[..method_index]
        .rfind('\n')
        .map_or(0, |index| index + 1);
    let before_method = &source[..line_start];
    let documentation_start = before_method.rfind("\n\n").map_or(0, |index| index + 2);

    &before_method[documentation_start..]
}

fn is_chinese(character: char) -> bool {
    matches!(
        character,
        '\u{3400}'..='\u{4dbf}' | '\u{4e00}'..='\u{9fff}' | '\u{f900}'..='\u{faff}'
    )
}
