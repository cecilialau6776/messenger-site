use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use dotenvy::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

lazy_static! {
  static ref EXEC_PATH: String = std::env::var("EXEC_PATH").unwrap();
  static ref KEY_DIR: String = std::env::var("KEY_DIR").unwrap();
  static ref INDEX_PATH: PathBuf =
    PathBuf::from_str(&std::env::var("INDEX_PATH").unwrap()).unwrap();
}

#[derive(Deserialize)]
struct Message {
  email: String,
  content: String,
}

#[get("/ping")]
async fn ping() -> impl Responder {
  HttpResponse::Ok().body("pong!")
}

#[get("/")]
async fn index() -> Result<NamedFile> {
  Ok(NamedFile::open(&*INDEX_PATH)?)
}

#[post("/sendMessage")]
async fn send_message(info: web::Json<Message>) -> impl Responder {
  let email = &info.email;

  match Command::new(&*EXEC_PATH)
    .current_dir(&*KEY_DIR)
    .arg("getKey")
    .arg(email)
    .output()
  {
    Ok(output) => {
      // println!("stdout: {}", std::str::from_utf8(&output.stdout).unwrap());
      // println!("stderr: {}", std::str::from_utf8(&output.stderr).unwrap());
      // println!("exit code: {}", output.status.code().unwrap());
      if let Some(code) = output.status.code() {
        if code == 1 {
          return HttpResponse::NotFound().body(format!("No key found for {}", email));
        }
      }
    }
    Err(e) => return HttpResponse::InternalServerError().body(format!("Other Error: {}", e)),
  };
  println!("key aquired for {}", email);
  // let paths = std::fs::read_dir(&*KEY_DIR).unwrap();
  // for path in paths {
  //   println!("{}", path.unwrap().path().display());
  // }
  match Command::new(&*EXEC_PATH)
    .current_dir(&*KEY_DIR)
    // Command::new("/run/linux-x64/publish/copads")
    .arg("sendMsg")
    .arg(email)
    .arg(&info.content)
    .output()
  {
    Ok(_output) => HttpResponse::Ok().body(format!("Message successfully sent to {}", email)),
    Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
  }
}

#[get("/getMessage")]
async fn get_message() -> std::io::Result<String> {
  let email = "cecilialau6776@gmail.com";
  let output = Command::new(&*EXEC_PATH)
    .current_dir(&*KEY_DIR)
    .arg("getMsg")
    .arg(email)
    .output()?;
  Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // let chom: PathBuf = PathBuf::from_str("INDEX_PATH");
  dotenv().ok();
  HttpServer::new(|| {
    App::new()
      // .service(getKey)
      .service(send_message)
      .service(get_message)
      .service(index)
  })
  .workers(4)
  .bind(("0.0.0.0", 3000))?
  .run()
  .await
}
