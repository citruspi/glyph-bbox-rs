#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use std::net::SocketAddr;

use minutiae;
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init_custom_env("MINUTIAE_LOG_LEVEL");

    let rt = minutiae::cli_entrypoint().get_matches();

    match rt.subcommand_name() {
        Some(v) => {
            let args = rt.subcommand_matches(v).unwrap();

            match v {
                "bbox" => {
                    let ds = minutiae::DataSet::from_file(minutiae::ReadOptions {
                        filename: args.value_of("dataset").unwrap().to_owned(),
                        format: minutiae::Format::JSON,
                    });

                    let bbox = ds.bounding_box(
                        args.value_of("str").unwrap(),
                        minutiae::BoundingBoxRenderOptions {
                            face: args.value_of("face").unwrap().to_owned(),
                            size: args.value_of("size").unwrap().to_owned(),
                        },
                    );

                    match bbox {
                        Some(v) => info!("{:?}", v),
                        None => error!("failed"),
                    }
                }
                "stat" => {
                    let ds = minutiae::DataSet::from_file(minutiae::ReadOptions {
                        filename: args.value_of("path").unwrap().to_owned(),
                        format: minutiae::Format::JSON,
                    });

                    println!("{:#?}", ds);
                }
                "server" => {
                    let index_html = warp::path::end()
                        .and_then(|| minutiae::web::serve_file("index.html", "text/html"));
                    let main_js = warp::path("main.js").and_then(|| {
                        minutiae::web::serve_file("main.js", "application/javascript")
                    });
                    let raphael_js = warp::path("raphael.js").and_then(|| {
                        minutiae::web::serve_file("vendor/raphael.min.js", "application/javascript")
                    });

                    let write = warp::post()
                        .and(warp::path!("write"))
                        .and(warp::query::<minutiae::WriteOptions>())
                        .and(warp::body::json())
                        .and_then(minutiae::web::write_dataset);

                    let bind_addr: SocketAddr = args
                        .value_of("bind")
                        .unwrap()
                        .parse()
                        .expect("Failed to parse bind address");

                    warp::serve(index_html.or(main_js).or(raphael_js).or(write))
                        .run(bind_addr)
                        .await;
                }
                _ => error!("unrecognized subcommand"),
            }
        }
        None => error!("no subcommand specified"),
    }
}
