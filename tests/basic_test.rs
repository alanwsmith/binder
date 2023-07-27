use std::path::PathBuf;

#[test]
pub fn filter_extensions_test() {
    let files = vec![
        PathBuf::from("/a/b/alfa.org"),
        PathBuf::from("/a/b/bravo.txt"),
    ];
    assert_eq!(
        vec![PathBuf::from("/a/b/alfa.org")],
        filter_extensions(files)
    );
}

#[test]
pub fn filter_status_false() {
    // allowed statuses are hard coded above
    let lines = [
        "",
        "-- attributes",
        "-- id: 12341234",
        "-- status: unpublished",
    ];
    assert_eq!(filter_status(lines.join("\n").as_str()).unwrap().1, false);
}

#[test]
pub fn filter_status_true() {
    let lines = [
        "",
        "-- attributes",
        "-- id: 12341234",
        "-- status: published",
    ];
    assert_eq!(filter_status(lines.join("\n").as_str()).unwrap().1, true);
}

#[test]
pub fn filter_status_with_trailing_space() {
    let lines = [
        "",
        "-- attributes",
        "-- id: 12341234",
        "-- status: published ",
    ];
    assert_eq!(filter_status(lines.join("\n").as_str()).unwrap().1, true);
}

#[test]
pub fn filter_status_with_no_content() {
    let lines = ["", "-- attributes", "-- date: 2023-02-03 13:14:15"];
    assert_eq!(filter_status(lines.join("\n").as_str()).unwrap().1, false);
}

#[test]
pub fn filter_site_test() {
    let lines = [
        "",
        "-- attributes",
        "-- id: 12341234",
        "-- site: neoengine",
        "-- status: published ",
    ];
    assert_eq!(
        filter_site(lines.join("\n").as_str(), "neoengine")
            .unwrap()
            .1,
        true
    );
}

#[test]
pub fn filter_site_test_with_no_attibutes() {
    let lines = ["this is a file with no attributes"];
    assert_eq!(
        filter_site(lines.join("\n").as_str(), "neoengine")
            .unwrap()
            .1,
        false
    );
}

#[test]
pub fn basic_output_dir_name() {
    let source = PathBuf::from("/some/posts/rust- Basic Path Example.neo");
    let id = String::from("1234qwer");
    let expected = Ok(("", "rust-basic-path-example--1234qwer".to_string()));
    let results = output_dir_name(source.file_stem().unwrap().to_str().unwrap(), id.as_str());
    assert_eq!(results, expected);
}

#[test]
pub fn dir_with_dashes_that_are_not_followed_by_a_space() {
    let source = PathBuf::from("alfa-bravo");
    let id = String::from("9876rewq");
    let expected = Ok(("", "alfa-bravo--9876rewq".to_string()));
    let results = output_dir_name(source.file_stem().unwrap().to_str().unwrap(), id.as_str());
    assert_eq!(results, expected);
}

#[test]
pub fn file_id_basic() {
    let lines = ["", "-- attributes", "-- id: 1234alfa"].join("\n");
    assert_eq!(file_id(lines.as_str()).unwrap().1, "1234alfa");
}

#[test]
pub fn file_id_with_trailing_white_space() {
    let lines = [
        "",
        "-- attributes",
        "-- id: 6789bravo ",
        "-- status: published",
        "",
    ]
    .join("\n");
    assert_eq!(file_id(lines.as_str()).unwrap().1, "6789bravo");
}

#[test]
pub fn get_override_path() {
    let lines = ["", "-- attributes", "-- path: index.neo", ""].join("\n");
    assert_eq!(
        override_path(lines.as_str()).unwrap().1,
        Some("index.neo".to_string())
    );
}

#[test]
pub fn valid_noce_test() {
    let name = PathBuf::from("d3- alfa bravo");
    assert_eq!(true, valid_nonce(name));
}

#[test]
pub fn valid_noce_test_skip() {
    let name = PathBuf::from("skipthis- charlie delta");
    assert_eq!(false, valid_nonce(name));
}
