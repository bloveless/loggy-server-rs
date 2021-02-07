use actix_web::middleware::Logger;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use chrono::DateTime;
use mongodb::bson::{self, doc};
use mongodb::Client;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
enum Stream {
    StdErr,
    StdOut,
}

#[derive(Deserialize, Debug, Clone)]
struct LogLine {
    datetime: String,
    stream: Stream,
    container: String,
    namespace: String,
    message: String,
}

#[post("/collect")]
async fn collect(log_line: web::Json<LogLine>, client: web::Data<Client>) -> impl Responder {
    println!("Log line: {:?}", log_line);

    let date_time = DateTime::parse_from_rfc3339(&log_line.datetime).unwrap();
    println!("Parsed DateTime: {:?}", date_time);

    println!("Message: {}", log_line.message);

    let mut document = doc! {
        "datetime": &log_line.datetime,
        "stream": match &log_line.stream {
            Stream::StdErr => "stderr",
            Stream::StdOut => "stdout",
        },
        "container" : &log_line.container,
        "namespace": &log_line.namespace,
    };

    match serde_json::from_str::<serde_json::Map<String, Value>>(log_line.message.as_str()) {
        Ok(value) => {
            document.insert("message", bson::ser::to_document(&value).unwrap());
        }
        Err(_) => {
            document.insert("message", &log_line.message);
        }
    };

    let db = client.database("logs");
    let coll = db.collection("log-collection");

    let result = match coll.insert_one(document, None).await {
        Ok(result) => result,
        Err(_) => panic!("Unable to insert into collection"),
    };

    println!("Result id: {}", result.inserted_id);

    HttpResponse::Ok().body("Received log message")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = match Client::with_uri_str("mongodb://admin:mongodb@localhost:27017/").await {
        Ok(client) => client,
        Err(_) => panic!("Unable to connect to mongodb"),
    };
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(client.clone())
            .wrap(Logger::default())
            .service(collect)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
