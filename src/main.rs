use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::collections::HashMap;
use dotenv;
use std::time::Duration;


fn main() {
    dotenv::dotenv().ok();

    let address_string = std::env::var("HOST").expect("HOST key not found in .env");
    let port_string = std::env::var("PORT").expect("Could not get PORT");
    let bind_string = format!("{}:{}", address_string, port_string);
    let listener = TcpListener::bind(bind_string).expect("TcpListener could not bind to value provided by HOST var");

    for stream in listener.incoming().flatten() {
        receive_event(stream);
    }

}

fn receive_event(mut stream: TcpStream) {

    let mut buffer = vec![];
    stream.read_to_end(&mut buffer).unwrap();
    let post_req = b"POST / HTTP/1.1\r\n";
    if buffer.starts_with(post_req) {


        let github_req = String::from_utf8(buffer).unwrap();

        let res = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap();

        let github_req = github_req.split("\r\n\r\n").collect::<Vec<_>>();

        
        let body: serde_json::Value = serde_json::from_str(github_req[1]).unwrap();

        let header_string = github_req[0].split("\n").collect::<Vec<_>>()[1..].join("\n");
        
        let mut headers_buffer = [httparse::EMPTY_HEADER; 25];
        let _parsed_headers_result = httparse::parse_headers(header_string.as_bytes(), &mut headers_buffer).unwrap();
        
        
        for header in headers_buffer.iter() {
            if header.name == "X-Github-Event" {
                let event = std::str::from_utf8(header.value).unwrap();
                match event {
                    "issues" => {
                        //body["issue"]["user"]["login"] may need to be body["sender"]["login"]
                        let alert_message = format!("{} {} issue {} in {} repo", body["issue"]["user"]["login"], body["action"], body["issue"]["title"], body["repository"]["full_name"]);
                        let alert_message = alert_message.replace("\"", "");
                        post_alert(alert_message);
                    }

                    "push" => {
                        let alert_message = format!("{} pushed changes to {}", body["pusher"]["name"], body["repository"]["name"]);
                        let alert_message = alert_message.replace("\"", "");
                        post_alert(alert_message);

                    }

                    "pull_request" => {
                        let alert_message = format!("{} {} a pull request on {} repo",  body["sender"]["login"], body["action"], body["repository"]["full_name"]);
                        let alert_message = alert_message.replace("\"", "");
                        post_alert(alert_message);
                    }
                    _ => {
                        println!("{:#?}", body);
                        println!("Event: {}", event);
                    }
                }
            }
        }

    }
}

fn post_alert(msg: String) {
    let oauth = std::env::var("STREAMLABS_OAUTH").expect("STREAMLABS_OAUTH not found in .env");
    let mut map = HashMap::new();
    map.insert("access_token", &oauth[..]);
    map.insert("type", "donation");
    map.insert("message", &msg);
    map.insert("duration", "5000");

    let time_out = Duration::from_secs(3);
    
    let client = reqwest::blocking::Client::builder().timeout(time_out).build().unwrap();
    let res = client.post("https://streamlabs.com/api/v1.0/alerts")
        .json(&map)
        .send();

    if res.unwrap().status().is_success() {
        println!("Success");
    }

}
