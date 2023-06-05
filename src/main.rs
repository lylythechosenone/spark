use clap::{command, Arg, Command};
mod actions;

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("install").about("install a package").arg(
                Arg::new("package")
                    .required(true)
                    .index(1)
                    .action(clap::ArgAction::Append),
            ),
        )
        // .subcommand(
        //     Command::new("server")
        //         .about("management of spark server")
        //         .arg(Arg::new("start").required(false))
        //         .arg(Arg::new("stop").required(false))
        //         .arg(Arg::new("restart").required(false))
        //         .arg(Arg::new("status").required(false))
        //         .arg(Arg::new("config").required(false)),
        // )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("install") {
        matches
            .get_one::<String>("package")
            .unwrap()
            .split(" ")
            .for_each(|package| {
                actions::install::handle(&mut package.to_string());
            });
    }
}
