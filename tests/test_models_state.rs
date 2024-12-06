use iocore::Path;
use ofvr::errors::Error;
use ofvr::models::author::Author;
use ofvr::models::state::OFVRState;

#[test]
fn test_state() -> Result<(), Error> {
    let author = Author::new("Gabriel Falcão", "gabrielfalcao@poems.codes")?;
    let path = Path::new(file!()).with_extension(".state");
    let mut state = OFVRState::empty(&path, &author)?;

    let author_id: u16 = state.get_author_id(&author)?;
    assert_eq!(state.get_author(author_id)?, author);

    let author_qa = Author::new("Gabriel Falcão", "gabrielfalcao@qa.poems.codes")?;
    let author_qa_id: u16 = state.add_author(&author_qa)?;
    assert_eq!(author_qa_id, 0x0cff);
    assert_eq!(state.get_author(author_qa_id)?, author_qa);

    let author_staging = Author::new("Gabriel Falcão", "gabrielfalcao@staging.poems.codes")?;
    let author_staging_id: u16 = state.add_author(&author_staging)?;
    assert_eq!(state.get_author(author_staging_id)?, author_staging);

    state.remove_author(author_qa_id)?;
    assert_eq!(
        state.get_author(author_qa_id).err().expect("error"),
        Error::StateError(format!("author {} NOT present in state", author_qa_id))
    );
    Ok(())
}
