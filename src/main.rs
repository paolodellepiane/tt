#![warn(clippy::all)]
use commands::Commands;
use prelude::*;
use settings::Settings;

mod commands;
mod history;
mod prelude;
mod select;
mod settings;
mod ssh;
mod teleport;

fn main() -> Result<()> {
    let settings = Settings::new()?;
    match &settings.args.command {
        Some(cmd) => match cmd {
            Commands::Cp(args) => commands::cp(&settings, args),
            Commands::Exec { command } => commands::exec(&settings, command),
            Commands::Code => commands::code(&settings),
            Commands::Get { file } => commands::get_file(&settings, file),
            Commands::Put { file } => commands::put_file(&settings, file),
            Commands::Put { file } => commands::put_file(&settings, file),
            Commands::EventLog => todo!(),
            Commands::Config => commands::append_tsh_to_ssh_config(),
            // Commands::Container { container } => todo!(),
            // Commands::Container { container } => match container {
            //     Container::EventLog => Container::win_container_event_log(hosts),
            //     Container::Vsdbg => Container::vsdbg(hosts),
            //     Container::Get => Container::get_file(hosts),
            //     Container::Put => Container::put_file(hosts),
            //     Container::Exec { command } => Container::exec(command, hosts),
            // },
        },
        None => commands::ssh(&settings),
    }?;
    Ok(())
}
