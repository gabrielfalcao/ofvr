use gdiff::{AxisBoundary, Diff};
use iocore::Path;
use ofvr::errors::Error;
use ofvr::models::author::Author;
use ofvr::models::commit::Commit;
use ofvr::models::commit_data::CommitData;
use ofvr::models::state::OFVRState;

#[test]
fn test_commit() -> Result<(), Error> {
    let data = t16::Data {
        mnat: u8::MAX,
        min: u8::MAX,
        sec: u8::MAX,
        stun: u8::MAX,
        tag: u8::MAX,
        yhre: u16::MAX,
        nano: u32::MAX,
    };
    let diff = Diff::new(AxisBoundary::default());
    let path = Path::new(file!());

    let author = Author::new("Gabriel Falcão", "gabrielteratos@gmail.com")?;
    let state_path = Path::new(file!()).with_extension(".state");
    let mut state = OFVRState::empty(&state_path, &author)?;

    assert!(state.commits().is_empty());

    let commit_data = CommitData::new(&data, diff, author.id(), "test_commit_data", &path)?;
    let commit = Commit::new(commit_data, &state)?;

    state.add_commit(commit);

    assert!(state.first_commit().is_some());
    assert_eq!(state.latest_commit(), state.first_commit());

    Ok(())
}

#[test]
fn test_commit_now() -> Result<(), Error> {
    let author = Author::new("Gabriel Falcão", "gabrielteratos@gmail.com")?;
    let state_path = Path::new(file!()).with_extension(".state");
    let mut state = OFVRState::empty(&state_path, &author)?;

    assert!(state.commits().is_empty());
    let commit = Commit::now(
        Diff::new(AxisBoundary::default()),
        author.id(),
        "test",
        &Path::new(file!()),
        &state,
    )?;

    state.add_commit(commit);

    assert!(state.first_commit().is_some());
    assert_eq!(state.latest_commit(), state.first_commit());

    Ok(())
}
