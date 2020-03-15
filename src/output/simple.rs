use crate::peg;
use csv;
use std::error::Error;

pub fn header() -> String {
    "Id,Verbatim,CanonicalFull,CanonicalSimple,CanonicalStem,Authorship,Year,Quality".to_string()
}

pub fn row(sn: peg::SciName) -> Result<String, Box<dyn Error>> {
    let mut can_full = String::new();
    let mut can_simple = String::new();
    let mut can_stem = String::new();
    let mut year = String::new();
    if sn.canonical_name.is_some() {
        let can = sn.canonical_name.unwrap();
        can_full = can.full;
        can_simple = can.simple;
        can_stem = can.stem;
    }
    if sn.year.is_some() {
        year = sn.year.unwrap();
    }

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&[
        sn.name_string_id.clone(),
        sn.verbatim.clone(),
        can_full,
        can_simple,
        can_stem,
        sn.authorship.unwrap_or("".to_string()).to_string(),
        year,
        sn.quality.to_string(),
    ])?;
    wtr.flush()?;
    let res = String::from_utf8(wtr.into_inner()?)?;
    Ok(res.to_string())
}
