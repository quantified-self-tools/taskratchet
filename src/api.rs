use crate::config::Config;
use crate::opt::{Command, Opts};
use crate::types::User;
use failure::Fallible;

const USERID_HEADER: &str = "X-Taskratchet-Userid";
const TOKEN_HEADER: &str = "X-Taskratchet-Token";

const BASE_URL: &str = "https://us-central1-taskratchet.cloudfunctions.net/api1";

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

    let url = format!("{}/users/{}", BASE_URL, config.user_id);

    let mut res = client
        .get(&url)
        .header(USERID_HEADER, config.user_id)
        .header(TOKEN_HEADER, config.token)
        .send()?;

    let user: User = res.json()?;

    println!("{}", serde_json::to_string_pretty(&user)?);
    Ok(())
}

fn run_tasks() -> Fallible<()> {
    let client = reqwest::Client::new();
    let config = Config::get()?;

    let url = format!("{}/users/{}/tasks", BASE_URL, config.user_id);

    let mut res = client
        .get(&url)
        .header(USERID_HEADER, config.user_id)
        .header(TOKEN_HEADER, config.token)
        .send()?;

    // Leaving this as raw json for now.
    let tasks: serde_json::Value = res.json()?;

    println!("{}", serde_json::to_string_pretty(&tasks)?);
    Ok(())
}
