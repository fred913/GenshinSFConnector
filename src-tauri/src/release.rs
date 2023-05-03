#[derive(serde::Serialize, serde::Deserialize)]
pub struct Release {
  pub tag_name: String,
  pub link: String,
}

#[tauri::command]
pub async fn get_latest_release() -> Release {
  let url = "https://api.github.com/repos/Grasscutters/Cultivation/releases/latest";
  let client = reqwest::Client::new();
  let response = client
    .get(url)
    .header("User-Agent", "Cultivation")
    .send()
    .await
    .unwrap();
  let text = response.text().await.unwrap();

  // This includes ip when github rate limits you, so avoid it for now to avoid leaks through screenshots
  //println!("Response: {}", text);

  // Parse "tag_name" from JSON
  let json: serde_json::Value = serde_json::from_str(&text).unwrap();
  let tag_name = json["tag_name"].as_str().unwrap();

  // Parse "html_url"
  let link = json["html_url"].as_str().unwrap();

  Release {
    tag_name: tag_name.to_string(),
    link: link.to_string(),
  }
}
