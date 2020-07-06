use reqwest::{Client, Error};
use crate::structs;

const ENDPOINT: &'static str = "https://discord.com/api/v6";

fn form_url(path: &str) -> String {
    return format!("{}{}", ENDPOINT, &path);
}

pub async fn me(token: String) -> Result<structs::Me, Error> {
    let url = form_url("/users/@me");

    let res = Client::new()
        .get(&url)
        .header("Authorization", token)
        .send()
        .await
        .unwrap();

    if res.status() != 200 {
        panic!("Request failed with status code: {}", res.status());
    }

    let user: structs::Me = res.json().await.unwrap();
    return Ok(user);
}

pub async fn applications(token: String) -> Result<Vec<structs::Application>, Error> {
    let url = form_url("/applications?with_team_applications=true");

    let res = Client::new()
        .get(&url)
        .header("Authorization", token)
        .send()
        .await
        .unwrap();

    if res.status() != 200 {
        panic!("Request failed with status code: {}", res.status());
    }

    let applications: Vec<structs::Application> = res.json().await.unwrap();
    return Ok(applications);
}

pub async fn teams(token: String) -> Result<Vec<structs::Team>, Error> {
    let url = form_url("/teams");

    let res = Client::new()
        .get(&url)
        .header("Authorization", token)
        .send()
        .await
        .unwrap();

    if res.status() != 200 {
        panic!("Request failed with status code: {}", res.status());
    }

    let teams: Vec<structs::Team> = res.json().await.unwrap();
    return Ok(teams);
}