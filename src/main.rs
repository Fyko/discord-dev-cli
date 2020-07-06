use reqwest::{Error};
pub mod structs;
pub mod helpers;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = std::env::var("TOKEN").unwrap();
    let me = helpers::me(token.clone()).await.unwrap();

    println!("{:#?}\n\n", me);

    let apps = helpers::applications(token.clone()).await.unwrap();
    println!("{:#?}\n\n", apps);

    let teams = helpers::teams(token.clone()).await.unwrap();
    println!("{:#?}\n\n", teams);

    Ok(())
}
