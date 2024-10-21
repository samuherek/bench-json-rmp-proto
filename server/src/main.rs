use bytes::Bytes;
use common::{command_proto, Command};
use prost::Message;
use warp::Filter;

#[tokio::main]
async fn main() {
    let json_route = warp::path("json")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_json);

    let msgpack_route = warp::path("msgpack")
        .and(warp::post())
        .and(warp::body::bytes())
        .and_then(handle_msgpack);

    let protobuf_route = warp::path("protobuf")
        .and(warp::post())
        .and(warp::body::bytes())
        .and_then(handle_protobuf);

    let routes = json_route.or(msgpack_route).or(protobuf_route);

    println!("Server running on http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_json(cmd: Command) -> Result<impl warp::Reply, warp::Rejection> {
    // println!("Received JSON: {:?}", cmd);
    Ok(warp::reply::with_status(
        "JSON Received",
        warp::http::StatusCode::OK,
    ))
}

async fn handle_msgpack(bytes: Bytes) -> Result<impl warp::Reply, warp::Rejection> {
    let cmd: Command = match rmp_serde::from_slice(&bytes) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("MessagePack deserialization error: {:?}", e);
            return Ok(warp::reply::with_status(
                "Invalid MessagePack",
                warp::http::StatusCode::BAD_REQUEST,
            ));
        }
    };
    // println!("Received MessagePack: {:?}", cmd);
    Ok(warp::reply::with_status(
        "MessagePack Received",
        warp::http::StatusCode::OK,
    ))
}

async fn handle_protobuf(bytes: Bytes) -> Result<impl warp::Reply, warp::Rejection> {
    let cmd_proto = match command_proto::Command::decode(&*bytes) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Protobuf deserialization error: {:?}", e);
            return Ok(warp::reply::with_status(
                "Invalid Protobuf",
                warp::http::StatusCode::BAD_REQUEST,
            ));
        }
    };
    // println!("Received Protobuf: {:?}", cmd_proto);
    Ok(warp::reply::with_status(
        "Protobuf Received",
        warp::http::StatusCode::OK,
    ))
}

