use ofvr::models::author::Author;
use ofvr::errors::Error;

#[test]
fn test_author_new() -> Result<(), Error> {
    let author = Author::new("Gabriel Falcão", "gabrielfalcao@poems.codes")?;

    assert_eq!(author.name(), "Gabriel Falcão");
    assert_eq!(author.email(), "gabrielfalcao@poems.codes");
    Ok(())
}


#[test]
fn test_author_encrypt() -> Result<(), Error> {
    let author = Author::new("Gabriel Falcão", "gabrielfalcao@poems.codes")?;

    let data = bincode::serialize(&author)?;
    assert_eq!(bincode::deserialize::<Author>(&data)?, author);
    let ciphertext = author.encrypt(&data)?;
    let plaintext = author.decrypt(&ciphertext)?;
    assert_eq!(data, plaintext);
    assert_eq!(bincode::deserialize::<Author>(&plaintext)?, author);
    Ok(())
}
