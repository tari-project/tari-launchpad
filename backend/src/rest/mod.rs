// Copyright 2021. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

pub mod quay_io;
pub mod docker_hub;
pub mod service_registry;

use bollard::{image::ListImagesOptions, models::ImageSummary};
use log::error;
use thiserror::Error;
use std::collections::HashMap;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

use crate::docker::{DOCKER_INSTANCE, ImageType, TariWorkspace};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct TagInfo {
    pub latest: bool,
    pub created_on: String,
    pub digest: String,
}

#[derive(Error, Debug)]
pub enum DockerImageError {
    #[error("The image {0} is not found")]
    ImageNotFound(String),
    #[error("Something went wrong with the Docker API")]
    DockerError(#[from] bollard::errors::Error),
    #[error("Could not create an identity file")]
    #[allow(dead_code)]
    InvalidImageType,
}

pub async fn list_image(fully_qualified_image_name: String) -> Result<Vec<ImageSummary>, DockerImageError> {
    let docker = DOCKER_INSTANCE.clone();
    let mut fillter: HashMap<String, Vec<String>> = HashMap::new();
    fillter.insert("reference".to_string(), vec![fully_qualified_image_name.clone()]);
    let result = docker
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            filters: fillter,
            ..Default::default()
        }))
        .await
        .map_err(|err| {
            error!("Error searching for{}. Err: {}", fully_qualified_image_name, err);
            DockerImageError::ImageNotFound(fully_qualified_image_name)
        })?;
    Ok(result)
}

pub async fn is_up_to_date(image: ImageType, tag: &TagInfo) -> Result<bool, DockerImageError> {
    let docker = DOCKER_INSTANCE.clone();
    let fully_qualified_image_name = TariWorkspace::fully_qualified_image(image, None);
    let image_ids: Vec<String> = list_image(fully_qualified_image_name.clone())
        .await?
        .iter()
        .map(|img| img.id.clone())
        .collect();

    // No local image found, tell them to fetch a new one
    if image_ids.is_empty() {
        return Ok(false)
    }

    for image_id in image_ids {
        let local_image = docker.inspect_image(&image_id).await?;

        println!("local date: {:?} - tag date: {:?}", local_image.created, tag.created_on);

        println!("{:?} Trying ", image.display_name().to_string());

        if let Ok(local_time) = DateTime::parse_from_rfc3339(&local_image.created) {
            println!("{:?} one", image.display_name().to_string());

            if let Ok(tag_time) = DateTime::parse_from_rfc2822(&tag.created_on) {
                println!("{:?} two", image.display_name().to_string());

                if local_time < tag_time {
                    return Ok(false);
                }
            }
        }
    }

    Ok(true)
}