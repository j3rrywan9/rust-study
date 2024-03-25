mod cli;

use clap::error::ErrorKind;
use clap::ArgMatches;
use ldap3::{self, LdapConnAsync, Scope, SearchEntry};

async fn handle_cli(version: &'static str) -> Result<(), i32> {
    let cli = cli::setup_cli();

    match cli.arg_required_else_help(true).try_get_matches() {
        Ok(matches) => handle_matches(&matches).await,
        Err(ref err) => match err.kind() {
            ErrorKind::DisplayHelp => {
                let _ = err.print();
                Ok(())
            }
            ErrorKind::DisplayVersion => {
                println!("ldap_client_poc {}", version);
                println!();
                Ok(())
            }
            _ => err.exit(),
        },
    }
}

async fn handle_matches(matches: &ArgMatches) -> Result<(), i32> {
    let url = matches.get_one::<String>("url").unwrap();
    let port = matches.get_one::<String>("port").unwrap();
    let common_name = matches.get_one::<String>("common_name").unwrap();
    let bind_password = matches.get_one::<String>("bind_password").unwrap();

    if let Ok((connection, mut ldap)) =
        LdapConnAsync::new(format!("{}:{}", url, port).as_str()).await
    {
        ldap3::drive!(connection);

        if ldap
            .simple_bind(
                format!("cn={},ou=Service Accounts,dc=corp,dc=qc", common_name).as_str(),
                bind_password,
            )
            .await
            .is_ok()
        {
            if let Ok(result) = ldap
                .search(
                    "ou=Domain Users,dc=corp,dc=qc",
                    Scope::Subtree,
                    "(sAMAccountName=jerry.wang)",
                    vec!["memberof"],
                )
                .await
            {
                if let Ok((result_entries, _)) = result.success() {
                    for result_entry in result_entries {
                        for value in SearchEntry::construct(result_entry).attrs.values() {
                            for e in value {
                                println!("{e}");
                            }
                        }
                    }

                    if ldap.unbind().await.is_ok() {
                        Ok(())
                    } else {
                        return Err(1);
                    }
                } else {
                    return Err(1);
                }
            } else {
                return Err(1);
            }
        } else {
            return Err(1);
        }
    } else {
        return Err(1);
    }
}

#[tokio::main]
async fn main() {
    let result = handle_cli(clap::crate_version!()).await;

    if let Err(exit_code) = result {
        std::process::exit(exit_code)
    }
}
