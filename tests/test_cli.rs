use iocore::Path;
use ofvr::cli::*;
use ofvr::errors::Result;

#[test]
fn test_commit() -> Result<()> {
    let from_file = Path::new(file!());
    let conf_path = from_file.with_extension(".conf");
    let ofvr_state_path = from_file.with_extension(".ofvr");

    let conf_opt = ConfOpt {
        command: ConfCommand::Init(ConfInitOpt {
            author_email: String::from("gabrielteratos@gmail.com"),
            author_name: "Gabriel Falcão"
                .split(" ")
                .map(|s| String::from(s))
                .collect::<Vec<String>>(),
            overwrite: true,
        }),
    };
    let conf_cli = Cli {
        conf_path: Some(conf_path.clone()),
        command: Command::Conf(conf_opt),
    };

    go(conf_cli)?;
    let commit_message = format!("first commit");
    let commit_opt = CommitOpt {
        from_file,
        commit_message,
        ofvr_state_path: Some(ofvr_state_path),
    };
    let commit_cli = Cli {
        conf_path: Some(conf_path.clone()),
        command: Command::Commit(commit_opt),
    };
    go(commit_cli)?;
    Ok(())
}
