use crate::model;

pub async fn fetch_user_events(
    username: &str,
) -> Result<Vec<model::Event>, Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/users/{}/events", username);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "rust-app")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err("Failed to fetch data".into());
    }

    let events = serde_json::from_str::<Vec<model::Event>>(&response.text().await?)?;
    Ok(events)
}

pub fn display_user_events(events: Vec<model::Event>) {
    events.iter().for_each(|event| {
        println!("{}", normalize_event((*event).clone()));
    });
}

fn normalize_event(event: model::Event) -> String {
    match event.event_type.as_str() {
        "PushEvent" => {
            let commit_length = event.payload["size"].as_u64().unwrap();
            format!(
                "{} pushed {} commits to {} at {}",
                event.actor.display_login, commit_length, event.repo.name, event.created_at
            )
        }
        "WatchEvent" => {
            format!(
                "{} starred {} at {}",
                event.actor.display_login, event.repo.name, event.created_at
            )
        }
        "ForkEvent" => {
            format!(
                "{} forked {} at {}",
                event.actor.display_login, event.repo.name, event.created_at
            )
        }
        "CreateEvent" => {
            format!(
                "{} created a repository {} at {}",
                event.actor.display_login, event.repo.name, event.created_at
            )
        }
        "PullRequestEvent" => {
            format!(
                "{} opened a pull request at {}",
                event.actor.display_login, event.created_at
            )
        }
        "PublicEvent" => {
            format!(
                "{} open sourced a repository at {}",
                event.actor.display_login, event.created_at
            )
        }
        _ => format!(
            "{} did {} something at {}",
            event.actor.display_login, event.event_type, event.created_at
        ),
    }
}
