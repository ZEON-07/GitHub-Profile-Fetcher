use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <github_repo_url>", args[0]);
        std::process::exit(1);
    }
    
    let user_name = &args[1];
    let url = format!("https://api.github.com/users/{}", user_name);

    let client = reqwest::blocking::Client::new();
    let response = client.get(&url)
        .header("User-Agent", "github-fetcher")
        .send()
        .expect("Failed to send request");
    let json: serde_json::Value = response.json().expect("Failed to parse JSON");

    println!("GitHub User Info:");
    println!("Name: {}", json["name"].as_str().unwrap_or("N/A"));
    println!("Bio: {}", json["bio"].as_str().unwrap_or("N/A"));
    println!("Public Repos: {}", json["public_repos"].as_i64().unwrap_or(0));
    


}
