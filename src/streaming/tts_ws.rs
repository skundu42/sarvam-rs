use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::config::SarvamConfig;
use crate::error::{Result, SarvamError};
use crate::types::common::*;

#[derive(Debug, Clone)]
pub struct TtsStreamBuilder {
    config: SarvamConfig,
    model: Option<TextToSpeechModel>,
    target_language_code: Option<TextToSpeechLanguage>,
    speaker: Option<TextToSpeechSpeaker>,
    pitch: Option<f64>,
    pace: Option<f64>,
    loudness: Option<f64>,
    temperature: Option<f64>,
    speech_sample_rate: Option<SpeechSampleRate>,
    enable_preprocessing: Option<bool>,
    output_audio_codec: Option<TextToSpeechOutputAudioCodec>,
    dict_id: Option<String>,
}

impl TtsStreamBuilder {
    pub(crate) fn new(config: SarvamConfig) -> Self {
        Self {
            config,
            model: None,
            target_language_code: None,
            speaker: None,
            pitch: None,
            pace: None,
            loudness: None,
            temperature: None,
            speech_sample_rate: None,
            enable_preprocessing: None,
            output_audio_codec: None,
            dict_id: None,
        }
    }

    pub fn model(mut self, model: TextToSpeechModel) -> Self {
        self.model = Some(model);
        self
    }

    pub fn target_language_code(mut self, lang: TextToSpeechLanguage) -> Self {
        self.target_language_code = Some(lang);
        self
    }

    pub fn speaker(mut self, speaker: TextToSpeechSpeaker) -> Self {
        self.speaker = Some(speaker);
        self
    }

    pub fn pitch(mut self, pitch: f64) -> Self {
        self.pitch = Some(pitch);
        self
    }

    pub fn pace(mut self, pace: f64) -> Self {
        self.pace = Some(pace);
        self
    }

    pub fn loudness(mut self, loudness: f64) -> Self {
        self.loudness = Some(loudness);
        self
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn speech_sample_rate(mut self, rate: SpeechSampleRate) -> Self {
        self.speech_sample_rate = Some(rate);
        self
    }

    pub fn enable_preprocessing(mut self, enable: bool) -> Self {
        self.enable_preprocessing = Some(enable);
        self
    }

    pub fn output_audio_codec(mut self, codec: TextToSpeechOutputAudioCodec) -> Self {
        self.output_audio_codec = Some(codec);
        self
    }

    pub fn dict_id(mut self, dict_id: impl Into<String>) -> Self {
        self.dict_id = Some(dict_id.into());
        self
    }

    pub async fn connect(self) -> Result<TtsStream> {
        let ws_base = self
            .config
            .base_url
            .replace("https://", "wss://")
            .replace("http://", "ws://");

        let mut url = format!("{}/text-to-speech/ws", ws_base);

        let model_str = match &self.model {
            Some(TextToSpeechModel::BulbulV3) => "bulbul:v3",
            _ => "bulbul:v2",
        };
        url = format!("{}?model={}&send_completion_event=true", url, model_str);

        let request = tokio_tungstenite::tungstenite::http::Request::builder()
            .uri(&url)
            .header("api-subscription-key", &self.config.api_subscription_key)
            .body(())
            .map_err(|e| SarvamError::Custom(format!("WebSocket request error: {}", e)))?;

        let (ws_stream, _response) = connect_async(request)
            .await
            .map_err(|e| SarvamError::Custom(format!("WebSocket connection error: {}", e)))?;

        let (mut write, read) = ws_stream.split();

        let config_msg = WsMessage {
            msg_type: "config".to_string(),
            data: WsConfigData {
                model: self.model.clone(),
                target_language_code: self.target_language_code.clone(),
                speaker: self.speaker.clone(),
                pitch: self.pitch,
                pace: self.pace,
                loudness: self.loudness,
                temperature: self.temperature,
                speech_sample_rate: self.speech_sample_rate.clone(),
                enable_preprocessing: self.enable_preprocessing,
                output_audio_codec: self.output_audio_codec.clone(),
                dict_id: self.dict_id.clone(),
                min_buffer_size: None,
                max_chunk_length: None,
                output_audio_bitrate: None,
            },
        };

        let json = serde_json::to_string(&config_msg)
            .map_err(|e| SarvamError::Custom(format!("Config serialization error: {}", e)))?;
        write
            .send(Message::Text(json.into()))
            .await
            .map_err(|e| SarvamError::Custom(format!("WebSocket send error: {}", e)))?;

        Ok(TtsStream {
            write,
            read,
        })
    }
}

#[derive(Debug, Serialize)]
struct WsMessage<T: Serialize> {
    #[serde(rename = "type")]
    msg_type: String,
    data: T,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
struct WsConfigData {
    model: Option<TextToSpeechModel>,
    target_language_code: Option<TextToSpeechLanguage>,
    speaker: Option<TextToSpeechSpeaker>,
    pitch: Option<f64>,
    pace: Option<f64>,
    loudness: Option<f64>,
    temperature: Option<f64>,
    speech_sample_rate: Option<SpeechSampleRate>,
    enable_preprocessing: Option<bool>,
    output_audio_codec: Option<TextToSpeechOutputAudioCodec>,
    dict_id: Option<String>,
    min_buffer_size: Option<u32>,
    max_chunk_length: Option<u32>,
    output_audio_bitrate: Option<String>,
}

#[derive(Debug, Serialize)]
struct WsTextData {
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct TtsAudioOutput {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub data: TtsAudioData,
}

#[derive(Debug, Deserialize)]
pub struct TtsAudioData {
    pub content_type: String,
    pub audio: String,
    #[serde(default)]
    pub request_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TtsEventResponse {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub data: TtsEventData,
}

#[derive(Debug, Deserialize)]
pub struct TtsEventData {
    pub event_type: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub timestamp: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TtsErrorResponse {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub data: TtsErrorData,
}

#[derive(Debug, Deserialize)]
pub struct TtsErrorData {
    pub message: String,
    #[serde(default)]
    pub code: Option<i32>,
    #[serde(default)]
    pub request_id: Option<String>,
}

#[derive(Debug)]
pub enum TtsMessage {
    Audio(TtsAudioOutput),
    Event(TtsEventResponse),
    Error(TtsErrorResponse),
}

pub struct TtsStream {
    write: futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        Message,
    >,
    read: futures_util::stream::SplitStream<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    >,
}

impl TtsStream {
    pub async fn send_text(&mut self, text: impl Into<String>) -> Result<()> {
        let msg = WsMessage {
            msg_type: "text".to_string(),
            data: WsTextData { text: text.into() },
        };
        let json = serde_json::to_string(&msg)
            .map_err(|e| SarvamError::Custom(format!("Serialization error: {}", e)))?;
        self.write
            .send(Message::Text(json.into()))
            .await
            .map_err(|e| SarvamError::Custom(format!("WebSocket send error: {}", e)))
    }

    pub async fn flush(&mut self) -> Result<()> {
        let msg = serde_json::json!({"type": "flush"});
        let json = serde_json::to_string(&msg)
            .map_err(|e| SarvamError::Custom(format!("Serialization error: {}", e)))?;
        self.write
            .send(Message::Text(json.into()))
            .await
            .map_err(|e| SarvamError::Custom(format!("WebSocket send error: {}", e)))
    }

    pub async fn ping(&mut self) -> Result<()> {
        let msg = serde_json::json!({"type": "ping"});
        let json = serde_json::to_string(&msg)
            .map_err(|e| SarvamError::Custom(format!("Serialization error: {}", e)))?;
        self.write
            .send(Message::Text(json.into()))
            .await
            .map_err(|e| SarvamError::Custom(format!("WebSocket send error: {}", e)))
    }

    pub async fn next(&mut self) -> Option<Result<TtsMessage>> {
        loop {
            let msg = self.read.next().await?;
            match msg {
                Ok(Message::Text(text)) => {
                    let parsed: serde_json::Value = match serde_json::from_str(&text) {
                        Ok(v) => v,
                        Err(e) => return Some(Err(SarvamError::Custom(format!("JSON parse error: {}", e)))),
                    };
                    let msg_type = parsed.get("type")?.as_str()?.to_string();
                    match msg_type.as_str() {
                        "audio" => {
                            match serde_json::from_value::<TtsAudioOutput>(parsed) {
                                Ok(audio) => return Some(Ok(TtsMessage::Audio(audio))),
                                Err(e) => return Some(Err(SarvamError::Custom(format!("Audio parse error: {}", e)))),
                            }
                        }
                        "event" => {
                            match serde_json::from_value::<TtsEventResponse>(parsed) {
                                Ok(event) => return Some(Ok(TtsMessage::Event(event))),
                                Err(e) => return Some(Err(SarvamError::Custom(format!("Event parse error: {}", e)))),
                            }
                        }
                        "error" => {
                            match serde_json::from_value::<TtsErrorResponse>(parsed) {
                                Ok(error) => return Some(Ok(TtsMessage::Error(error))),
                                Err(e) => return Some(Err(SarvamError::Custom(format!("Error parse error: {}", e)))),
                            }
                        }
                        other => {
                            return Some(Err(SarvamError::Custom(format!(
                                "Unknown message type: {}",
                                other
                            ))));
                        }
                    }
                }
                Ok(Message::Close(_)) => return None,
                Ok(_) => continue,
                Err(e) => {
                    return Some(Err(SarvamError::Custom(format!(
                        "WebSocket error: {}",
                        e
                    ))));
                }
            }
        }
    }
}

impl std::fmt::Debug for TtsStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TtsStream").finish()
    }
}
