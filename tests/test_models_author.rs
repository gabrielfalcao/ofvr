// use ofvr::errors::Error;
// use pqpfs::{PlainBytes, EncryptionKey, DecryptionKey, Data};
// use ofvr::models::author::Author;

// #[test]
// fn test_author_new() -> Result<(), Error> {
//     let author = Author::new("Gabriel Falcão", "gabrielfalcao@poems.codes")?;

//     assert_eq!(author.name(), "Gabriel Falcão");
//     assert_eq!(author.email(), "gabrielfalcao@poems.codes");
//     assert_eq!(author.id(), 0xf842);
//     Ok(())
// }

// #[test]
// fn test_author_encrypt() -> Result<(), Error> {
//     let author = Author::new("Gabriel Falcão", "gabrielfalcao@poems.codes")?;

//     let data = author.to_bytes();
//     assert_eq!(Author::from_bytes(&data)?, author);
//     let ciphertext = author.encrypt(data.iter()).expect("encrypt");
//     let plaintext = author.decrypt(ciphertext.iter()).expect("decrypt");
//     assert_eq!(Data::from(data), plaintext);
//     assert_eq!(Author::from_bytes(&plaintext.to_vec())?, author);
//     Ok(())
// }
