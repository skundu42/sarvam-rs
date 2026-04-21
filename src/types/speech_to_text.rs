use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpeechToTextModel {
    #[serde(rename = "saarika:v2.5")]
    SaarikaV2_5,
    #[serde(rename = "saaras:v3")]
    SaarasV3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SttMode {
    Transcribe,
    Translate,
    Verbatim,
    Translit,
    Codemix,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpeechToTextLanguage {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "hi-IN")]
    HiIn,
    #[serde(rename = "bn-IN")]
    BnIn,
    #[serde(rename = "kn-IN")]
    KnIn,
    #[serde(rename = "ml-IN")]
    MlIn,
    #[serde(rename = "mr-IN")]
    MrIn,
    #[serde(rename = "od-IN")]
    OdIn,
    #[serde(rename = "pa-IN")]
    PaIn,
    #[serde(rename = "ta-IN")]
    TaIn,
    #[serde(rename = "te-IN")]
    TeIn,
    #[serde(rename = "en-IN")]
    EnIn,
    #[serde(rename = "gu-IN")]
    GuIn,
    #[serde(rename = "as-IN")]
    AsIn,
    #[serde(rename = "ur-IN")]
    UrIn,
    #[serde(rename = "ne-IN")]
    NeIn,
    #[serde(rename = "kok-IN")]
    KokIn,
    #[serde(rename = "ks-IN")]
    KsIn,
    #[serde(rename = "sd-IN")]
    SdIn,
    #[serde(rename = "sa-IN")]
    SaIn,
    #[serde(rename = "sat-IN")]
    SatIn,
    #[serde(rename = "mni-IN")]
    MniIn,
    #[serde(rename = "brx-IN")]
    BrxIn,
    #[serde(rename = "mai-IN")]
    MaiIn,
    #[serde(rename = "doi-IN")]
    DoiIn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputAudioCodec {
    #[serde(rename = "wav")]
    Wav,
    #[serde(rename = "x-wav")]
    XWav,
    #[serde(rename = "wave")]
    Wave,
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "mpeg")]
    Mpeg,
    #[serde(rename = "mpeg3")]
    Mpeg3,
    #[serde(rename = "x-mp3")]
    XMp3,
    #[serde(rename = "x-mpeg-3")]
    XMpeg3,
    #[serde(rename = "aac")]
    Aac,
    #[serde(rename = "x-aac")]
    XAac,
    #[serde(rename = "aiff")]
    Aiff,
    #[serde(rename = "x-aiff")]
    XAiff,
    #[serde(rename = "ogg")]
    Ogg,
    #[serde(rename = "opus")]
    Opus,
    #[serde(rename = "flac")]
    Flac,
    #[serde(rename = "x-flac")]
    XFlac,
    #[serde(rename = "mp4")]
    Mp4,
    #[serde(rename = "x-m4a")]
    XM4a,
    #[serde(rename = "amr")]
    Amr,
    #[serde(rename = "x-ms-wma")]
    XMsWma,
    #[serde(rename = "webm")]
    Webm,
    #[serde(rename = "pcm_s16le")]
    PcmS16le,
    #[serde(rename = "pcm_l16")]
    PcmL16,
    #[serde(rename = "pcm_raw")]
    PcmRaw,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpeechToTextResponse {
    pub request_id: Option<String>,
    pub transcript: String,
    #[serde(default)]
    pub timestamps: Option<Timestamps>,
    #[serde(default)]
    pub diarized_transcript: Option<DiarizedTranscript>,
    pub language_code: Option<String>,
    #[serde(default)]
    pub language_probability: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Timestamps {
    pub words: Vec<String>,
    pub start_time_seconds: Vec<f64>,
    pub end_time_seconds: Vec<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiarizedTranscript {
    pub entries: Vec<DiarizedEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiarizedEntry {
    pub transcript: String,
    pub start_time_seconds: f64,
    pub end_time_seconds: f64,
    pub speaker_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpeechToTextTranslateModel {
    #[serde(rename = "saaras:v2.5")]
    SaarasV2_5,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpeechToTextTranslateResponse {
    pub request_id: Option<String>,
    pub transcript: String,
    pub language_code: Option<String>,
    #[serde(default)]
    pub diarized_transcript: Option<DiarizedTranscript>,
    #[serde(default)]
    pub language_probability: Option<f64>,
}
