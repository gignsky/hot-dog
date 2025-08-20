use dioxus::prelude::*;

// The database is only available to server code
#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        // Open the database from the persisted "hotdog.db" file
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database");

        // Create the "dogs" table if it doesn't already exist
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );",
        ).unwrap();

        // Return the connection
        conn
    };
}

// Expose a `save_dog` endpoint on our server that takes an "image" parameter
#[server]
pub async fn save_dog(image: String) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("INSERT INTO dogs (url) VALUES (?1)", &[&image]))?;
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
