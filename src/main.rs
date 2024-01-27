use clap::Parser;
// use chrono::prelude::*;
use chrono::{DateTime, Local, Utc};

#[derive(Parser)]
#[command(name="timeat",author,version,about,long_about=None)]
struct Cli {
    // unixtime: Option<u64>,
    unixtime: Vec<i64>,

    #[arg(short = 'u', long, default_value = "false")]
    utc: bool,
}

fn main() {
    let cli = Cli::parse();

    for ut in cli.unixtime {
        let utc: DateTime<Utc> = DateTime::from_timestamp(ut, 0).unwrap();
        if cli.utc {
            println!("unixtime: {} , utc: {}", ut, utc);
        } else {
            let local: DateTime<Local> = utc.with_timezone(&Local);
            println!("unixtime: {} , local: {}", ut, local);
        }
    }

    ()
}
