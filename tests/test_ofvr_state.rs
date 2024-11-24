use iocore::Path;
use ofvr::OFVRState;

fn get_tests_path() -> Path {
    Path::new(file!()).try_canonicalize().parent().expect("./tests/")
}

fn test_file_path(name: &str) -> Path {
    get_tests_path().join(name)
}
fn read_test_file_path(name: &str) -> pqpfs::Data {
    test_file_path(name)
        .read_bytes()
        .expect(&format!("read bytes from {}", name)).into()
}

#[test]
fn test_empty_commit() {
    let state = OFVRState::empty(&Path::new(file!())).expect("new state");
    assert_eq!(state.latest_commit().is_some(), false);
}

#[test]
fn test_new_commit_blob() {
    let mut state = OFVRState::empty(&test_file_path("test.commit.ofvrf")).expect("new state");
    let commit = state
        .commit_blob(
            read_test_file_path("before-after/target/release/before-after"),
            "Testy McTesterson <testymctesterson@qa.poems.codes>",
            "release binary",
        )
        .expect("new commit");
    assert_eq!(state.latest_commit(), Some(commit));
}

#[test]
fn test_commit_from_file() {
    let mut state = OFVRState::empty(&test_file_path("test.commits.ofvrf")).expect("new state");
    let first_commit = state
        .commit(
            &test_file_path("before-after/target/debug/before-after"),
            "Testy McTesterson <testymctesterson@qa.poems.codes>",
            "debug binary",
        )
        .expect("first commit");
    assert_eq!(state.latest_commit(), Some(first_commit.clone()));
    assert_eq!(state.first_commit(), Some(first_commit.clone()));
    assert_eq!(dbg!(state.to_bytes().expect("bytes").len()) >= 43918, true);
    assert_eq!(dbg!(state.to_bytes().expect("bytes").len()) <= 43922, true);
    let latest_commit = state
        .commit(
            &test_file_path("before-after/target/release/before-after"),
            "Testy McTesterson <testymctesterson@qa.poems.codes>",
            "release binary",
        )
        .expect("latest commit");
    assert_eq!(state.latest_commit(), Some(latest_commit));
    assert_eq!(state.first_commit(), Some(first_commit.clone()));
    assert_eq!(dbg!(state.to_bytes().expect("bytes").len()) >= 46016, true);
    assert_eq!(dbg!(state.to_bytes().expect("bytes").len()) <= 46020, true);
}
