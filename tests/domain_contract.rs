use financial_api::{
    AShareCode, CompanyId, Cursor, FinancialRange, FinancialReport, ManagerId, NaturalDate,
    ReportType, ShanghaiDateMillis, Thscode, UnixMillis,
};
use time::macros::date;

#[test]
fn complete_target_code_is_normalized_and_rejects_tickers() {
    let code = Thscode::new(" 600519.sh ").unwrap();
    assert_eq!(code.as_str(), "600519.SH");
    let error = Thscode::new("600519").unwrap_err();
    assert_eq!(error.field(), "thscode");
    assert_eq!(error.problem(), "must include a market suffix");
}

#[test]
fn a_share_code_rejects_non_a_share_suffixes() {
    assert!(AShareCode::new("600519.SH").is_ok());
    assert!(AShareCode::new("510300.OF").is_err());
    assert!(AShareCode::new("886042.TI").is_err());
}

#[test]
fn natural_date_validates_the_calendar() {
    assert_eq!(
        NaturalDate::parse("2024-02-29").unwrap().to_string(),
        "2024-02-29"
    );
    assert!(NaturalDate::parse("2023-02-29").is_err());
    assert!(NaturalDate::parse("2024-2-9").is_err());
}

#[test]
fn financial_range_makes_conflicting_modes_unrepresentable() {
    assert!(FinancialRange::recent(1).is_ok());
    assert!(FinancialRange::recent(20).is_ok());
    assert!(FinancialRange::recent(0).is_err());
    assert!(FinancialRange::recent(21).is_err());

    let start = UnixMillis::new(1_700_000_000_000).unwrap();
    let end = UnixMillis::new(1_600_000_000_000).unwrap();
    assert!(FinancialRange::between(start, end).is_err());
}

#[test]
fn domain_values_use_infallible_and_fallible_standard_conversions() {
    let target: Thscode = "600519.sh".parse().unwrap();
    let a_share: AShareCode = "600519.sh".parse().unwrap();
    let date_from_text: NaturalDate = "2024-02-29".parse().unwrap();
    let date_from_value = NaturalDate::from(date!(2024 - 02 - 29));
    let report: FinancialReport = "2024-4".parse().unwrap();

    assert_eq!(target, a_share.clone().into());
    assert_eq!(date_from_text, date_from_value);
    assert_eq!(report.as_str(), "2024-4");
    assert!("510300.OF".parse::<AShareCode>().is_err());
    assert!("2023-02-29".parse::<NaturalDate>().is_err());
}

#[test]
fn endpoint_identifiers_have_distinct_validated_types() {
    assert_eq!(ManagerId::new("manager-1").unwrap().as_str(), "manager-1");
    assert_eq!(CompanyId::new("company-1").unwrap().as_str(), "company-1");
    assert_eq!(
        Cursor::new("opaque+/cursor==").unwrap().as_str(),
        "opaque+/cursor=="
    );
    assert_eq!(ReportType::new("quarter").unwrap().as_str(), "quarter");
    assert_eq!(ManagerId::new(" manager-1 ").unwrap().as_str(), "manager-1");
    assert_eq!(
        CompanyId::new("\tcompany-1\n").unwrap().as_str(),
        "company-1"
    );
    assert_eq!(ReportType::new(" quarter ").unwrap().as_str(), "quarter");
    assert_eq!(
        Cursor::new(" opaque+/cursor== ").unwrap().as_str(),
        " opaque+/cursor== "
    );

    assert!(ManagerId::new(" ").is_err());
    assert!(CompanyId::new("").is_err());
    assert!(Cursor::new("\t").is_err());
    assert!(ReportType::new("\n").is_err());
}

#[test]
fn shanghai_date_millis_is_always_local_midnight() {
    let value = ShanghaiDateMillis::from_date(NaturalDate::parse("2024-05-20").unwrap()).unwrap();
    assert_eq!(value.get(), 1_716_134_400_000);
}
