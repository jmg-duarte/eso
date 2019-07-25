use clap::App;

mod cli;
mod vm;
mod io;

fn main() {
    let yml = clap::load_yaml!("cli.yml");
    let matches = App::from_yaml(yml)
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .get_matches();

    cli::run(matches);
}
