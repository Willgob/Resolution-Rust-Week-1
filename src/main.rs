use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    name: String,
    login: String,
    bio: String,
    html_url: String,
}

fn main() {

    let client = reqwest::blocking::Client::new();

    let url = format!("https://api.github.com/users/{}", "willgob");

    let user: User = client
        .get(&url)
        .header("User-Agent", "Rust-Reqwest")
        .send()
        .expect("Failed to fetch userdata")
        .json()
        .expect("Failed to parse userdata");

    println!("Github User Data for{}\n", user.name);
    println!("Name - {} \n Username - {} \n Bio - {} \n Github Url - {}" , user.name, user.login, user.bio, user.html_url);
}
