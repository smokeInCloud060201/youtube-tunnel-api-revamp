use base64::Engine;
use base64::engine::general_purpose;
use futures::future;
use futures::future::join_all;
use reqwest::{Client, Url};
use shared::model::video_search::{QueryParams, SearchVideoResponseResult, Thumbnail, Thumbnails, VideoSearchResult};

#[derive(Clone)]
pub struct VideoService {
    client: Client,
    api_key: String,
    url: String,
}

impl VideoService {
    pub fn new() -> Self {
        let api_key = std::env::var("YOUTUBE_API_KEY")
            .expect("YOUTUBE_API_KEY must be set in .env");
        let youtube_host = std::env::var("YOUTUBE_HOST")
            .expect("YOUTUBE_HOST must be set in .env");

        Self {
            client: Client::new(),
            api_key,
            url: youtube_host,
        }
    }

    async fn get_best_thumbnail(&self, thumbs: &Thumbnails) -> Result<Option<String>> {
        async fn fetch_and_encode(client: &Client, url: &str) -> Result<String> {
            let bytes = client.get(url).send().await?.bytes().await?;
            Ok(general_purpose::STANDARD.encode(&bytes))
        }

        let thumbs_default = if let Some(thumbs) = &thumbs.default_thumbnail {
            Some(fetch_and_encode(&self.client, &thumbs.url).await?)
        } else {
            None
        };

        let thumbs_medium = if let Some(thumbs) = &thumbs.medium {
            Some(fetch_and_encode(&self.client, &thumbs.url).await?)
        } else {
            None
        };

        let thumbs_high = if let Some(thumbs) = &thumbs.high {
            Some(fetch_and_encode(&self.client, &thumbs.url).await?)
        } else {
            None
        };


        let (default, medium, high) = future::join3(
            async {
                if let Some(f) = thumbs_default { f.await.ok() }
                else {
                    None
                }
            },
            async {
                if let Some(f) = thumbs_medium { f.await.ok() }
                else {
                    None
                }
            },
            async {
                if let Some(f) = thumbs_high { f.await.ok() }
                else {
                    None
                }
            }
        ).await;

        Ok(default.or(medium).or(high))
    }

    pub async fn search(&self, query: QueryParams) -> anyhow::Result<Vec<VideoSearchResult>> {

        let url = format!("{}/youtube/v3/search?part={}&q={}&type={}&maxResults={}&key={}", self.url, query.part, query.q, query.resource_type, query.resource_type, self.api_key);

        let result = self.client.get(&url).send().await?.text().await?;
        let search_result: SearchVideoResponseResult = serde_json::from_str(&result)?;

        let futures = search_result.items.into_iter().map(|item| {
            let client = self.client.clone();
            async move {
                let thumbs = Thumbnails {
                    default_thumbnail: Some(Thumbnail { url: "https://i.ytimg.com/vi/abc123/default.jpg".into(), width: None, height: None }),
                    medium: None,
                    high: Some(Thumbnail { url: "https://i.ytimg.com/vi/abc123/hqdefault.jpg".into(), width: None, height: None }),
                };

                let best_thumb = self.get_best_thumbnail(&thumbs).await.ok().flatten();

                if let Some(ref best) = best_thumb {
                    println!("Best thumbnail (base64): {}", &best[..50]);
                }

                VideoSearchResult {
                    id: item.id.video_id,
                    title: item.snippet.title,
                    description: item.snippet.description,
                    thumbnails: Vec::from(best_thumb),
                    channel_title: item.snippet.channel_title,
                    publish_time: item.snippet.publish_time,
                    published_at: item.snippet.published_at,
                }
            }
        });

        let response: Vec<VideoSearchResult> = join_all(futures).await;

        Ok(response)
    }
}