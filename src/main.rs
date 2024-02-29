use std::{path::PathBuf};
use color_eyre::eyre::Result;
use clap::{arg, command, value_parser};
use pam::Authenticator;
use rpassword::prompt_password;
use duct::cmd;

fn main() -> Result<()> {
    let matches = command!().args(&[
        arg!(command: <COMMAND> ... "Command to run").required(false).value_parser(value_parser!(String)),
        arg!(user: -u --user <USER> "User to run command as").required(false).value_parser(value_parser!(String)),
        //arg!(config: -C --config [config] "Path to config file").required(false).value_parser(value_parser!(PathBuf)),
    ])
    .get_matches(); 
    
    /*
    let default_config = PathBuf::from("/etc/aldoas.conf");
    let config = matches.get_one::<PathBuf>("config").unwrap_or(&default_config).to_owned();
    */

    let command  = matches.get_many::<String>("command")
        .unwrap_or_default()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
        .join(" ");

    let default_user = format!("root");
    let user = matches
        .get_one::<String>("user")
        .unwrap_or(&default_user)
        .to_owned();
    
    // Run Command as User
    let mut authenticator = Authenticator::with_password("aldoas")?; // Create a new authenticator

    authenticator
        .get_handler()
        .set_credentials(user, prompt_password("Your password: ")?);

    if authenticator.authenticate().is_ok() && authenticator.open_session().is_ok() {
        cmd!(command.as_str()).run()?;
    }

    Ok(())
}
    
