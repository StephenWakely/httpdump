use ansi_term::Colour::{Blue, Green};
use clap::Parser;
use hyper::service::{make_service_fn, service_fn};
use hyper::{body, Body, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 3000)]
    port: u16,

    #[arg(short, long, default_value_t = 200)]
    status_code: u16,

    #[arg(short, long)]
    response: Option<String>,
}

async fn endpoint(
    req: Request<Body>,
    status_code: StatusCode,
    response: &str,
) -> Result<Response<Body>, Infallible> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let headers = req
        .headers()
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_str().unwrap_or("").to_string()))
        .collect::<Vec<_>>();
    let body = body::to_bytes(req.into_body()).await.unwrap();
    let s = String::from_utf8_lossy(&body);

    println!(
        "{} {}",
        Blue.bold().paint("METHOD:\t"),
        Blue.normal().paint(method.as_str())
    );
    println!(
        "{} {}",
        Blue.bold().paint("URI:\t"),
        Blue.normal().paint(uri.to_string())
    );

    println!();
    println!("{}", Blue.bold().paint("HEADERS:"));

    let space = headers
        .iter()
        .map(|(name, _)| name.len())
        .max()
        .unwrap_or(0)
        + 3;
    for (name, value) in &headers {
        println!(
            "{}{:space$}{}",
            Blue.bold().paint(name),
            "",
            Blue.normal().paint(value),
            space = space - name.len(),
        );
    }

    println!();
    println!("{}", Green.bold().paint("BODY:"));
    println!("{}", Green.normal().paint(s));

    for _ in 0..2 {
        println!();
    }

    Ok(Response::builder()
        .status(status_code)
        .body(response.to_string().into())
        .unwrap())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    let response = Arc::new(args.response.unwrap_or_else(|| "OK".to_string()));
    let status_code = StatusCode::from_u16(args.status_code).unwrap();

    let make_svc = make_service_fn(move |_conn| {
        let response = Arc::clone(&response);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let response = Arc::clone(&response);
                async move { endpoint(req, status_code, &response).await }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
