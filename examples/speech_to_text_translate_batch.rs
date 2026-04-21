use std::time::Duration;

use sarvam::{
    types::{
        CreateSpeechBatchJobRequest, SpeechBatchDownloadUrlsRequest, SpeechBatchModel,
        SpeechBatchUploadUrlsRequest, SpeechToTextTranslateBatchJobParameters, SttMode,
    },
    SarvamClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("SARVAM_API_KEY").expect("SARVAM_API_KEY not set");
    let client = SarvamClient::new(api_key);

    let job = client
        .speech_to_text_translate_batch()
        .create_job(
            CreateSpeechBatchJobRequest {
                job_parameters: SpeechToTextTranslateBatchJobParameters {
                    model: Some(SpeechBatchModel::SaarasV3),
                    mode: Some(SttMode::Translate),
                    ..Default::default()
                },
                callback: None,
            },
            None,
        )
        .await?;

    println!("Created job: {}", job.job_id);

    let upload_urls = client
        .speech_to_text_translate_batch()
        .get_upload_urls(
            SpeechBatchUploadUrlsRequest {
                job_id: job.job_id.clone(),
                files: vec!["audio.wav".to_string()],
            },
            None,
        )
        .await?;

    println!("Upload URLs: {}", upload_urls.upload_urls.len());
    let started = client
        .speech_to_text_translate_batch()
        .start_job(&job.job_id, None)
        .await?;
    println!("Started: {:?}", started.job_state);

    let status = client
        .speech_to_text_translate_batch()
        .wait_until_terminal(&job.job_id, Duration::from_secs(2), None)
        .await?;
    println!("Final state: {:?}", status.job_state);

    if status.job_state.is_success_like() {
        let output_files = status.successful_output_files();
        let downloads = client
            .speech_to_text_translate_batch()
            .get_download_urls(
                SpeechBatchDownloadUrlsRequest {
                    job_id: job.job_id.clone(),
                    files: output_files,
                },
                None,
            )
            .await?;
        println!("Download URLs: {}", downloads.download_urls.len());
    }

    Ok(())
}
