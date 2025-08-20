use dioxus::prelude::*;

// Expose a `save_dog` endpoint on our server that takes an "image" parameter
#[server]
pub async fn save_dog(image: String) -> Result<(), ServerFnError> {
    use std::io::Write;

    // Open the `dogs.txt` file in append-only mode, creating it if it doesn't exist;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .unwrap();

    // And then write a newline to it with the image url
    file.write_fmt(format_args!("{image}\n"));

    Ok(())
}

// Documentation:
// // on the Client:
// #[server]
// async fn save_dog(image: String) -> Result<(), ServerFnError> {
//     Ok(())
// }
//
// // expanded
// async fn save_dog(image: String) -> Result<(), ServerFnError> {
//     reqwest::Client::new()
//         .post("http://localhost:8080/api/save_dog")
//         .json(&image)
//         .send()
//         .await?;
//
//     Ok(())
// }
//
// // on the server:
// struct SaveDogArgs {
//     image: String,
// }
//
// async fn save_dog(Json(args): Json<SaveDogArgs>) -> Result<(), ServerFnError> {
//     Ok(())
// }
//
// // // ❌ this will leak your DB_PASSWORD to your client app!
// // static DB_PASSWORD: &str = "1234";
// //
// // #[server]
// // async fn DoThing() -> Result<(), ServerFnError> {
// //     connect_to_db(DB_PASSWORD).await
// //     // ...
// // }
//
// // ✅ code in this module can only be accessed on the server
// #[cfg(feature = "server")]
// mod server_utils {
//     pub static DB_PASSWORD: &str = "1234";
// }
