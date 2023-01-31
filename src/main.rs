use std::fs::{File, OpenOptions};
use std::io::{self, Read, ErrorKind, Write};

use hyper::{Body, Method, Request, Response, Server};
use hyper::body::HttpBody;
use hyper::service::{make_service_fn, service_fn};

async fn handle_reqs(req: Request<Body>) -> Result<Response<Body>, io::Error> {
    match req.method() {
        &Method::POST => {
            let mut buffer = Vec::new();
            let mut body = Vec::new();
            req.body().read_to_end(&mut body).unwrap();
            while let Some(chunk) = &body.data().await {
                let chunk_res = match chunk{
                    Ok(chunk) => chunk,
                    Err(e) => return Err(std::io::Error::new(ErrorKind::Other, e)),
                };
                buffer.extend_from_slice(&chunk_res);
            }
            let mut file = File::create("vid.mp4")?;
            file.write_all(&buffer).unwrap();
            Ok(Response::new(Body::from("Video Uploaded to https://breath.pileatedpixels.com/")))
        }
        &Method::GET => {
            let path = "vid.mp4";
            let mut file = match OpenOptions::new().read(true).open(path) {
                Ok(file) => file,
                Err(e) =>{
                    if e.kind() == ErrorKind::NotFound {
                        return Ok(Response::builder().status(404).body(Body::from("Video Unavailable")).unwrap());
                    } else {
                        return Err(e);
                    }
                }
            };
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            Ok(Response::new(Body::from(buffer)))
        }
        _ => Ok(Response::new(Body::from("")))
    }
}

#[tokio::main]
async fn main() {
    let addr = ([66, 94, 127, 226], 1691).into();
    let make_service = make_service_fn(|_| async { Ok::<_, io::Error>(service_fn(handle_reqs)) });
    let server = Server::bind(&addr).serve(make_service);
    println!("Listening...");
    server.await.unwrap();
}
