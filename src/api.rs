use crate::config::Config;
use crate::opt::{Command, Opts};
use crate::types::User;
use failure::Fallible;

pub fn run(opt: Opts) -> Fallible<()> {
    match opt.command {
        Command::User { .. } => run_user(),
        Command::Tasks { .. } => run_tasks(),
        Command::Completion => Ok(Opts::print_completion()),
    }
}

fn run_user() -> Fallible<()> {
    let client = reqwest::Client::new();
    let config = Config::get()?;

    let url = format!(
        "https://us-central1-taskratchet.cloudfunctions.net/api1/users/{}",
        config.user_id
    );

    let mut res = client.post(&url).form(&config).send()?;

    let user: User = res.json()?;

    println!("{}", serde_json::to_string_pretty(&user)?);
    Ok(())
}

fn run_tasks() -> Fallible<()> {
    let client = reqwest::Client::new();
    let config = Config::get()?;

    let url = format!(
        "https://us-central1-taskratchet.cloudfunctions.net/api1/users/{}/tasks",
        config.user_id
    );

    let mut res = client.post(&url).form(&config).send()?;

    // Leaving this as raw json for now.
    let tasks: serde_json::Value = res.json()?;

    println!("{}", serde_json::to_string_pretty(&tasks)?);
    Ok(())
}
