use gdiff::{AxisBoundary, Diff};
use iocore::Path;
use ofvr::{
    errors::Error,
    models::{author::Author, commit::Commit, commit_data::CommitData},
};
// use ofvr::{state::OFVRState};
use pqpfs::RSAPrivateKey;

#[test]
fn test_commit_encrypt_and_decrypt_commit_data() -> Result<(), Error> {
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

    let author = Author::new("Gabriel Falcão", "gabrielteratos@gmail.com")?;

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

    assert!(decrypted.is_some());
    assert!(decryption_failed.is_none());
    Ok(())
}
// #[test]
// fn test_commit() -> Result<(), Error> {
//     let data = t16::Data {
//         mnat: u8::MAX,
//         min: u8::MAX,
//         sec: u8::MAX,
//         stun: u8::MAX,
//         tag: u8::MAX,
//         yhre: u16::MAX,
//         nano: u32::MAX,
//     };
//     let diff = Diff::new(AxisBoundary::default());
//     let path = Path::new(file!());

//     let author = Author::new("Gabriel Falcão", "gabrielteratos@gmail.com")?;
//     let state_path = Path::new(file!()).with_extension(".state");
//     let mut state = OFVRState::empty(&state_path, &author)?;

//     assert!(state.commits().is_empty());

//     let commit_data = CommitData::new(&data, diff, author.id(), "test_commit_data", &path)?;
//     let commit = Commit::new(commit_data, &state)?;

//     state.add_commit(commit);

//     assert!(state.first_commit().is_some());
//     assert_eq!(state.latest_commit(), state.first_commit());

//     Ok(())
// }

// #[test]
// fn test_commit_now() -> Result<(), Error> {
//     let author = Author::new("Gabriel Falcão", "gabrielteratos@gmail.com")?;
//     let state_path = Path::new(file!()).with_extension(".state");
//     let mut state = OFVRState::empty(&state_path, &author)?;

//     assert!(state.commits().is_empty());
//     let commit = Commit::now(
//         Diff::new(AxisBoundary::default()),
//         author.id(),
//         "test",
//         &Path::new(file!()),
//         &state,
//     )?;

//     state.add_commit(commit);

//     assert!(state.first_commit().is_some());
//     assert_eq!(state.latest_commit(), state.first_commit());

//     Ok(())
// }
