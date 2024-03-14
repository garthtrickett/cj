use cornucopia::CodegenSettings;
use dotenv::dotenv;
use postgres::{Client, NoTls};

fn main() -> Result<(), postgres::Error> {
    dotenv().ok(); // This line loads the .env file

    let queries_path = "src/queries";
    let destination = "src/cornucopia.rs";
    let settings = CodegenSettings {
        is_async: true,
        derive_ser: false,
    };

    println!("cargo:rerun-if-changed=migrations");
    println!("cargo:rerun-if-changed={}", queries_path);

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("DATABASE_URL: {}", database_url);

    let mut client = Client::connect(&database_url, NoTls)?;
    let result = cornucopia::generate_live(&mut client, queries_path, Some(destination), settings);
    match result {
        Ok(ok) => println!("{:?}", ok),
        Err(e) => eprintln!("Error: {:?}", e),
    }

    Ok(())
}
