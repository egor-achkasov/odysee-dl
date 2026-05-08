use crate::error::Error;

pub struct Post {
    pub name: String,
    pub filename: String,
    pub streaming_url: String,
}

impl Post {
    pub fn from_json(json: &serde_json::Value) -> Result<Self, Error> {
        let name = json["name"]
            .as_str()
            .ok_or_else(|| Error::Http("name not found".to_string()))?
            .to_string();
        let claim_id = json["claim_id"]
            .as_str()
            .ok_or_else(|| Error::Http("claim_id not found".to_string()))?;
        let sd_hash = json["value"]["source"]["sd_hash"]
            .as_str()
            .ok_or_else(|| Error::Http("sd_hash not found".to_string()))?;
        // URL format: confirmed by inspecting the LBRY `get` API response.
        // CDN only checks Referer: https://odysee.com/ — no auth token required.
        let streaming_url = format!(
            "https://player.odycdn.com/v6/streams/{}/{}.mp4",
            claim_id, &sd_hash[..6]
        );
        let filename = json["value"]["source"]["name"]
            .as_str()
            .unwrap_or(&name)
            .to_string();
        Ok(Post { name, filename, streaming_url })
    }

    pub fn content_length(&self) -> Option<u64> {
        ureq::head(&self.streaming_url)
            .header("Referer", "https://odysee.com/")
            .header("Origin", "https://odysee.com")
            .call()
            .ok()
            .and_then(|r| {
                r.headers()
                    .get("content-length")?
                    .to_str()
                    .ok()?
                    .parse()
                    .ok()
            })
    }

    pub fn download(&self, dir: &std::path::Path) -> Result<(), Error> {
        let path = dir.join(&self.filename);
        let mut response = ureq::get(&self.streaming_url)
            .header("Referer", "https://odysee.com/")
            .header("Origin", "https://odysee.com")
            .call()?;
        let mut file = std::fs::File::create(&path)?;
        let mut reader = response.body_mut().as_reader();
        std::io::copy(&mut reader, &mut file)?;
        Ok(())
    }
}
