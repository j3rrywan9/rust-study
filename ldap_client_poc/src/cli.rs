use clap::{command, Arg, ArgAction, Command};

pub fn setup_cli() -> Command {
    command!()
        .arg(
            Arg::new("url")
                .long("url")
                .required(true)
                .action(ArgAction::Set)
                .help("The LDAP server URL"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .required(true)
                .action(ArgAction::Set)
                .help("The LDAP server port"),
        )
        .arg(
            Arg::new("common_name")
                .long("cn")
                .required(true)
                .action(ArgAction::Set)
                .help("The common name used to bind (authenticate) to the directory"),
        )
        .arg(
            Arg::new("bind_password")
                .long("bind-password")
                .required(true)
                .action(ArgAction::Set)
                .help("The password used to bind (authenticate) to the directory"),
        )
}
