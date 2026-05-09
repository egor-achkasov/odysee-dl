pub mod config;
pub mod error;
pub mod post;
pub mod event;

use error::Error;
use post::Post;
use event::Event;

pub fn run(config: config::Config, tx: std::sync::mpsc::Sender<Event>) -> Result<(), Error> {
    let channel = url_to_channel(&config.url)?;

    tx.send(Event::GetPostsStarted(config.url.clone())).ok();
    let posts = match get_posts(&channel, &tx) {
        Ok(posts) => {
            tx.send(Event::GetPostsFinished(config.url.clone())).ok();
            posts
        }
        Err(e) => {
            tx.send(Event::GetPostsFailed(config.url.clone(), e.to_string())).ok();
            return Err(e);
        }
    };

    std::fs::create_dir_all(&config.output_dir)?;
    for post in posts {
        tx.send(Event::DownloadPostStarted(post.name.clone())).ok();
        if config.resume {
            let local_path = config.output_dir.join(&post.filename);
            if let Ok(meta) = std::fs::metadata(&local_path) {
                if post.content_length(&tx).map_or(false, |remote| meta.len() == remote) {
                    tx.send(Event::DownloadPostSkipped(post.name.clone())).ok();
                    continue;
                }
            }
        }
        match post.download(&config.output_dir, &tx) {
            Ok(()) => tx.send(Event::DownloadPostFinished(post.name.clone())).ok(),
            Err(e) => tx.send(Event::DownloadPostFailed(post.name.clone(), e.to_string())).ok(),
        };
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    tx.send(Event::Done).ok();
    Ok(())
}

/// Extract the channel URL part from an Odysee URL, e.g. "https://odysee.com/@channel:1" -> "@channel:1"
fn url_to_channel(url: &str) -> Result<String, Error> {
    let rest = url
        .split("odysee.com/")
        .nth(1)
        .ok_or(Error::InvalidUrl)?;
    let channel = rest.split('?').next().unwrap_or(rest);
    Ok(channel.to_string())
}

fn get_posts(channel: &str, tx: &std::sync::mpsc::Sender<Event>) -> Result<Vec<Post>, Error> {
    let mut page = 1usize;
    let mut posts = Vec::new();

    loop {
        let body = serde_json::json!({
            "method": "claim_search",
            "params": {
                "channel": channel,
                "page": page,
                "page_size": 50,
                "claim_type": ["stream"],
            }
        });

        let body_bytes = serde_json::to_vec(&body)?;
        let response: serde_json::Value = post::with_retry(
            || {
                ureq::post("https://api.na-backend.odysee.com/api/v1/proxy")
                    .header("Content-Type", "application/json")
                    .send(body_bytes.clone())
            },
            || { tx.send(Event::RateLimited("API".to_string())).ok(); },
        )?
        .body_mut()
        .read_json()?;

        let items = response["result"]["items"]
            .as_array()
            .ok_or_else(|| Error::Http("items not found".to_string()))?;

        if items.is_empty() {
            break;
        }

        let page_posts: Vec<Post> = items
            .iter()
            .map(Post::from_json)
            .collect::<Result<Vec<Post>, Error>>()?;

        let total_pages = response["result"]["total_pages"].as_u64().unwrap_or(1) as usize;
        posts.extend(page_posts);

        if page >= total_pages {
            break;
        }
        page += 1;
    }

    Ok(posts)
}
