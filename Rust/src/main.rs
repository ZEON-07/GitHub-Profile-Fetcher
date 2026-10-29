use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <github_repo_url> [--gitlab]", args[0]);
        std::process::exit(1);
    }

    let gitlab = args.len() > 2 && args[2] == "--gitlab";
    let user_name = &args[1];

    if gitlab {
        let json = fetch_gitlab_user_info(user_name).expect("Failed to fetch GitLab user info");
        println!("GitLab User Info:");
        println!("Name: {}", json["name"].as_str().unwrap_or("N/A"));
        println!("Bio: {}", json["bio"].as_str().unwrap_or("N/A"));
        println!("Public Repos: {}", json["public_repos"].as_i64().unwrap_or(0));
    } else {
        let json = fetch_github_user_info(user_name).expect("Failed to fetch GitHub user info");
        println!("GitHub User Info:");
        println!("Name: {}", json["name"].as_str().unwrap_or("N/A"));
        println!("Bio: {}", json["bio"].as_str().unwrap_or("N/A"));
        println!("Public Repos: {}", json["public_repos"].as_i64().unwrap_or(0));
    }
}



fn fetch_github_user_info(user_name: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/users/{}", user_name);
    let client = reqwest::blocking::Client::new();
    let response = client.get(&url)
        .header("User-Agent", "github-fetcher")
        .send()?;
    let json: serde_json::Value = response.json()?;
    Ok(json)
}

fn fetch_gitlab_user_info(user_name: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("https://gitlab.com/api/v4/users?username={}", user_name);
    let client = reqwest::blocking::Client::new();
    
    let response = client.get(&url)
        .header("User-Agent", "gitlab-fetcher")
        .send()?;
    let json: serde_json::Value = response.json()?;

    Ok(json)
}

