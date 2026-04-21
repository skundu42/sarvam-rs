use eventsource_stream::Eventsource;
use futures_core::Stream;
use futures_util::StreamExt;

use crate::error::{Result, SarvamError};
use crate::types::chat::ChatCompletionChunk;

pub struct ChatStream {
    inner: std::pin::Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk>> + Send>>,
}

impl ChatStream {
    pub(crate) fn new(response: reqwest::Response) -> Self {
        let stream = response
            .bytes_stream()
            .eventsource()
            .filter_map(move |result| async move {
                match result {
                    Ok(event) => {
                        if event.data == "[DONE]" {
                            return None;
                        }
                        match serde_json::from_str::<ChatCompletionChunk>(&event.data) {
                            Ok(chunk) => Some(Ok(chunk)),
                            Err(e) => Some(Err(SarvamError::JsonError(e))),
                        }
                    }
                    Err(e) => Some(Err(SarvamError::Custom(format!("SSE error: {}", e)))),
                }
            })
            .boxed();

        Self { inner: stream }
    }

    pub async fn next(&mut self) -> Option<Result<ChatCompletionChunk>> {
        self.inner.next().await
    }
}

impl std::fmt::Debug for ChatStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChatStream").finish()
    }
}
