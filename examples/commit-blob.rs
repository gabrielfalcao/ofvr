use iocore::Path;
use ofvr::{Author, OFVRState, Result};

fn main() -> Result<()> {
    let author = Author::new("Gabriel Falcão", "gabrielfalcao@poems.codes")?;
    let path = Path::new(file!()).with_extension(".ofvrf");
    let mut state = OFVRState::empty(&path, &author)?;

    assert!(state.commits().is_empty());
    let data = vec![0, 0, 0, 0, 0, 0, 0];
    state.commit_blob(&data, &author, "Commit 1")?;

    assert_eq!(state.commits().len(), 1);
    let data = vec![1, 1, 1, 1, 1, 1, 1];
    state.commit_blob(&data, &author, "Commit N")?;

    assert_eq!(state.commits().len(), 2);
    Ok(())
}
