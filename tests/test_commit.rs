use iocore_test::{path_to_test_file, seq_bytes};
use ofvr::errors::Result;
use ofvr::models::author::Author;
use ofvr::state::OFVRState;
use ofvr::traits::PlainBytes;

fn load_author() -> Author {
    let path = path_to_test_file!("test_models_commit.auth");
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
    Author::new("Gabriel DeMoura", "gabrielteratos@gmail.com")
}

#[test]
fn test_state_commit() -> Result<()> {
    let author = load_author();
    let path = path_to_test_file!("test_state_commit.ofvrf").delete()?;

    let mut state = OFVRState::empty(&path, &author)?;
    let from_file = path_to_test_file!("test_state_commit.data");
    from_file.write(&seq_bytes(u16::MAX.into()))?;

    state.commit(&from_file, &author, "Commit A")?;

    assert_eq!(state.commits().len(), 1);
    from_file.write(&seq_bytes(u8::MAX.into()))?;

    state.commit(&from_file, &author, "Commit 9")?;

    assert_eq!(state.commits().len(), 2);
    Ok(())
}
