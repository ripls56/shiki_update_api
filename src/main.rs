use std::fs;
use actix_web::{get, App, HttpServer, HttpResponse, HttpRequest};
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use colored::Colorize;


#[get("/update")]
async fn send_apk(req: HttpRequest) -> HttpResponse {
    return match find_first_file_with_extension("", "apk") {
        Ok(value) => {
            let file = actix_files::NamedFile::open_async(value).await.unwrap();
            file.into_response(&req)
        }
        Err(error) => HttpResponse::NotFound()
            .status(StatusCode::NOT_FOUND)
            .body(error)
    };
}

fn find_first_file_with_extension(folder_path: &str, target_extension: &str) -> Result<String, String> {
    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(target_extension) {
                        return Ok(file_name.to_string());
                    }
                }
            }
        }
    }
    Err(String::from("File doesn't exist"))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("{}", "Starting HTTP server!".green());
    HttpServer::new(|| {
        App::new()
            .service(send_apk)
            .wrap(Logger::new(("\nStatus Code %s ".green().to_string()
                + "IP: %a".cyan().to_string().as_str()
                + " %T".magenta().to_string().as_str()
                + "\nUser-Agent: %{User-Agent}i".bright_yellow().to_string().as_str()).as_str()))
    })
        .bind(("0.0.0.0", 8080))
        .unwrap()
        .run()
        .await
}