use iocore::Path;
use ofvr::*;

#[test]
fn test_commit() -> Result<()> {
    let from_file = path_to_test_file("test_cli_commit.data");
    from_file.write(&u64::MAX.to_ne_bytes())?;

    let conf_path = path_to_test_file("test_cli_commit.conf");
    let ofvr_state_path = path_to_test_file("test_cli_commit.ofvr");

    let conf_opt = ConfOpt {
        command: ConfCommand::Init(ConfInitOpt {
            author_email: String::from("gabrielteratos@gmail.com"),
            author_name: "Gabriel Falcão G DeMoura"
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

    let conf = Conf::load_from_file(&conf_path)?;
    let author = conf.author();

    let state = OFVRState::empty(&ofvr_state_path, &author)?;
    state.store()?;

    let commit_message = format!("first commit");
    go(Cli {
        conf_path: Some(conf_path.clone()),
        command: Command::Commit(CommitOpt {
            from_file,
            commit_message,
            ofvr_state_path: Some(ofvr_state_path),
        }),
    })?;
    Ok(())
}

fn path_to_test_file(name: &str) -> Path {
    let path = Path::new(file!()).with_filename(name).hidden();
    if path.exists() {
        path.delete().expect(&format!("delete {}", &path));
    }
    path
}
