use std::path::PathBuf;
use vcd_rust;

fn get_test_file_path(filename: &str) -> String {
    let mut test_file_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file_buf.push(format!("tests/{}", filename));
    return test_file_buf.as_path().display().to_string();
}

#[test]
fn parse_vcd_file() {
    let test_file = get_test_file_path("no_vardump.golden.vcd");
    let vcd = vcd_rust::load_from_file(test_file).unwrap();

    assert_eq!(vcd.date, "August 9th, 2020");
    assert_eq!(vcd.version, "Version 4.20");
    assert_eq!(
        vcd.comments,
        vec![
            "The golden VCD test file with no dumped vars",
            "Here's another comment for good measure"
        ]
    );
    assert_eq!(
        vcd.timescale,
        vcd_rust::types::timescale::TimeScale::init(1, vcd_rust::types::timescale::TimeUnit::PS)
    )
}
