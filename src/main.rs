use anyhow::Result;
use hxcmp::{matrix, tests};
// fn main() -> Result<()> {
//     matrix::run()?;
//     Ok(())
// }

use clap::{load_yaml, App};

fn main() -> Result<()> {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    println!("{:?}", matches);

    env_logger::init();

    match matches.subcommand() {
        ("test", Some(sub_m)) => {
            let cli_files = sub_m.values_of("FILE");
            let input = sub_m.value_of("input-file");
            if input.is_some() {
                return Ok(());
            }
            if input.is_none() && cli_files.is_none() {
                println!("Error: no provided input files");
                return Ok(());
            }
            for file in cli_files.unwrap() {
                let data = std::fs::read_to_string(file)?;

                // preview file
                // println!("{}", data.chars().into_iter().take(100).collect::<String>());

                matrix::run_matrix(
                    std::io::stdout(),
                    vec![tests::Test {
                        name: file,
                        input: tests::Input::AString(data),
                    }],
                )?;
            }
        }
        ("builtin", Some(sub_m)) => matrix::run_matrix(std::io::stdout(), tests::get_tests())?,
        _ => println!("No command provided"),
    }
    Ok(())
}
