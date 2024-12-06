use gdiff::{AxisBoundary, Diff};
use iocore::Path;
use ofvr::errors::Error;
use ofvr::models::author::Author;
use ofvr::models::commit_data::CommitData;
use ofvr::models::state::OFVRState;

#[test]
fn test_commit_data() -> Result<(), Error> {
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
    let state = OFVRState::empty(&state_path, &author)?;

    let commit_data = CommitData::new(&data, diff, author.id(), "test_commit_data", &path)?;

    assert_eq!(commit_data.date_rfc2822(), "Fri, 7 Mar 5541 15:15:15 +0000");
    assert_eq!(commit_data.date_rfc3339(), "5541-03-07T15:15:15.294967299+00:00");

    assert_eq!(commit_data.author(&state)?, author);
    Ok(())
}
