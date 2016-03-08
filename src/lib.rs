// ISC License (ISC)
//
// Copyright (c) 2016, Austin Hellyer <hello@austinhellyer.me>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER
// RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF
// CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// What is ISO 15924?
//
// | ISO 15924, Codes for the representation of names of scripts, defines two
// | sets of codes for a number of writing systems (scripts). Each script is
// | given both a four-letter code and a numeric one.[1] Script is defined as
// | "set of graphic characters used for the written form of one or more
// | languages".
// |
// | - [Wikipedia](http://en.wikipedia.org/wiki/ISO_15924)
//
// Originally by taiyaeix on GitHub.

mod codes;

pub use codes::all;

/// `Err` `Result` enum for creation of `ScriptDate`s via `ScriptDate::new`.
///
/// Derives from `Debug`.
#[derive(Debug, PartialEq)]
pub enum ScriptDateError {
    /// When a `day` is provided that is greater than `31`.
    DayTooLarge(u16),
    /// When a `day` is provided that is less than `1`.
    DayTooSmall(u16),
    /// When a `month` is provided that is greater than `12`.
    MonthTooLarge(u16),
    /// When a `month` is provided that is less than `1`.
    MonthTooSmall(u16),
}

/// Representation of a date of introduction for a script code. This contains
/// `year`, `month`, and `day` public fields of `u16` type.
///
/// # Examples
///
/// Create a new valid `ScriptDate`:
///
/// ```rust
/// use iso15924::ScriptDate;
///
/// let _ = ScriptDate::new(1976, 7, 2).unwrap();
/// ```
///
/// Check if a `ScriptDate` is equal to, less than, etc. another `ScriptDate`:
///
/// ```rust
/// use iso15924::ScriptDate;
///
/// let date1 = ScriptDate::new(1997, 7, 7).unwrap();
/// let date2 = ScriptDate::new(2005, 5, 5).unwrap();
///
/// // Check that `date1` is not equal to `date2`.
/// assert!(date1 != date2);
///
/// // Check that `date1` is less than `date2`.
/// assert!(date1 < date2);
///
/// // Check that `date1` is _not_ greater than or equal to `date2`.
/// assert!(!(date1 >= date2));
/// ```
///
/// Derives from `Clone`, `Copy, `Debug`, and `PartialOrd`.
#[derive(Clone, Copy, Debug, PartialOrd)]
pub struct ScriptDate {
    pub year: u16,
    pub month: u16,
    pub day: u16,
}

impl ScriptDate {
    /// Creates a new `ScriptDate` from the given `year`, `month`, and `day`, if
    /// possible.
    ///
    /// If the month is greater than 12 or less than 1 or the day is greater
    /// than 31 or less than 1, then an `Err` is returned with a value from the
    /// `ScriptDateError` enum.
    ///
    /// # Examples
    ///
    /// Create a new valid `ScriptDate`:
    ///
    /// ```rust
    /// use iso15924::ScriptDate;
    ///
    /// let _ = ScriptDate::new(2000, 1, 1).unwrap();
    /// ```
    ///
    /// As there can be errors, it's a good idea to match for the various
    /// `ScriptDateError` values:
    ///
    /// ```rust
    /// use iso15924::{ScriptDateError, ScriptDate};
    ///
    /// let month_too_small = ScriptDate::new(2000, 0, 1);
    /// let month_too_large = ScriptDate::new(2000, 13, 1);
    /// let day_too_small = ScriptDate::new(2000, 1, 0);
    /// let day_too_large = ScriptDate::new(2000, 1, 35);
    ///
    /// assert!(match month_too_small {
    ///     Err(ScriptDateError::MonthTooSmall(_)) => true,
    ///     _ => false,
    /// });
    /// assert!(match month_too_large {
    ///     Err(ScriptDateError::MonthTooLarge(_)) => true,
    ///     _ => false,
    /// });
    /// assert!(match day_too_small {
    ///     Err(ScriptDateError::DayTooSmall(_)) => true,
    ///     _ => false,
    /// });
    /// assert!(match day_too_large {
    ///     Err(ScriptDateError::DayTooLarge(_)) => true,
    ///     _ => false,
    /// });
    /// ```
    pub fn new(year: u16,
               month: u16,
               day: u16) -> Result<ScriptDate, ScriptDateError> {
        if month > 12 {
            Err(ScriptDateError::MonthTooLarge(month))
        } else if month == 0 {
            Err(ScriptDateError::MonthTooSmall(month))
        } else if day > 31 {
            Err(ScriptDateError::DayTooLarge(day))
        } else if day == 0 {
            Err(ScriptDateError::DayTooSmall(day))
        } else {
            Ok(ScriptDate {
                year: year,
                month: month,
                day: day,
            })
        }
    }
}

impl PartialEq for ScriptDate {
    fn eq(&self, other: &ScriptDate) -> bool {
        let self_items = (self.year, self.month, self.day);
        let other_items = (other.year, other.month, other.day);

        self_items == other_items
    }
}

/// Representation of each script code with 6 pieces of information:
///
/// - *alias* - The `Property Value Alias` as defined by unicode.org, with a
///             definition located here:
///             <http://www.unicode.org/Public/UNIDATA/PropertyValueAliases.txt>
/// - *code* - 4-character representation of the script code.
/// - *date* - a `ScriptDate` containing a `u16` `year`, `month`, and `day`.
///            This is the date of introduction to the standard.
/// - *name* - The English name of the script code.
/// - *name_french* - The French name of the script code.
/// - *num* - Numeric 3-digit representation of the script code as a `str`.
///
/// Derives from `Clone`, `Copy`, and `Debug`.
#[derive(Clone, Copy, Debug)]
pub struct ScriptCode<'a> {
    pub alias: Option<&'a str>,
    pub code: &'a str,
    pub date: ScriptDate,
    pub name: &'a str,
    pub name_french: &'a str,
    pub num: &'a str,
}

/// Retrieve a `ScriptCode` via its `alias` (`Property Value Alias`) value if
/// one exists.
///
/// # Examples
///
/// ```rust
/// iso15924::get_by_alias("Ahom").is_some();
/// ```
pub fn get_by_alias<'a, S: Into<String>>(alias: S) -> Option<ScriptCode<'a>> {
    let alias_str: &str = &alias.into()[..];

    for code in all() {
        if code.alias.is_some() && code.alias.unwrap() == alias_str {
            return Some(code);
        }
    }

    None
}

/// Retrieve a `ScriptCode` via its `code` value if one exists.
///
/// # Examples
///
/// ```rust
/// iso15924::get_by_code("Blis").is_some();
/// iso15924::get_by_code("Abza").is_none();
/// ```
pub fn get_by_code<'a, S: Into<String>>(code: S) -> Option<ScriptCode<'a>> {
    let code_str: &str = &code.into()[..];

    for code in all() {
        if code.code == code_str {
            return Some(code);
        }
    }

    None
}

/// Retrieve a `Vec` of `ScriptCode`s with `ScriptDate`s that are within the
/// range of the `from` and `to` given. The `from` and `to` are both optional,
/// and can either be `None` or `Some(ScriptDate)` for variations of the range
/// wanted.
///
/// # Examples
///
/// Getting all `ScriptCode`s between `2005-01-01` and `2012-01-01`:
///
/// ```rust
/// use iso15924::{ScriptDate, get_by_date_range};
///
/// let date_from = ScriptDate::new(2005, 01, 01).unwrap();
/// let date_to = ScriptDate::new(2012, 01, 01).unwrap();
///
/// let scripts = get_by_date_range(Some(date_from), Some(date_to));
/// ```
///
/// Retrieving all `ScriptCode`s after `2005-01-01`:
///
/// ```rust
/// use iso15924::{ScriptDate, get_by_date_range};
///
/// let date_from = ScriptDate::new(2005, 01, 01).unwrap();
///
/// let scripts = get_by_date_range(Some(date_from), None);
/// ```
///
/// Retrieving all `ScriptCode`s before `2012-01-01`:
///
/// ```rust
/// use iso15924::{ScriptDate, get_by_date_range};
///
/// let date_to = ScriptDate::new(2012, 01, 01).unwrap();
///
/// let scripts = get_by_date_range(None, Some(date_to));
/// ```
///
/// Consequentially, you can also retrieve no values:
///
/// ```rust
/// use iso15924::get_by_date_range;
///
/// let scripts = get_by_date_range(None, None);
/// ```
pub fn get_by_date_range<'a>(from: Option<ScriptDate>,
                         to: Option<ScriptDate>) -> Vec<ScriptCode<'a>> {
    let from_do: bool = from.is_some();
    let to_do: bool = to.is_some();

    // If searching via neither the given `from` or `to`, then nothing will be
    // found, so just return an empty `Vec`.
    if !from_do && !to_do {
        return vec![];
    }

    let mut codes: Vec<ScriptCode> = vec![];

    for code in all() {
        // If the date of the given `from` is less than that of the `code`, then
        // don't push it to the `Vec`.
        if from_do && code.date < from.unwrap() {
            continue;
        }

        // If the date of the given `to` is greater than that of the `code`,
        // then don't push it to the `Vec`.
        if to_do && code.date > to.unwrap() {
            continue;
        }

        codes.push(code);
    }

    codes
}

/// Retrieve a `ScriptCode` via its `name` if it exists:
///
/// ```rust
/// use iso15924::get_by_name;
///
/// let script1 = get_by_name("Adlam");
/// script1.is_some();
///
/// let script2 = get_by_name("Aaaaa");
/// script2.is_none();
/// ```
pub fn get_by_name<'a, S: Into<String>>(name: S) -> Option<ScriptCode<'a>> {
    let name_str: &str = &name.into()[..];

    for code in all() {
        if code.name == name_str {
            return Some(code);
        }
    }

    None
}

/// Retrieve a `ScriptCode` via its `name_french` if it exists:
///
/// ```rust
/// use iso15924::get_by_name_french;
///
/// let script = get_by_name_french("arabe");
///
/// get_by_name_french(String::from("aaaaa")).is_none();
/// ```
pub fn get_by_name_french<'a, S: Into<String>>(name: S) -> Option<ScriptCode<'a>> {
    let name_str: &str = &name.into()[..];

    for code in all() {
        if code.name_french == name_str {
            return Some(code);
        }
    }

    None
}

/// Retrieve a `ScriptCode` via its `num` if it exists:
///
/// ```rust
/// use iso15924::get_by_num;
///
/// let script = get_by_num("070");
///
/// get_by_num("000").is_none();
/// ```
pub fn get_by_num<'a, S: Into<String>>(num: S) -> Option<ScriptCode<'a>> {
    let num_str: &str = &num.into()[..];

    for code in all() {
        if code.num == num_str {
            return Some(code);
        }
    }

    None
}
