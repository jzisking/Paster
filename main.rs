use rocket::*;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::response::Redirect;
use rocket::tokio::fs::File;
use rocket::tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("index.html").await.ok()
}

/*
#[get("/publish?<id_title>&<id_content>")]
async fn publish(id_title: String, id_content: String) -> String {
    let uuid = Uuid::new_v4().to_string();
    if let Ok(mut file) = File::create(format!("entries/{}", uuid)).await {
        file.write(format!("Title: {}, Content: {}", id_title, id_content).as_bytes()).await;
        file.flush().await;
        uuid
    } else {
        "Failed to create paste".to_owned()
    }
}
 */

#[get("/publish?<id_title>&<id_content>")]
async fn publish(id_title: String, id_content: String) -> Redirect {
    let uuid = Uuid::new_v4().to_string();
    if let Ok(mut file) = File::create(format!("entries/{}", uuid)).await {
        file.write(format!("Title: {}, Content: {}", id_title, id_content).as_bytes()).await;
        file.flush().await;
        Redirect::to(format!("paste/{}", uuid))
    } else {
        Redirect::to("/")
    }
}

#[get("/paste/<id>")]
async fn paste(id: String) -> String {
    if let Ok(mut file) = File::open(format!("entries/{id}")).await {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).await;
        buffer
    } else {
        format!("Failed to open paste {id}, unknon id")
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, publish, paste])
}