use bitvec::BitArr;
use regex::Regex;

use crate::UNICODE_VERSION;

pub type CodepointBitArr = BitArr!(for 0x110000);

/// Download the specified Unicode data file from the Unicode website,
/// using the version specified in [`UNICODE_VERSION`].
fn fetch_unicode_file(file: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(reqwest::blocking::get(format!(
        "https://www.unicode.org/Public/{}.{}.{}/ucd/{file}",
        UNICODE_VERSION.0, UNICODE_VERSION.1, UNICODE_VERSION.2
    ))?
    .error_for_status()?
    .text()?)
}

#[derive(Debug)]
pub struct DataFiles {
    pub unicode_data: String,
    pub derived_core_properties: String,
    pub prop_list: String,
    pub scripts: String,
}

/// Retrieve all the data files we need.
pub fn data_files() -> Result<DataFiles, Box<dyn std::error::Error>> {
    Ok(DataFiles {
        unicode_data: fetch_unicode_file("UnicodeData.txt")?,
        derived_core_properties: fetch_unicode_file("DerivedCoreProperties.txt")?,
        prop_list: fetch_unicode_file("PropList.txt")?,
        scripts: fetch_unicode_file("Scripts.txt")?,
    })
}

/// - `arr`: bit array (1 bit per unicode code point)
/// - `data_file`: Unicode data file to look for properties in
/// - `props`: regex matching one or more Unicode properties
/// - `set_to`: what we should set the bits corresponding to matching code points to
pub fn set_by_prop(arr: &mut CodepointBitArr, data_file: &str, props: &str, set_to: bool) {
    let regex_string = format!(r"^([0-9A-F]+)(?:\.\.([0-9A-F]+))?\s*;\s*(?:{props})");
    let regex = Regex::new(&regex_string).unwrap();
    for line in data_file.lines() {
        if let Some(captures) = regex.captures(line) {
            let start = usize::from_str_radix(&captures[1], 16).unwrap();
            let codepoint_range = start
                ..=captures
                    .get(2)
                    .map_or(start, |m| usize::from_str_radix(m.as_str(), 16).unwrap());
            for cp in codepoint_range {
                arr.set(cp, set_to);
            }
        }
    }
}

/// - `arr`: bit array (1 bit per unicode code point)
/// - `props`: regex matching one or more Unicode character categories
/// - `set_to`: what we should set the bits corresponding to matching code points to
pub fn set_by_general_category(
    arr: &mut CodepointBitArr,
    data: &DataFiles,
    categories: &str,
    set_to: bool,
) {
    let regex_string = format!(r"^([0-9A-F]+);(.*?);({categories});");
    let regex = Regex::new(&regex_string).unwrap();
    let mut range_start: Option<(usize, String, String)> = None;
    for line in data.unicode_data.lines() {
        if let Some(captures) = regex.captures(line) {
            let cp = usize::from_str_radix(&captures[1], 16).unwrap();

            if let Some((range_start_cp, prefix, category)) = range_start {
                assert_eq!(captures[2].strip_suffix(", Last>"), Some(prefix).as_deref());
                assert_eq!(category, &captures[3]);
                range_start = None;
                for cp_within_range in range_start_cp..=cp {
                    arr.set(cp_within_range, set_to);
                }
            } else if let Some(prefix) = captures[2].strip_suffix(", First>") {
                assert!(range_start.is_none());
                range_start = Some((cp, prefix.to_owned(), captures[3].to_owned()));
            } else {
                assert!(range_start.is_none());
                arr.set(cp, set_to);
            }
        } else {
            assert!(range_start.is_none());
        }
    }
    assert!(range_start.is_none());
}
