use super::{errors::ApiError, github_api_result::ContainerImage};
use itertools::Itertools;

const ACCEPT_VALUE: &str = "application/vnd.github.v3+json";

pub struct GhcrClient {
    token: String,
    pub url: String,
}

impl GhcrClient {
    pub fn new(token: String, image_name: String) -> Self {
        let token = format!("token {}", token);
        let url = format!(
            "https://api.github.com/user/packages/container/{}/versions",
            image_name
        );

        Self { token, url }
    }

    pub fn delete_images(&self, image_id: &str) -> Result<ureq::Response, ApiError> {
        let url = format!("{}/{}", self.url, image_id);
        ureq::delete(&url)
            .set("Authorization", &self.token)
            .set("Accept", ACCEPT_VALUE)
            .call()
            .map_err(ApiError::from)
    }

    pub fn list_images(&self) -> Result<Vec<ContainerImage>, ApiError> {
        let result = ureq::get(&self.url)
            .set("Authorization", &self.token)
            .set("Accept", ACCEPT_VALUE)
            .call();
        result.map_err(ApiError::from).and_then(|res| {
            res.into_json::<Vec<ContainerImage>>()
                .map(|list| {
                    list.into_iter()
                        .filter(|image| image.metadata.container.tags.is_empty())
                        .collect_vec()
                })
                .map_err(ApiError::from)
        })
    }
}

impl Default for GhcrClient {
    fn default() -> Self {
        Self::new("default_token".into(), "default_image_name".into())
    }
}
