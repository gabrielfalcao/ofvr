use iocore_test::path_to_test_file;
use ofvr::errors::Error;
use ofvr::models::author::Author;
use ofvr::models::conf::Conf;
use ofvr::traits::{FileSystemBytes, PlainBytes};

#[test]
fn test_conf_new() -> Result<(), Error> {
    let author = Author::new("Gabriel DeMoura", "gabrielteratos@gmail.com");
    let conf = Conf::new(author);

    assert_eq!(conf.author().name(), "Gabriel DeMoura");
    assert_eq!(conf.author().email(), "gabrielteratos@gmail.com");
    Ok(())
}

#[test]
fn test_conf_to_bytes() -> Result<(), Error> {
    let author = Author::new("Gabriel DeMoura", "gabrielteratos@gmail.com");
    let conf = Conf::new(author);
    let bytes = conf.to_flate_bytes()?;
    assert_eq!(Conf::from_deflate_bytes(&bytes)?, conf);
    Ok(())
}

#[test]
fn test_conf_save_and_load() -> Result<(), Error> {
    let author = Author::new("Gabriel DeMoura", "gabrielteratos@gmail.com");
    let conf = Conf::new(author);
    let path = path_to_test_file!("conf");
    conf.save_to_file(&path)?;

    assert_eq!(path.exists(), true);
    assert_eq!(path.is_file(), true);
    assert_eq!(Conf::load_from_file(&path)?, conf);

    Ok(())
}
