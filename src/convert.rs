use crate::output;
use crate::peg::{self, Build};
use lazy_static;
use uuid::Uuid;

lazy_static! {
    static ref GN_NAMESPACE: Uuid =
        Uuid::new_v5(&Uuid::NAMESPACE_DNS, "globalnames.org".as_bytes());
}

pub fn to_sci_name(snn: peg::SciNameNode, is_test: bool) -> output::SciName {
    let mut parser_version = env!("CARGO_PKG_VERSION").to_string();
    if is_test {
        parser_version = "test_version".to_string();
    }
    let name_string_id = name_id(&snn.verbatim);
    let mut normalized = None;
    let mut quality = 0_i8;
    let mut canonical = None;
    let mut details = None;
    let mut pos = None;
    let mut parsed = false;
    if snn.name.is_some() {
        let ref name = snn.name.unwrap();
        let n = match name {
            peg::NameNode::Uninomial(ref uni) => uni,
        };
        quality = get_quality(snn.warnings);
        parsed = true;
        normalized = Some(n.normalize());
        canonical = Some(n.canonical());
        details = Some(vec![n.details()]);
        pos = Some(n.pos());
    }

    output::SciName {
        parsed: parsed,
        quality: quality,
        verbatim: snn.verbatim,
        normalized: normalized,
        canonical_name: canonical,
        details: details,
        positions: pos,
        name_string_id: name_string_id,
        parser_version: parser_version,
        ..Default::default()
    }
}

fn get_quality(ws: Vec<output::Warning>) -> i8 {
    if ws.len() == 0 {
        return 1;
    }
    2
}

fn name_id(name: &str) -> String {
    let id = Uuid::new_v5(&GN_NAMESPACE, name.as_bytes());
    id.to_string()
}

#[test]
fn correct_uuid_v5() {
    assert_eq!(name_id("Homo"), "89f48cba-d38b-5640-99ba-8dac0dcaf2f8")
}
