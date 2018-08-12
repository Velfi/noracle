#![feature(rust_2018_preview)]
#![feature(nll)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate chrono;
extern crate env_logger;
extern crate futures;
extern crate json;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

use self::lfm::LogFileManager;
use actix_web::{
    http, middleware, server, App, AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse,
};
use chrono::prelude::*;
use futures::Future;
use std::fmt::{self, Debug};
use std::fs::OpenOptions;
use std::io::Write;
use uuid::Uuid;

/// This handler uses `HttpRequest::json()` for loading json object.
fn index(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.json()
        .from_err()  // convert all errors into `Error`
        .and_then(|val: PredictionEvent| {
            println!("model: {:?}", val);
            Ok(HttpResponse::Ok().json(val))  // <- send response
        })
        .responder()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // let sys = actix::System::new("json-example");

    let mut prediction_events_log: LogFileManager<PredictionEvent> = match LogFileManager::new(&"log.txt")
    {
        Ok(lfm) => lfm,
        Err(e) => {
            println!("Could not create new lfm: {:?}", e);
            panic!();
        }
    };
    match prediction_events_log.load_logs_from_file() {
        Err(e) => println!(
            "Could load data from JSON: {:?}, perhaps the file is empty?",
            e
        ),
        Ok(_) => (),
    };

    prediction_events_log.data.push(PredictionEvent::new());
    prediction_events_log
        .commit_logs_to_file()
        .expect("Could not write logs");

    // server::new(|| {
    //     App::new()
    //         // enable logger
    //         .middleware(middleware::Logger::default())
    //         .resource("/", |r| r.method(http::Method::POST).f(index))
    // }).bind("127.0.0.1:8080")
    // .unwrap()
    // .shutdown_timeout(1)
    // .start();

    // println!("Started http server: 127.0.0.1:8080");
    // let _ = sys.run();
}

type Id = Uuid;

#[derive(Serialize, Deserialize)]
enum Prediction {
    Affirmative,
    Negative,
}

impl Debug for Prediction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n",
            match self {
                Prediction::Affirmative => "Affirmative",
                Prediction::Negative => "Negative",
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PredictionEvent {
    id: Id,
    for_event: Id,
    from_user: Id,
    prediction: Prediction,
    timestamp: DateTime<Utc>,
}

impl PredictionEvent {
    fn new() -> Self {
        Self::default()
    }
}

impl Default for PredictionEvent {
    fn default() -> Self {
        PredictionEvent {
            id: Uuid::new_v4(),
            for_event: Uuid::new_v4(),
            from_user: Uuid::new_v4(),
            prediction: Prediction::Affirmative,
            timestamp: Utc::now(),
        }
    }
}

mod lfm {

    extern crate csv;
    extern crate serde;
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use std::fmt::Debug;
    use std::fs::OpenOptions;
    use std::io::Write;

    pub struct LogFileManager<T> {
        file_path: String,
        file_handle: std::fs::File,
        pub data: Vec<T>,
    }

    impl<T> LogFileManager<T>
    where
        T: Debug + DeserializeOwned + Serialize,
    {
        pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
            let file_handle = Self::get_file_handle(file_path)?;

            Ok(LogFileManager {
                file_path: file_path.to_string(),
                file_handle,
                data: Vec::new(),
            })
        }

        pub fn commit_logs_to_file(&mut self) -> Result<(), ()> {
            let mut writer = csv::Writer::from_writer(&self.file_handle);

            for log in self.data.iter() {
                if let Err(error) = writer.serialize(log) {
                    println!("{}", error);
                };
            }

            writer.flush();
            Ok(())
        }

        pub fn load_logs_from_file(&mut self) -> Result<(), serde_json::Error> {
            let mut logs = csv::Reader::from_reader(&self.file_handle);
            for log_result in logs.deserialize() {
                match log_result {
                    Ok(log) => self.data.push(log),
                    Err(e) => println!("{:?}", e),
                };
            }
            Ok(())
        }

        fn get_file_handle(file_path: &str) -> Result<std::fs::File, std::io::Error> {
            OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(file_path)
        }
    }
}
