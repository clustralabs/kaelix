use std::collections::HashMap;

use bollard::Docker;
use bollard::errors::Error;
use bollard::plugin::{ContainerCreateBody, HostConfig};
use bollard::query_parameters::{CreateContainerOptionsBuilder, CreateImageOptions};
use futures_util::StreamExt;

/// pull an image from the registry, streaming progress to the logs
// TODO: wire this up to a route handler — kept as a helper for now
#[allow(dead_code)]
pub async fn pull_image(docker: &Docker, image: &str, tag: &str) -> Result<(), Error> {
    let options = CreateImageOptions {
        from_image: Some(image.to_string()),
        tag: Some(tag.to_string()),
        ..Default::default()
    };

    tracing::info!(image, tag, "pulling image");

    let mut stream = docker.create_image(Some(options), None, None);

    // consume the stream until the download finishes
    while let Some(result) = stream.next().await {
        let output = result?;
        // the daemon reports pull failures mid-stream as errorDetail
        if let Some(detail) = output.error_detail {
            return Err(Error::DockerStreamError {
                error: detail
                    .message
                    .unwrap_or_else(|| "unknown pull error".into()),
            });
        }
        if let Some(status) = output.status {
            tracing::debug!(status, "pull progress");
        }
    }

    tracing::info!(image, tag, "pull complete");
    Ok(())
}

//Create a container
#[allow(dead_code)]
pub async fn create_container(
    docker: &Docker,
    image: &str,
    name: &str,
    domain: &str,
    port: &str,
) -> Result<(), Error> {
    let options = CreateContainerOptionsBuilder::default().name(name).build();
    let mut labels = HashMap::new();
    labels.insert("traefik.enable".to_string(), "true".to_string());
    labels.insert(
        format!("traefik.http.routers.{name}.rule"),
        format!("Host(`{domain}`)"),
    );
    labels.insert(
        format!("traefik.http.routers.{name}.entrypoints"),
        "web".to_string(),
    );
    labels.insert(
        format!("traefik.http.services.{name}.loadbalancer.server.port"),
        port.to_string(),
    );
    // kaelix-net is hardcoded
    let config = ContainerCreateBody {
        image: Some(image.to_string()),
        labels: Some(labels),
        host_config: Some(HostConfig {
            network_mode: Some("kaelix-net".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };
    tracing::info!(container = name, image, domain, "creating container");
    docker.create_container(Some(options), config).await?;
    tracing::info!(container = name, image, domain, "created container");
    Ok(())
}

#[allow(dead_code)]
pub async fn start_container(docker: &Docker, name: &str) -> Result<(), Error> {
    docker.start_container(name, None).await?;
    Ok(())
}
