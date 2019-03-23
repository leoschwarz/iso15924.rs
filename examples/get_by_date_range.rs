extern crate iso15924;

use iso15924::{get_by_date_range, ScriptDate};

fn main() {
    // Retrieve a `ScriptCode` based on a range of `ScriptCode`s' `date`s..

    // Define some `ScriptDate`s, which is simply a struct that contains a
    // `year`, `month`, and `day`.
    let date2005 = ScriptDate::new(2005, 1, 1).unwrap();
    let date2012 = ScriptDate::new(2012, 1, 1).unwrap();

    // Retrieve `ScriptCode`s between `2005-01-01` and `2012-01-01`:
    let _ = get_by_date_range(Some(date2005), Some(date2012));

    // Retrieve `ScriptCode`s after `2005-01-01`:
    let _ = get_by_date_range(Some(date2005), None);

    // Retrieve `ScriptCode`s before `2012-01-01`:
    let _ = get_by_date_range(None, Some(date2012));

    // Consequentially, you can also retrieve no values:
    let _ = get_by_date_range(None, None);
}
