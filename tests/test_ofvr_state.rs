use iocore::Path;
use ofvr::OFVRState;

fn get_tests_path() -> Path {
    Path::new(file!()).parent().expect("./tests/")
}

fn new_test_file_path(name: &str) -> Path {
    let path = get_tests_path().join(name);
    path.delete().unwrap_or(path)
}

#[test]
fn test_new_diff() {
    let path = new_test_file_path("new_diff.ofvrf");
    let _state = OFVRState::new_diff(&Path::new(file!()), &path).expect("new state");
    assert_eq!(path.is_file(), true);
    // assert_eq!(state.to_string(), "");
}

#[test]
fn test_update_diff() {
    let path = new_test_file_path("update_diff.ofvrf");
    let mut state = OFVRState::new_diff(&Path::new(file!()), &path).expect("new state from path");
    state
        .update_diff(b"test update_diff")
        .expect("update state's diff");
    assert_eq!(path.is_file(), true);
}
