#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use std::net::SocketAddr;

use clap::{App, Arg, SubCommand};
use minutiae;
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init_custom_env("MINUTIAE_LOG_LEVEL");

    let rt = App::new("minutiæ")
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
        .subcommand(
            SubCommand::with_name("stat").about("inspect data set").arg(
                Arg::with_name("path")
                    .help("path of the dataset to inspect")
                    .takes_value(true)
                    .index(1)
                    .required(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("bbox")
                .about("calculate bounding box")
                .arg(
                    Arg::with_name("dataset")
                        .help("dataset path")
                        .takes_value(true)
                        .short("d")
                        .long("data-set")
                        .required(true),
                )
                .arg(
                    Arg::with_name("face")
                        .help("font face")
                        .takes_value(true)
                        .short("f")
                        .long("face")
                        .required(true),
                )
                .arg(
                    Arg::with_name("size")
                        .help("font size")
                        .takes_value(true)
                        .short("s")
                        .long("size")
                        .required(true),
                )
                .arg(
                    Arg::with_name("str")
                        .help("string")
                        .takes_value(true)
                        .index(1)
                        .required(true),
                ),
        )
        .get_matches();

    match rt.subcommand_name() {
        Some("bbox") => {
            let opts = rt.subcommand_matches("bbox").unwrap();

            let ds = minutiae::DataSet::from_file(minutiae::ReadOptions {
                filename: opts.value_of("dataset").unwrap().to_owned(),
                format: minutiae::Format::JSON,
            });

            let bbox = ds.bounding_box(
                opts.value_of("str").unwrap(),
                minutiae::BoundingBoxRenderOptions {
                    face: opts.value_of("face").unwrap().to_owned(),
                    size: opts.value_of("size").unwrap().to_owned(),
                },
            );

            match bbox {
                Some(v) => info!("{:?}", v),
                None => error!("failed"),
            }
        }
        Some("stat") => {
            let path: String;
            let opts = rt.subcommand_matches("stat").unwrap();

            match opts.value_of("path") {
                Some(v) => {
                    path = v.to_owned();

                    let ds = minutiae::DataSet::from_file(minutiae::ReadOptions {
                        filename: path,
                        format: minutiae::Format::JSON,
                    });

                    println!("{:#?}", ds);
                }
                None => error!("no path"),
            }
        }
        Some("server") => {
            let arg = rt.subcommand_matches("server");

            let index_html =
                warp::path::end().and_then(|| minutiae::web::serve_file("index.html", "text/html"));
            let main_js = warp::path("main.js")
                .and_then(|| minutiae::web::serve_file("main.js", "application/javascript"));
            let raphael_js = warp::path("raphael.js").and_then(|| {
                minutiae::web::serve_file("vendor/raphael.min.js", "application/javascript")
            });

            let write = warp::post()
                .and(warp::path!("write"))
                .and(warp::query::<minutiae::WriteOptions>())
                .and(warp::body::json())
                .and_then(minutiae::web::write_dataset);

            let bind_addr: SocketAddr = arg
                .unwrap()
                .value_of("bind")
                .unwrap()
                .parse()
                .expect("Failed to parse bind address");

            warp::serve(index_html.or(main_js).or(raphael_js).or(write))
                .run(bind_addr)
                .await;
        }
        _ => println!(),
    }
}
