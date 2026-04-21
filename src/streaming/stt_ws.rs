use base64::Engine;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::config::SarvamConfig;
use crate::error::{Result, SarvamError};
use crate::types::speech_to_text::{InputAudioCodec, SpeechToTextLanguage, SttMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SttEndpointKind {
    SpeechToText,
    SpeechToTextTranslate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebSocketSampleRate {
    #[serde(rename = "16000")]
    Hz16000,
    #[serde(rename = "8000")]
    Hz8000,
}

impl WebSocketSampleRate {
    fn as_str(self) -> &'static str {
        match self {
            WebSocketSampleRate::Hz16000 => "16000",
            WebSocketSampleRate::Hz8000 => "8000",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SttAudioEncoding {
    #[serde(rename = "audio/wav")]
    AudioWav,
    #[serde(rename = "audio/pcm_s16le")]
    AudioPcmS16le,
    #[serde(rename = "audio/pcm_l16")]
    AudioPcmL16,
    #[serde(rename = "audio/pcm_raw")]
    AudioPcmRaw,
}

impl SttAudioEncoding {
    fn as_str(self) -> &'static str {
        match self {
            SttAudioEncoding::AudioWav => "audio/wav",
            SttAudioEncoding::AudioPcmS16le => "audio/pcm_s16le",
            SttAudioEncoding::AudioPcmL16 => "audio/pcm_l16",
            SttAudioEncoding::AudioPcmRaw => "audio/pcm_raw",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SttStreamingModel {
    #[serde(rename = "saaras:v3")]
    SaarasV3,
    #[serde(rename = "saaras:v2.5")]
    SaarasV2_5,
    #[serde(rename = "saarika:v2.5")]
    SaarikaV2_5,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SttStreamingTranslateModel {
    #[serde(rename = "saaras:v3")]
    SaarasV3,
    #[serde(rename = "saaras:v2.5")]
    SaarasV2_5,
}

#[derive(Debug, Clone)]
pub struct SttStreamBuilder {
    config: SarvamConfig,
    model: Option<SttStreamingModel>,
    mode: Option<SttMode>,
    language_code: Option<SpeechToTextLanguage>,
    sample_rate: Option<WebSocketSampleRate>,
    high_vad_sensitivity: Option<bool>,
    positive_speech_threshold: Option<f32>,
    negative_speech_threshold: Option<f32>,
    min_speech_frames: Option<u32>,
    first_turn_min_speech_frames: Option<u32>,
    negative_frames_count: Option<u32>,
    negative_frames_window: Option<u32>,
    start_speech_volume_threshold: Option<f32>,
    interrupt_min_speech_frames: Option<u32>,
    pre_speech_pad_frames: Option<u32>,
    num_initial_ignored_frames: Option<u32>,
    vad_signals: Option<bool>,
    flush_signal: Option<bool>,
    input_audio_codec: Option<InputAudioCodec>,
}

#[derive(Debug, Clone)]
pub struct SttTranslateStreamBuilder {
    config: SarvamConfig,
    model: Option<SttStreamingTranslateModel>,
    mode: Option<SttMode>,
    sample_rate: Option<WebSocketSampleRate>,
    high_vad_sensitivity: Option<bool>,
    positive_speech_threshold: Option<f32>,
    negative_speech_threshold: Option<f32>,
    min_speech_frames: Option<u32>,
    first_turn_min_speech_frames: Option<u32>,
    negative_frames_count: Option<u32>,
    negative_frames_window: Option<u32>,
    start_speech_volume_threshold: Option<f32>,
    interrupt_min_speech_frames: Option<u32>,
    pre_speech_pad_frames: Option<u32>,
    num_initial_ignored_frames: Option<u32>,
    vad_signals: Option<bool>,
    flush_signal: Option<bool>,
    input_audio_codec: Option<InputAudioCodec>,
}

macro_rules! with_ws_params {
    ($ty:ident) => {
        impl $ty {
            pub fn mode(mut self, mode: SttMode) -> Self {
                self.mode = Some(mode);
                self
            }

            pub fn sample_rate(mut self, sample_rate: WebSocketSampleRate) -> Self {
                self.sample_rate = Some(sample_rate);
                self
            }

            pub fn high_vad_sensitivity(mut self, enabled: bool) -> Self {
                self.high_vad_sensitivity = Some(enabled);
                self
            }

            pub fn positive_speech_threshold(mut self, threshold: f32) -> Self {
                self.positive_speech_threshold = Some(threshold);
                self
            }

            pub fn negative_speech_threshold(mut self, threshold: f32) -> Self {
                self.negative_speech_threshold = Some(threshold);
                self
            }

            pub fn min_speech_frames(mut self, count: u32) -> Self {
                self.min_speech_frames = Some(count);
                self
            }

            pub fn first_turn_min_speech_frames(mut self, count: u32) -> Self {
                self.first_turn_min_speech_frames = Some(count);
                self
            }

            pub fn negative_frames_count(mut self, count: u32) -> Self {
                self.negative_frames_count = Some(count);
                self
            }

            pub fn negative_frames_window(mut self, count: u32) -> Self {
                self.negative_frames_window = Some(count);
                self
            }

            pub fn start_speech_volume_threshold(mut self, threshold: f32) -> Self {
                self.start_speech_volume_threshold = Some(threshold);
                self
            }

            pub fn interrupt_min_speech_frames(mut self, count: u32) -> Self {
                self.interrupt_min_speech_frames = Some(count);
                self
            }

            pub fn pre_speech_pad_frames(mut self, count: u32) -> Self {
                self.pre_speech_pad_frames = Some(count);
                self
            }

            pub fn num_initial_ignored_frames(mut self, count: u32) -> Self {
                self.num_initial_ignored_frames = Some(count);
                self
            }

            pub fn vad_signals(mut self, enabled: bool) -> Self {
                self.vad_signals = Some(enabled);
                self
            }

            pub fn flush_signal(mut self, enabled: bool) -> Self {
                self.flush_signal = Some(enabled);
                self
            }

            pub fn input_audio_codec(mut self, codec: InputAudioCodec) -> Self {
                self.input_audio_codec = Some(codec);
                self
            }
        }
    };
}

impl SttStreamBuilder {
    pub(crate) fn new(config: SarvamConfig) -> Self {
        Self {
            config,
            model: None,
            mode: None,
            language_code: None,
            sample_rate: None,
            high_vad_sensitivity: None,
            positive_speech_threshold: None,
            negative_speech_threshold: None,
            min_speech_frames: None,
            first_turn_min_speech_frames: None,
            negative_frames_count: None,
            negative_frames_window: None,
            start_speech_volume_threshold: None,
            interrupt_min_speech_frames: None,
            pre_speech_pad_frames: None,
            num_initial_ignored_frames: None,
            vad_signals: None,
            flush_signal: None,
            input_audio_codec: None,
        }
    }

    pub fn model(mut self, model: SttStreamingModel) -> Self {
        self.model = Some(model);
        self
    }

    pub fn language_code(mut self, language_code: SpeechToTextLanguage) -> Self {
        self.language_code = Some(language_code);
        self
    }

    pub async fn connect(self) -> Result<SttStream> {
        let mut query = vec![(
            "language-code".to_string(),
            serde_json::to_value(self.language_code.unwrap_or(SpeechToTextLanguage::Unknown))?
                .as_str()
                .unwrap_or("unknown")
                .to_string(),
        )];

        if let Some(model) = &self.model {
            query.push((
                "model".to_string(),
                serde_json::to_value(model)?
                    .as_str()
                    .unwrap_or("saaras:v3")
                    .to_string(),
            ));
        }

        connect_stt_ws(
            self.config,
            SttEndpointKind::SpeechToText,
            query,
            SttCommonQueryParams {
                mode: self.mode,
                sample_rate: self.sample_rate,
                high_vad_sensitivity: self.high_vad_sensitivity,
                positive_speech_threshold: self.positive_speech_threshold,
                negative_speech_threshold: self.negative_speech_threshold,
                min_speech_frames: self.min_speech_frames,
                first_turn_min_speech_frames: self.first_turn_min_speech_frames,
                negative_frames_count: self.negative_frames_count,
                negative_frames_window: self.negative_frames_window,
                start_speech_volume_threshold: self.start_speech_volume_threshold,
                interrupt_min_speech_frames: self.interrupt_min_speech_frames,
                pre_speech_pad_frames: self.pre_speech_pad_frames,
                num_initial_ignored_frames: self.num_initial_ignored_frames,
                vad_signals: self.vad_signals,
                flush_signal: self.flush_signal,
                input_audio_codec: self.input_audio_codec,
            },
        )
        .await
    }
}

with_ws_params!(SttStreamBuilder);

impl SttTranslateStreamBuilder {
    pub(crate) fn new(config: SarvamConfig) -> Self {
        Self {
            config,
            model: None,
            mode: None,
            sample_rate: None,
            high_vad_sensitivity: None,
            positive_speech_threshold: None,
            negative_speech_threshold: None,
            min_speech_frames: None,
            first_turn_min_speech_frames: None,
            negative_frames_count: None,
            negative_frames_window: None,
            start_speech_volume_threshold: None,
            interrupt_min_speech_frames: None,
            pre_speech_pad_frames: None,
            num_initial_ignored_frames: None,
            vad_signals: None,
            flush_signal: None,
            input_audio_codec: None,
        }
    }

    pub fn model(mut self, model: SttStreamingTranslateModel) -> Self {
        self.model = Some(model);
        self
    }

    pub async fn connect(self) -> Result<SttStream> {
        let mut query = Vec::new();
        if let Some(model) = &self.model {
            query.push((
                "model".to_string(),
                serde_json::to_value(model)?
                    .as_str()
                    .unwrap_or("saaras:v3")
                    .to_string(),
            ));
        }

        connect_stt_ws(
            self.config,
            SttEndpointKind::SpeechToTextTranslate,
            query,
            SttCommonQueryParams {
                mode: self.mode,
                sample_rate: self.sample_rate,
                high_vad_sensitivity: self.high_vad_sensitivity,
                positive_speech_threshold: self.positive_speech_threshold,
                negative_speech_threshold: self.negative_speech_threshold,
                min_speech_frames: self.min_speech_frames,
                first_turn_min_speech_frames: self.first_turn_min_speech_frames,
                negative_frames_count: self.negative_frames_count,
                negative_frames_window: self.negative_frames_window,
                start_speech_volume_threshold: self.start_speech_volume_threshold,
                interrupt_min_speech_frames: self.interrupt_min_speech_frames,
                pre_speech_pad_frames: self.pre_speech_pad_frames,
                num_initial_ignored_frames: self.num_initial_ignored_frames,
                vad_signals: self.vad_signals,
                flush_signal: self.flush_signal,
                input_audio_codec: self.input_audio_codec,
            },
        )
        .await
    }
}

with_ws_params!(SttTranslateStreamBuilder);

#[derive(Debug, Clone, Default)]
struct SttCommonQueryParams {
    mode: Option<SttMode>,
    sample_rate: Option<WebSocketSampleRate>,
    high_vad_sensitivity: Option<bool>,
    positive_speech_threshold: Option<f32>,
    negative_speech_threshold: Option<f32>,
    min_speech_frames: Option<u32>,
    first_turn_min_speech_frames: Option<u32>,
    negative_frames_count: Option<u32>,
    negative_frames_window: Option<u32>,
    start_speech_volume_threshold: Option<f32>,
    interrupt_min_speech_frames: Option<u32>,
    pre_speech_pad_frames: Option<u32>,
    num_initial_ignored_frames: Option<u32>,
    vad_signals: Option<bool>,
    flush_signal: Option<bool>,
    input_audio_codec: Option<InputAudioCodec>,
}

async fn connect_stt_ws(
    config: SarvamConfig,
    endpoint_kind: SttEndpointKind,
    mut query: Vec<(String, String)>,
    params: SttCommonQueryParams,
) -> Result<SttStream> {
    if let Some(mode) = &params.mode {
        query.push((
            "mode".to_string(),
            serde_json::to_value(mode)?
                .as_str()
                .unwrap_or("transcribe")
                .to_string(),
        ));
    }
    if let Some(sample_rate) = params.sample_rate {
        query.push(("sample_rate".to_string(), sample_rate.as_str().to_string()));
    }
    push_bool(
        &mut query,
        "high_vad_sensitivity",
        params.high_vad_sensitivity,
    );
    push_number(
        &mut query,
        "positive_speech_threshold",
        params.positive_speech_threshold,
    );
    push_number(
        &mut query,
        "negative_speech_threshold",
        params.negative_speech_threshold,
    );
    push_number(&mut query, "min_speech_frames", params.min_speech_frames);
    push_number(
        &mut query,
        "first_turn_min_speech_frames",
        params.first_turn_min_speech_frames,
    );
    push_number(
        &mut query,
        "negative_frames_count",
        params.negative_frames_count,
    );
    push_number(
        &mut query,
        "negative_frames_window",
        params.negative_frames_window,
    );
    push_number(
        &mut query,
        "start_speech_volume_threshold",
        params.start_speech_volume_threshold,
    );
    push_number(
        &mut query,
        "interrupt_min_speech_frames",
        params.interrupt_min_speech_frames,
    );
    push_number(
        &mut query,
        "pre_speech_pad_frames",
        params.pre_speech_pad_frames,
    );
    push_number(
        &mut query,
        "num_initial_ignored_frames",
        params.num_initial_ignored_frames,
    );
    push_bool(&mut query, "vad_signals", params.vad_signals);
    push_bool(&mut query, "flush_signal", params.flush_signal);

    if let Some(codec) = &params.input_audio_codec {
        query.push((
            "input_audio_codec".to_string(),
            serde_json::to_value(codec)?
                .as_str()
                .unwrap_or("wav")
                .to_string(),
        ));
    }

    let ws_base = config
        .base_url
        .replace("https://", "wss://")
        .replace("http://", "ws://");
    let path = match endpoint_kind {
        SttEndpointKind::SpeechToText => "/speech-to-text/ws",
        SttEndpointKind::SpeechToTextTranslate => "/speech-to-text-translate/ws",
    };
    let mut url = format!("{ws_base}{path}");
    if !query.is_empty() {
        let query_string = query
            .into_iter()
            .map(|(key, value)| format!("{key}={}", urlencoding::encode(&value)))
            .collect::<Vec<_>>()
            .join("&");
        url = format!("{url}?{query_string}");
    }

    let request = tokio_tungstenite::tungstenite::http::Request::builder()
        .uri(&url)
        .header("api-subscription-key", &config.api_subscription_key)
        .body(())
        .map_err(|e| SarvamError::Custom(format!("WebSocket request error: {}", e)))?;

    let (ws_stream, _response) = connect_async(request)
        .await
        .map_err(|e| SarvamError::Custom(format!("WebSocket connection error: {}", e)))?;

    let (write, read) = ws_stream.split();
    Ok(SttStream {
        write,
        read,
        endpoint_kind,
    })
}

fn push_bool(query: &mut Vec<(String, String)>, key: &str, value: Option<bool>) {
    if let Some(value) = value {
        query.push((key.to_string(), value.to_string()));
    }
}

fn push_number<T: ToString>(query: &mut Vec<(String, String)>, key: &str, value: Option<T>) {
    if let Some(value) = value {
        query.push((key.to_string(), value.to_string()));
    }
}

#[derive(Debug, Serialize)]
pub struct SttAudioMessage {
    pub audio: SttAudioPayload,
}

#[derive(Debug, Serialize)]
pub struct SttAudioPayload {
    pub data: String,
    pub sample_rate: String,
    pub encoding: String,
}

#[derive(Debug, Serialize)]
struct SttFlushMessage {
    flush: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SttDataMetrics {
    #[serde(default)]
    pub audio_duration: Option<f64>,
    #[serde(default)]
    pub processing_latency: Option<f64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SttTranscriptData {
    #[serde(default)]
    pub request_id: Option<String>,
    #[serde(default)]
    pub transcript: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub metrics: Option<SttDataMetrics>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl SttTranscriptData {
    pub fn text(&self) -> Option<&str> {
        self.transcript.as_deref().or(self.text.as_deref())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SttEventMessage {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SttErrorMessage {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub code: Option<i64>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub enum SttMessage {
    Transcript(SttTranscriptData),
    Event(SttEventMessage),
    Error(SttErrorMessage),
    Raw(serde_json::Value),
}

pub struct SttStream {
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
    endpoint_kind: SttEndpointKind,
}

impl SttStream {
    pub async fn send_audio_base64(
        &mut self,
        audio_base64: impl Into<String>,
        encoding: SttAudioEncoding,
        sample_rate: WebSocketSampleRate,
    ) -> Result<()> {
        let payload = SttAudioMessage {
            audio: SttAudioPayload {
                data: audio_base64.into(),
                sample_rate: sample_rate.as_str().to_string(),
                encoding: encoding.as_str().to_string(),
            },
        };
        let json = serde_json::to_string(&payload)
            .map_err(|e| SarvamError::Custom(format!("Serialization error: {}", e)))?;
        self.write
            .send(Message::Text(json.into()))
            .await
            .map_err(|e| SarvamError::Custom(format!("WebSocket send error: {}", e)))
    }

    pub async fn send_audio(
        &mut self,
        audio: impl AsRef<[u8]>,
        encoding: SttAudioEncoding,
        sample_rate: WebSocketSampleRate,
    ) -> Result<()> {
        let audio_base64 = base64::engine::general_purpose::STANDARD.encode(audio.as_ref());
        self.send_audio_base64(audio_base64, encoding, sample_rate)
            .await
    }

    pub async fn flush(&mut self) -> Result<()> {
        let json = serde_json::to_string(&SttFlushMessage { flush: true })
            .map_err(|e| SarvamError::Custom(format!("Serialization error: {}", e)))?;
        self.write
            .send(Message::Text(json.into()))
            .await
            .map_err(|e| SarvamError::Custom(format!("WebSocket send error: {}", e)))
    }

    pub async fn close(&mut self) -> Result<()> {
        self.write
            .close()
            .await
            .map_err(|e| SarvamError::Custom(format!("WebSocket close error: {}", e)))
    }

    pub async fn next(&mut self) -> Option<Result<SttMessage>> {
        while let Some(message) = self.read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    let parsed: serde_json::Value = match serde_json::from_str(&text) {
                        Ok(parsed) => parsed,
                        Err(err) => {
                            return Some(Err(SarvamError::Custom(format!(
                                "JSON parse error: {}",
                                err
                            ))))
                        }
                    };

                    let event_type = parsed
                        .get("type")
                        .and_then(|value| value.as_str())
                        .map(str::to_string);

                    match event_type.as_deref() {
                        Some("data") => {
                            match serde_json::from_value::<SttEnvelope<SttTranscriptData>>(
                                parsed.clone(),
                            ) {
                                Ok(message) => {
                                    return Some(Ok(SttMessage::Transcript(message.data)))
                                }
                                Err(err) => {
                                    return Some(Err(SarvamError::Custom(format!(
                                        "Transcript parse error: {}",
                                        err
                                    ))))
                                }
                            }
                        }
                        Some("speech_start") | Some("speech_end") | Some("transcript") => {
                            match serde_json::from_value::<SttEventMessage>(parsed.clone()) {
                                Ok(message) => return Some(Ok(SttMessage::Event(message))),
                                Err(err) => {
                                    return Some(Err(SarvamError::Custom(format!(
                                        "Event parse error: {}",
                                        err
                                    ))))
                                }
                            }
                        }
                        Some("error") => {
                            match serde_json::from_value::<SttErrorMessage>(parsed.clone()) {
                                Ok(message) => return Some(Ok(SttMessage::Error(message))),
                                Err(err) => {
                                    return Some(Err(SarvamError::Custom(format!(
                                        "Error parse error: {}",
                                        err
                                    ))))
                                }
                            }
                        }
                        _ => {
                            return Some(Ok(SttMessage::Raw(parsed)));
                        }
                    }
                }
                Ok(Message::Binary(_)) => {
                    return Some(Err(SarvamError::Custom(
                        "Unexpected binary message from STT WebSocket".into(),
                    )))
                }
                Ok(Message::Close(_)) => return None,
                Ok(_) => continue,
                Err(err) => {
                    return Some(Err(SarvamError::Custom(format!(
                        "WebSocket receive error: {}",
                        err
                    ))))
                }
            }
        }
        None
    }

    pub fn is_translate_endpoint(&self) -> bool {
        matches!(self.endpoint_kind, SttEndpointKind::SpeechToTextTranslate)
    }
}

impl std::fmt::Debug for SttStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SttStream")
            .field("endpoint_kind", &self.endpoint_kind)
            .finish()
    }
}

#[derive(Debug, Deserialize)]
struct SttEnvelope<T> {
    data: T,
}
