use common::{command_proto, Command, Environment};
use prost::Message;
use reqwest::Client;
use rmp_serde::to_vec as to_msgpack;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let cmd = Command {
        command: "cmd lkajsdflkj".to_string(),
        timestamp: "2024-04-27T12:34:56Z".to_string(),
        timestamp1: "2024-05-27T12:34:56Z".to_string(),
        timestamp2: "2024-06-27T12:34:56Z".to_string(),
        timestamp3: "2024-07-27T12:34:56Z".to_string(),
        user: "johndoe".to_string(),
        environment: Environment {
            shell: "bash".to_string(),
            os: "linux".to_string(),
        },
    };

    println!(
        "siez of struct (without string data): {}",
        std::mem::size_of_val(&cmd)
    );
    println!("------");

    let iterations = 1000;

    let mut json_total_time = 0.0;
    let mut json_comp = 0.0;
    let mut json_total_size = 0;

    for _ in 0..iterations {
        let s = std::time::Instant::now();
        let data = serde_json::to_vec(&cmd).unwrap();
        json_comp += s.elapsed().as_secs_f64() * 1000.0;
        json_total_size += data.len();
        let start = std::time::Instant::now();
        client
            .post("http://127.0.0.1:3030/json")
            .header("Content-Type", "application/json")
            .body(data)
            .send()
            .await
            .unwrap();
        let duration = start.elapsed();
        json_total_time += duration.as_secs_f64() * 1000.0;
    }

    println!("parse avg: {:.4} ms", json_comp / iterations as f64);
    println!("request avg: {:.4} ms", json_total_time / iterations as f64);
    println!("total: {:.1} ms", json_total_time);
    println!("size avg: {:.2} bytes", json_total_size / iterations);
    println!("--------------");

    let mut msgp_total_time = 0.0;
    let mut msgp_comp = 0.0;
    let mut msgp_total_size = 0;

    for _ in 0..iterations {
        let s = std::time::Instant::now();
        let data = to_msgpack(&cmd).unwrap();
        msgp_comp += s.elapsed().as_secs_f64() * 1000.0;
        msgp_total_size += data.len();
        let start = std::time::Instant::now();
        client
            .post("http://127.0.0.1:3030/msgpack")
            .header("Content-Type", "application/octet-stream")
            .body(data)
            .send()
            .await
            .unwrap();
        let duration = start.elapsed();
        msgp_total_time += duration.as_secs_f64() * 1000.0;
    }

    println!("parse avg: {:.4} ms", msgp_comp / iterations as f64);
    println!("request avg: {:.4} ms", msgp_total_time / iterations as f64);
    println!("total: {:.1} ms", msgp_total_time);
    println!("size avg: {:.2} bytes", msgp_total_size / iterations);
    println!("--------------");

    let mut proto_total_time = 0.0;
    let mut proto_comp = 0.0;
    let mut proto_total_size = 0;

    for _ in 0..iterations {
        let s = std::time::Instant::now();
        let mut temp = command_proto::Command::default();
        temp.command = cmd.command.clone();
        temp.timestamp = cmd.timestamp.clone();
        temp.user = cmd.user.clone();
        temp.environment = Some(command_proto::Environment {
            shell: cmd.environment.shell.clone(),
            os: cmd.environment.os.clone(),
        });
        let data = temp.encode_to_vec();

        proto_comp += s.elapsed().as_secs_f64() * 1000.0;
        proto_total_size += data.len();
        let start = std::time::Instant::now();
        client
            .post("http://127.0.0.1:3030/protobuf")
            .header("Content-Type", "application/octet-stream")
            .body(data)
            .send()
            .await
            .unwrap();
        let duration = start.elapsed();
        proto_total_time += duration.as_secs_f64() * 1000.0;
    }

    println!("parse avg: {:.4} ms", proto_comp / iterations as f64);
    println!(
        "request avg: {:.4} ms",
        proto_total_time / iterations as f64
    );
    println!("total: {:.1} ms", proto_total_time);
    println!("size avg: {:.2} bytes", proto_total_size / iterations);
    println!("--------------");
}
