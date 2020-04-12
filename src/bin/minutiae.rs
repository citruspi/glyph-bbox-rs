#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use std::net::SocketAddr;

use clap::{App, Arg, SubCommand};
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init_custom_env("MINUTIAE_LOG_LEVEL");

    let m = App::new("minutiæ")
        .version(crate_version!())
        .author("Mihir Singh (@citruspi)")
        .subcommand(
            SubCommand::with_name("server")
                .about("run rendering server")
                .arg(
                    Arg::with_name("bind")
                        .takes_value(true)
                        .short("b")
                        .long("bind")
                        .help("address to bind to")
                        .default_value("127.0.0.1:2352"),
                ),
        )
        .get_matches();

    match m.subcommand_name() {
        Some("server") => {
            let raw_bind_addr: &str;

            match m.value_of("bind") {
                Some(v) => raw_bind_addr = v,
                None => raw_bind_addr = "127.0.0.1:2352",
            }

            let hello = warp::path::end().map(|| format!("hello world"));

            let bind_addr: SocketAddr =
                raw_bind_addr.parse().expect("Failed to parse bind address");

            println!("Listening on http://{}", bind_addr);

            warp::serve(hello).run(bind_addr).await;
        }
        _ => println!(),
    }
}
