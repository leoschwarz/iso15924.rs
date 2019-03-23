extern crate iso15924;

use iso15924::ScriptDateError::*;
use iso15924::*;

#[test]
fn test_all() {
    // Test the number of script codes for backwards-compat purposes.
    //
    // This number CAN increase without breaking backwards compatibility,
    // but decreasing the number is a BC break.
    assert!(all().len() == 182);
}

#[test]
fn test_scriptdate_error_values() {
    // Test that all of these values remain in existence.
    let _ = DayTooLarge(1);
    let _ = DayTooSmall(1);
    let _ = MonthTooLarge(1);
    let _ = MonthTooSmall(1);
}

#[test]
fn test_scriptdate_inits() {
    ScriptDate::new(2000, 1, 1).ok();
    ScriptDate::new(2000, 0, 1).is_err();
    ScriptDate::new(2000, 13, 1).is_err();
    ScriptDate::new(2000, 1, 0).is_err();
    ScriptDate::new(2000, 1, 32).is_err();
}

#[test]
fn test_scriptdate_eqs() {
    let date1 = ScriptDate::new(2000, 1, 1).unwrap();
    let date2 = ScriptDate::new(2000, 1, 2).unwrap();

    assert!(date1 != date2);
    assert!(date1 == ScriptDate::new(2000, 1, 1).unwrap());
    assert!(date1 < date2);
    assert!(date1 <= date2);
    assert!(date2 > date1);
    assert!(date2 >= date1);
}

#[test]
fn test_get_by_alias() {
    get_by_alias("Ahom").is_some();
    get_by_alias(String::from("Ahom")).is_some();
    get_by_alias("aaaa").is_none();
}

#[test]
fn test_get_by_code() {
    get_by_code("Blis").is_some();
    get_by_code(String::from("Blis")).is_some();
    get_by_alias("abza").is_none();
}

#[test]
fn test_get_by_date_range() {
    let f = ScriptDate::new(2005, 1, 1).unwrap();
    let t = ScriptDate::new(2012, 1, 1).unwrap();

    let r1 = get_by_date_range(Some(f), Some(t));
    assert!(r1.len() == 62);
    let r2 = get_by_date_range(Some(f), None);
    assert!(r2.len() == 112);
    let r3 = get_by_date_range(None, Some(t));
    assert!(r3.len() == 132);
    let r4 = get_by_date_range(None, None);
    assert!(r4.len() == 0);
}

#[test]
fn test_get_by_name() {
    get_by_name("Adlam").is_some();
    get_by_name(String::from("Adlam")).is_some();
    get_by_name("Aaaaa").is_none();
}

#[test]
fn test_get_by_name_french() {
    get_by_name_french("arabe").is_some();
    get_by_name_french(String::from("arabe")).is_some();
    get_by_name_french("aaaaa").is_none();
}

#[test]
fn test_get_by_num() {
    get_by_num("070").is_some();
    get_by_num(String::from("070")).is_some();
    get_by_num("000").is_none();
}
