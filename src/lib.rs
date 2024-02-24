use clap::{Arg, ArgAction, Command};
use std::{error::Error, fmt::Debug};

type MyResult<T> = Result<T, Box<dyn Error + Send + Sync + 'static>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("y-yamada")
        .about("rust head")
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .short('n')
                .long("lines")
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .short('c')
                .long("bytes")
                .conflicts_with("lines"),
        )
        .arg(
            Arg::new("files")
                .action(ArgAction::Append)
                .value_name("FILES")
                .help("Input file name")
                .default_value("-"),
        )
        .get_matches();

    let files: Vec<_> = matches
        .get_many::<String>("files")
        .unwrap_or_default()
        .map(|v| v.to_owned())
        .collect();

    let lines = matches
        .get_one::<String>("lines")
        .map(|v| parse_positive_int(v))
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .get_one::<String>("bytes")
        .map(|v| parse_positive_int(v))
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files,
        lines: lines.expect("default").to_owned(),
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse::<usize>() {
        Ok(v) if v > 0 => Ok(v),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
