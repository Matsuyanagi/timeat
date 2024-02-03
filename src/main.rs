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

    #[arg(short = 'd', long, default_value = "false")]
    datetime_only: bool,

    #[arg(short = 'f', long = "format", default_value = "")]
    format: String,
}

fn main() {
    let cli = Cli::parse();

    if cli.unixtime.len() > 0 {
        // 引数として unixtime が1つ以上渡されたとき

        // 日付フォーマット文字列取得。なければデフォルト
        let default_datetime_format = "%Y-%m-%d %H:%M:%S %z";
        let mut date_time_format = cli.format;
        if date_time_format.is_empty() {
            date_time_format = default_datetime_format.to_string();
        }

        // 
        for ut in cli.unixtime {
            let utc: DateTime<Utc> = DateTime::from_timestamp(ut, 0).unwrap();
            if cli.utc {
                if cli.datetime_only {
                    println!("{}", utc.format(date_time_format.as_str()));
                } else {
                    println!( "unixtime: {} , utc: {}", ut, utc.format(date_time_format.as_str()).to_string() );
                }
            } else {
                let local: DateTime<Local> = utc.with_timezone(&Local);
                if cli.datetime_only {
                    println!("{}", local.format(date_time_format.as_str()).to_string());
                } else {
                    println!( "unixtime: {} , local: {}", ut, local.format(date_time_format.as_str()).to_string() );
                }
            }
        }
    } else {
        // 引数に unixtime が渡されなかったとき
        let dt: DateTime<Local> = Local::now();
        let timestamp: i64 = dt.timestamp();
        // 現在の unixtime を出力する
        println!("{}", timestamp);
    }

    ()
}
