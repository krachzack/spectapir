mod legal;
mod listen;

use clap::{crate_authors, crate_name, crate_version, App, Arg};

fn main() {
    let authors: &str = &format!(
        "{}\n\n{}",
        crate_authors!(),
        legal::thanks()
    );
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(authors)
        .arg(
            Arg::with_name("license")
                .long("license")
                .help(&format!(
                    "Displays the GPLv3 under which this \
                        program is licensed, we well as attributions \
                        for external code used by {}",
                    crate_name!()
                ))
                .conflicts_with_all(&["authors", "warranty"]),
        )
        .arg(
            Arg::with_name("authors")
                .long("authors")
                .help(&format!(
                    "Shows attributions for external code used by {}",
                    crate_name!()
                ))
                .conflicts_with_all(&["license", "warranty"]),
        )
        .arg(
            Arg::with_name("warranty")
                .long("warranty")
                .help("Shows warranty information")
                .conflicts_with_all(&["license", "authors"]),
        )
        .get_matches();

    if matches.is_present("license") {
        println!("{}\n\n{}", legal::license(), legal::thanks());
    } else if matches.is_present("authors") {
        println!("{}", legal::thanks());
    } else if matches.is_present("warranty") {
        println!(
            "Excerpt of the GNU General Public License v3.0, start with `--license'\n\
            for additional details.\n\
            \n{}",
            legal::warranty()
        );
    } else {
        // normal application flow, maybe build a config struct
    }
}
