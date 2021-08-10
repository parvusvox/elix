// cli arg parser
use clap::{Arg, App};

pub fn build_arg_parser(version:&str) -> App{
    App::new("Elix")
        .version(version)
        .author("Ian Kim <ian@ianmkim.com>")
        .about("A small, fast, and dirty file transfer utility")
        .arg(Arg::new("chunk-size")
            .short('c')
            .long("chunk-size")
            .value_name("CHUNK SIZE")
            .about("determines the chunk size when breaking up large files (default: 256KB)")
            .takes_value(true))
        .subcommand(App::new("send")
            .about("Sends a file using Elix")
            .arg(Arg::new("filename")
                .index(1)
                .required(true)
                .about("A relative path to the file you want to send")))
        .subcommand(App::new("take")
            .about("Receive a file using Elix given a code")
            .arg(Arg::new("code")
                .index(1)
                .required(true)
                .about("A code that was given when sending a file")))
}