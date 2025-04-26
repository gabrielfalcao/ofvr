use bt_diff::{AxisBoundary, Diff};
use iocore::Path;
use iocore_test::{path_to_test_file, seq_bytes};
use ofvr::errors::Result;
use ofvr::models::author::Author;
use ofvr::models::commit::Commit;
use ofvr::models::commit_data::CommitData;
use ofvr::state::OFVRState;

fn author() -> Author {
    Author::new("Gabriel DeMoura", "gabrielteratos@gmail.com.com")
}

#[test]
fn test_commit() -> Result<()> {
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

    let author = author();
    let state_path = Path::new(file!()).with_extension(".state");
    let mut state = OFVRState::empty(&state_path, &author)?;

    assert!(state.commits().is_empty());

    let commit_data = CommitData::new(&data, diff, author.id(), "test_commit.-", &path)?;
    let commit = Commit::new(commit_data, &state)?;

    assert_eq!(state.commits().len(), 0);
    state.add_commit(commit)?;
    assert_eq!(state.commits().len(), 1);

    assert!(state.first_commit().is_some());
    assert_eq!(state.latest_commit(), state.first_commit());

    let diff = Diff::new(AxisBoundary::default());
    let path = Path::new(file!());
    let commit_data = CommitData::new(&data, diff, author.id(), "test_commit.--", &path)?;
    let commit = Commit::new(commit_data, &state)?;

    state.add_commit(commit)?;
    assert_eq!(state.commits().len(), 2);
    assert_eq!(state.latest_commit(), Some(state.commits()[1].clone()));

    Ok(())
}

#[test]
fn test_commit_now() -> Result<()> {
    let author = author();
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

    state.add_commit(commit)?;

    assert!(state.first_commit().is_some());
    assert_eq!(state.latest_commit(), state.first_commit());

    Ok(())
}

#[test]
fn test_state_commit() -> Result<()> {
    let author = author();
    let path = path_to_test_file!("test_state_commit.ofvrf").delete()?;
    let mut state = OFVRState::empty(&path, &author)?;
    let from_file = path_to_test_file!("test_state_commit.data");
    from_file.write(&seq_bytes(u16::MAX.into()))?;

    state.commit(&from_file, &author, "Commit N")?;

    assert_eq!(state.commits().len(), 1);

    state.commit(&from_file, &author, "Commit N")?;

    assert_eq!(state.commits().len(), 2);
    Ok(())
}
