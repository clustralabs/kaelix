use bollard::Docker;
use bollard::errors::Error;
use bollard::query_parameters::CreateImageOptions;
use futures_util::StreamExt;

/// pull an image from the registry, streaming progress to the logs
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
                error: detail.message.unwrap_or_else(|| "unknown pull error".into()),
            });
        }
        if let Some(status) = output.status {
            tracing::debug!(status, "pull progress");
        }
    }

    tracing::info!(image, tag, "pull complete");
    Ok(())
}
