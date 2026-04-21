use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TranslateSourceLanguage {
    Auto,
    #[serde(rename = "bn-IN")]
    BnIn,
    #[serde(rename = "en-IN")]
    EnIn,
    #[serde(rename = "gu-IN")]
    GuIn,
    #[serde(rename = "hi-IN")]
    HiIn,
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
    #[serde(rename = "as-IN")]
    AsIn,
    #[serde(rename = "brx-IN")]
    BrxIn,
    #[serde(rename = "doi-IN")]
    DoiIn,
    #[serde(rename = "kok-IN")]
    KokIn,
    #[serde(rename = "ks-IN")]
    KsIn,
    #[serde(rename = "mai-IN")]
    MaiIn,
    #[serde(rename = "mni-IN")]
    MniIn,
    #[serde(rename = "ne-IN")]
    NeIn,
    #[serde(rename = "sa-IN")]
    SaIn,
    #[serde(rename = "sat-IN")]
    SatIn,
    #[serde(rename = "sd-IN")]
    SdIn,
    #[serde(rename = "ur-IN")]
    UrIn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TranslateTargetLanguage {
    #[serde(rename = "bn-IN")]
    BnIn,
    #[serde(rename = "en-IN")]
    EnIn,
    #[serde(rename = "gu-IN")]
    GuIn,
    #[serde(rename = "hi-IN")]
    HiIn,
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
    #[serde(rename = "as-IN")]
    AsIn,
    #[serde(rename = "brx-IN")]
    BrxIn,
    #[serde(rename = "doi-IN")]
    DoiIn,
    #[serde(rename = "kok-IN")]
    KokIn,
    #[serde(rename = "ks-IN")]
    KsIn,
    #[serde(rename = "mai-IN")]
    MaiIn,
    #[serde(rename = "mni-IN")]
    MniIn,
    #[serde(rename = "ne-IN")]
    NeIn,
    #[serde(rename = "sa-IN")]
    SaIn,
    #[serde(rename = "sat-IN")]
    SatIn,
    #[serde(rename = "sd-IN")]
    SdIn,
    #[serde(rename = "ur-IN")]
    UrIn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SpeakerGender {
    Male,
    Female,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TranslateMode {
    Formal,
    #[serde(rename = "modern-colloquial")]
    ModernColloquial,
    #[serde(rename = "classic-colloquial")]
    ClassicColloquial,
    #[serde(rename = "code-mixed")]
    CodeMixed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TranslateModel {
    #[serde(rename = "mayura:v1")]
    MayuraV1,
    #[serde(rename = "sarvam-translate:v1")]
    SarvamTranslateV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TransliterateMode {
    Roman,
    #[serde(rename = "fully-native")]
    FullyNative,
    #[serde(rename = "spoken-form-in-native")]
    SpokenFormInNative,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NumeralsFormat {
    #[serde(rename = "international")]
    International,
    #[serde(rename = "native")]
    Native,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpokenFormNumeralsFormat {
    #[serde(rename = "english")]
    English,
    #[serde(rename = "native")]
    Native,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TransliterateSourceLanguage {
    Auto,
    #[serde(rename = "bn-IN")]
    BnIn,
    #[serde(rename = "en-IN")]
    EnIn,
    #[serde(rename = "gu-IN")]
    GuIn,
    #[serde(rename = "hi-IN")]
    HiIn,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TransliterateTargetLanguage {
    #[serde(rename = "bn-IN")]
    BnIn,
    #[serde(rename = "en-IN")]
    EnIn,
    #[serde(rename = "gu-IN")]
    GuIn,
    #[serde(rename = "hi-IN")]
    HiIn,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TextToSpeechLanguage {
    #[serde(rename = "bn-IN")]
    BnIn,
    #[serde(rename = "en-IN")]
    EnIn,
    #[serde(rename = "gu-IN")]
    GuIn,
    #[serde(rename = "hi-IN")]
    HiIn,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextToSpeechSpeaker {
    Anushka,
    Abhilash,
    Manisha,
    Vidya,
    Arya,
    Karun,
    Hitesh,
    Aditya,
    Ritu,
    Priya,
    Neha,
    Rahul,
    Pooja,
    Rohan,
    Simran,
    Kavya,
    Amit,
    Dev,
    Ishita,
    Shreya,
    Ratan,
    Varun,
    Manan,
    Sumit,
    Roopa,
    Kabir,
    Aayan,
    Shubh,
    Ashutosh,
    Advait,
    Anand,
    Tanya,
    Tarun,
    Sunny,
    Mani,
    Gokul,
    Vijay,
    Shruti,
    Suhani,
    Mohit,
    Kavitha,
    Rehan,
    Soham,
    Rupali,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpeechSampleRate {
    #[serde(rename = "8000")]
    Hz8000,
    #[serde(rename = "16000")]
    Hz16000,
    #[serde(rename = "22050")]
    Hz22050,
    #[serde(rename = "24000")]
    Hz24000,
    #[serde(rename = "32000")]
    Hz32000,
    #[serde(rename = "44100")]
    Hz44100,
    #[serde(rename = "48000")]
    Hz48000,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextToSpeechModel {
    #[serde(rename = "bulbul:v2")]
    BulbulV2,
    #[serde(rename = "bulbul:v3")]
    BulbulV3,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextToSpeechOutputAudioCodec {
    Mp3,
    #[serde(rename = "linear16")]
    Linear16,
    Mulaw,
    Alaw,
    Opus,
    Flac,
    Aac,
    Wav,
}

#[cfg(test)]
mod tests {
    use super::TextToSpeechOutputAudioCodec;

    #[test]
    fn text_to_speech_output_audio_codec_serializes_to_api_values() {
        assert_eq!(
            serde_json::to_string(&TextToSpeechOutputAudioCodec::Mp3).unwrap(),
            "\"mp3\""
        );
        assert_eq!(
            serde_json::to_string(&TextToSpeechOutputAudioCodec::Linear16).unwrap(),
            "\"linear16\""
        );
        assert_eq!(
            serde_json::to_string(&TextToSpeechOutputAudioCodec::Wav).unwrap(),
            "\"wav\""
        );
    }
}
