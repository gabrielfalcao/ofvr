#![allow(unused)]
use iocore::Path;
use ofvr::{Author, OFVRState};
use ofvr::traits::{PlainBytes};

fn get_tests_path() -> Path {
    Path::new(file!())
        .try_canonicalize()
        .parent()
        .expect("./tests/")
}

fn test_file_path(name: &str) -> Path {
    get_tests_path().join(name)
}
fn read_test_file_path(name: &str) -> Vec<u8> {
    test_file_path(name)
        .read_bytes()
        .expect(&format!("read bytes from {}", name))
}

// #[test]
// fn test_new_commit_blob() {
//     let author =
//         Author::new("Testy McTesterson", "testymctesterson+qa@noon.noon").expect("author");
//     let mut state =
//         OFVRState::empty(&test_file_path("test.commit.ofvrf"), &author).expect("new state");
//     let commit = state
//         .commit_blob(
//             &read_test_file_path("before-after/target/release/before-after"),
//             &author,
//             "release binary",
//         )
//         .expect("success");
//     assert!(state.latest_commit().is_some())
// }

// #[test]
// fn test_commit_from_file() {
//     let author =
//         Author::new("Testy McTesterson", "testymctesterson+qa@noon.noon").expect("author");
//     let mut state =
//         OFVRState::empty(&test_file_path("test.commits.ofvrf"), &author).expect("new state");
//     state
//         .commit(
//             &test_file_path("before-after/target/debug/before-after"),
//             &author,
//             "debug binary",
//         )
//         .expect("first commit");

//     let first_commit = state.first_commit();
//     assert_eq!(state.first_commit(), first_commit);
//     assert_eq!(state.first_commit(), state.latest_commit());

//     assert!(dbg!(state.to_bytes().len()) >= 43918);
//     assert!(dbg!(state.to_bytes().len()) <= 43922);

//     state
//         .commit(
//             &test_file_path("before-after/target/release/before-after"),
//             &author,
//             "release binary",
//         )
//         .expect("latest commit");
//     assert_eq!(state.latest_commit(), state.first_commit());

//     assert!(dbg!(state.to_bytes().len()) >= 46016);
//     assert!(dbg!(state.to_bytes().len()) <= 46020);
// }
