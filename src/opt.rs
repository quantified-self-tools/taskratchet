use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "taskratchet")]
pub struct Opts {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Prints the details of your user account
    #[structopt(name = "user")]
    User,
    #[structopt(name = "tasks")]
    /// Lists your tasks
    Tasks,
    /// Prints a shell completion script
    #[structopt(name = "_completions")]
    Completion,
}

impl Opts {
    pub fn print_completion() {
        let mut stdout = std::io::stdout();
        Opts::clap().gen_completions_to("taskratchet", structopt::clap::Shell::Bash, &mut stdout);
    }
}
