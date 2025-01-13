use iocore::Path;
use iocore_test::{path_to_test_file, seq_bytes};
use ofvr::errors::{Error, Result};
use ofvr::models::author::Author;
use ofvr::models::state::OFVRState;
use pqpfs::traits::PlainBytes;

#[test]
fn test_state() -> Result<()> {
    let author = Author::new("Gabriel Falcão", "gabrielfalcao@poems.codes")?;
    let path = Path::new(file!()).with_extension(".test_state.ofvrf");
    let state = OFVRState::empty(&path, &author)?;
    state.store()?;
    let mut state = OFVRState::from_path(&path)?;

    assert_eq!(&state.path(), &path);

    let author_id: u16 = state.get_author_id(&author)?;
    assert_eq!(state.get_author(author_id)?, author);

    let author_qa = Author::new("Gabriel Falcão", "gabrielfalcao@qa.poems.codes")?;
    let author_qa_id: u16 = state.add_author(&author_qa)?;
    assert_eq!(author_qa_id, 0x0CFF);
    assert_eq!(state.get_author(author_qa_id)?, author_qa);

    let author_staging = Author::new("Gabriel Falcão", "gabrielfalcao@staging.poems.codes")?;
    let author_staging_id: u16 = state.add_author(&author_staging)?;
    assert_eq!(state.get_author(author_staging_id)?, author_staging);

    state.remove_author(author_qa_id)?;
    assert_eq!(
        state.get_author(author_qa_id).err().expect("error"),
        Error::StateError(format!("author {} NOT present in state", author_qa_id))
    );
    assert!(state.commits().is_empty());

    state.store()?;
    assert_eq!(OFVRState::from_path(&path)?, state);

    Ok(())
}

#[test]
fn test_state_commit_blob() -> Result<()> {
    let author = load_author();
    let path = Path::new(file!()).with_extension(".test_state_commit_blob.ofvrf");
    let mut state = OFVRState::empty(&path, &author)?;

    assert!(state.commits().is_empty());
    let data = vec![0, 0, 0, 0, 0, 0, 0];
    let commit = state.commit_blob(&data, &author, "Commit 1")?;

    assert_eq!(state.commits().len(), 1);

    let commit_data = commit.data(&state)?;
    assert_eq!(commit.id.clone(), commit_data.id()?);

    let data = vec![1, 1, 1, 1, 1, 1, 1];
    state.commit_blob(&data, &author, "Commit N")?;

    assert_eq!(state.commits().len(), 2);
    Ok(())
}

#[test]
fn test_state_from_path() -> Result<()> {
    let author = load_author();
    let path = Path::new(file!())
        .canonicalize()?
        .with_filename("test_state_from_path")
        .with_extension("ofvrf")
        .hidden();
    let state = OFVRState::empty(&path, &author)?;
    state.store()?;

    assert_eq!(state, OFVRState::from_path(&path)?);
    Ok(())
}

fn load_author() -> Author {
    let path = path_to_test_file!("test_models_commit", "auth");
    || -> Result<Author> {
        Ok(if path.is_file() {
            Author::from_deflate_bytes(&path.read_bytes()?)?
        } else {
            let author = author();
            path.write(&author.to_flate_bytes()?)?;
            author
        })
    }()
    .unwrap_or_else(|_| author())
}

fn author() -> Author {
    Author::new("Gabriel Falcão", "gabrielfalcao@protonmail.com").expect(&format!("author"))
}

#[test]
fn test_state_commit() -> Result<()> {
    let author = Author::new("Gabriel Falcão", "gabrielfalcao@poems.codes")?;
    let path = path_to_test_file!("test_state_commit", "ofvrf").delete()?;
    let mut state = OFVRState::empty(&path, &author)?;
    let from_file = path_to_test_file!("test_state_commit", "data");
    from_file.write(&seq_bytes(u16::MAX.into()))?;

    state.commit(&from_file, &author, "Commit N")?;

    assert_eq!(state.commits().len(), 1);
    Ok(())
}
