// use iocore_test::{path_to_test_file, seq_bytes};
// use ofvr::*;

// #[test]
// fn test_conf() -> Result<()> {
//     let conf_path = path_to_test_file!("test_conf", "conf");

//     let conf_opt = ConfOpt {
//         command: ConfCommand::Init(ConfInitOpt {
//             author_email: String::from("gabrielteratos@gmail.com"),
//             author_name: "Gabriel Falcão G DeMoura"
//                 .split(" ")
//                 .map(|s| String::from(s))
//                 .collect::<Vec<String>>(),
//             overwrite: true,
//         }),
//     };
//     let conf_cli = Cli {
//         conf_path: Some(conf_path.clone()),
//         command: Command::Conf(conf_opt),
//     };

//     go(conf_cli)?;
//     Ok(())
// }


// #[test]
// fn test_commit() -> Result<()> {
//     let from_file = path_to_test_file!("test_commit", "data");
//     from_file.write(&seq_bytes(u16::MAX.into()))?;

//     let conf_path = path_to_test_file!("test_commit", "conf");
//     conf_path.delete()?;

//     let author = Author::new(
//         "Gabriel Falcão G DeMoura",
//         "gabrielteratos@gmail.com",
//     )?;
//     let conf = Conf::new(author.clone());
//     conf.save_to_file(&conf_path)?;

//     let ofvr_state_path = path_to_test_file!("test_commit", "ofvr");
//     ofvr_state_path.delete()?;

//     let commit_message = format!("first commit");

//     go(Cli {
//         conf_path: Some(conf_path.clone()),
//         command: Command::Commit(CommitOpt {
//             from_file,
//             commit_message,
//             ofvr_state_path: Some(ofvr_state_path),
//         }),
//     })?;
//     Ok(())
// }
