use gdiff::{AxisBoundary, Diff};
use iocore::Path;
use iocore_test::{path_to_test_file, seq_bytes};
use ofvr::errors::{Error, Result};
use ofvr::models::author::Author;
use ofvr::models::commit::Commit;
use ofvr::models::commit_data::CommitData;
use ofvr::state::OFVRState;
use pqpfs::{PlainBytes, RSAPrivateKey};

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
fn test_commit_encrypt_and_decrypt_commit_data() -> Result<()> {
    let data = t16::Data {
        mnat: u8::MAX,
        min: u8::MAX,
        sec: u8::MAX,
        stun: u8::MAX,
        tag: u8::MAX,
        yhre: u16::MAX,
        nano: u32::MAX,
    };
    let mut diff = Diff::new(AxisBoundary::default());
    let path = Path::new(file!());

    let author = load_author();

    let commit_data = CommitData::new(
        &data,
        diff.clone(),
        author.id(),
        "test_commit_encrypt_and_decrypt_commit_data",
        &path,
    )?;
    diff.update(&vec![0, 1])?;
    let commit_data_01 = CommitData::new(
        &data,
        diff,
        author.id(),
        "test_commit_encrypt_and_decrypt_commit_data_01",
        &path,
    )?;

    let private_key = RSAPrivateKey::generate()?;
    let public_key = private_key.public_key();
    let private_key_01 = RSAPrivateKey::generate()?;
    let public_key_01 = private_key_01.public_key();

    let encrypted = Commit::encrypt_commit_data(&public_key, &commit_data)?;
    let encrypted_01 = Commit::encrypt_commit_data(&public_key_01, &commit_data_01)?;
    let decrypted = Commit::decrypt_commit_data(&private_key, &encrypted);
    let decryption_failed = Commit::decrypt_commit_data(&private_key, &encrypted_01);

    assert!(decrypted.is_ok());
    assert!(decryption_failed.is_err());
    Ok(())
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

    let author = load_author();
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
    let author = load_author();
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
    let author = load_author();
    let path = path_to_test_file!("test_state_commit", "ofvrf").delete()?;
    let mut state = OFVRState::empty(&path, &author)?;
    let from_file = path_to_test_file!("test_state_commit", "data");
    from_file.write(&seq_bytes(u16::MAX.into()))?;

    state.commit(&from_file, &author, "Commit N")?;

    assert_eq!(state.commits().len(), 1);

    state.commit(&from_file, &author, "Commit N")?;

    assert_eq!(state.commits().len(), 2);
    Ok(())
}
