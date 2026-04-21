use std::time::Duration;

use sarvam::{
    types::{
        CreateDocumentIntelligenceJobRequest, DocumentIntelligenceJobParameters,
        DocumentIntelligenceLanguage, DocumentOutputFormat, DocumentUploadUrlsRequest,
    },
    SarvamClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("SARVAM_API_KEY").expect("SARVAM_API_KEY not set");
    let client = SarvamClient::new(api_key);

    let job = client
        .document_intelligence()
        .create_job(CreateDocumentIntelligenceJobRequest {
            job_parameters: Some(DocumentIntelligenceJobParameters {
                language: Some(DocumentIntelligenceLanguage::EnIn),
                output_format: Some(DocumentOutputFormat::Markdown),
                ..Default::default()
            }),
            callback: None,
        })
        .await?;

    println!("Created doc job: {}", job.job_id);

    let upload_urls = client
        .document_intelligence()
        .get_upload_urls(DocumentUploadUrlsRequest {
            job_id: job.job_id.clone(),
            files: vec!["document.pdf".to_string()],
        })
        .await?;

    println!("Upload URLs: {}", upload_urls.upload_urls.len());
    client
        .document_intelligence()
        .start_job(&job.job_id)
        .await?;

    let status = client
        .document_intelligence()
        .wait_until_terminal(&job.job_id, Duration::from_secs(2))
        .await?;
    println!("Final state: {:?}", status.job_state);

    if status.job_state.is_success_like() {
        let downloads = client
            .document_intelligence()
            .get_download_urls(&job.job_id)
            .await?;
        println!("Download URLs: {}", downloads.download_urls.len());
    }

    Ok(())
}
