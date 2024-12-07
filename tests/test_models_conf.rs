use iocore::Path;
use ofvr::errors::Error;
use ofvr::models::author::Author;
use ofvr::models::conf::Conf;
use ofvr::traits::FileSystemBytes;
use pqpfs::PlainBytes;

#[test]
fn test_conf_new() -> Result<(), Error> {
    let author = Author::new("Gabriel Falcão", "gabrielfalcao@poems.codes")?;
    let conf = Conf::new(author);

    assert_eq!(conf.author().name(), "Gabriel Falcão");
    assert_eq!(conf.author().email(), "gabrielfalcao@poems.codes");
    Ok(())
}

#[test]
fn test_conf_to_bytes() -> Result<(), Error> {
    let author = Author::new("Gabriel Falcão", "gabrielfalcao@poems.codes")?;
    let conf = Conf::new(author);
    let bytes = conf.to_flate_bytes()?;
    assert_eq!(Conf::from_deflate_bytes(&bytes)?, conf);
    Ok(())
}

#[test]
fn test_conf_key_path() -> Result<(), Error> {
    let path = Path::new(file!()).with_extension(".conf");
    assert_eq!(path.to_string(), "tests/test_models_conf.conf");
    assert_eq!(
        Conf::key_path(&path).to_string(),
        "tests/.test_models_conf.ky"
    );
    Ok(())
}

#[test]
fn test_conf_save_and_load_key_for_path() -> Result<(), Error> {
    let path = Path::new(file!()).with_extension(".ld");
    let private_key = Conf::save_new_key_for_path(&path)?;
    assert_eq!(Conf::load_key_for_path(&path)?,
               private_key);
    Ok(())
}

#[test]
fn test_conf_save_and_load() -> Result<(), Error> {
    let author = Author::new("Gabriel Falcão", "gabrielfalcao@poems.codes")?;
    let conf = Conf::new(author);
    let path = Path::new(file!()).with_extension(".conf");
    let key_path = Conf::key_path(&path);
    conf.save_to_file(&path)?;

    assert_eq!(key_path.exists(), true);
    assert_eq!(key_path.is_file(), true);
    assert_eq!(path.exists(), true);
    assert_eq!(path.is_file(), true);
    assert_eq!(Conf::load_from_file(&path)?, conf);

    Ok(())
}
