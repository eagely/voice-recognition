use std::{io::Cursor, sync::Arc};

use crate::error::{Error, Result};
use bytes::Bytes;
use hound::{SampleFormat, WavReader};
use whisper_rs::{FullParams, WhisperContext, WhisperContextParameters};

#[derive(Clone)]
pub struct WhisperTranscriber {
    context: Arc<WhisperContext>,
}

impl WhisperTranscriber {
    pub fn new() -> Result<Self> {
        let mut params = WhisperContextParameters::default();
        params.use_gpu = true;
        let context = Arc::new(WhisperContext::new_with_params("model.bin", params)?);
        Ok(Self { context })
    }

    pub fn transcribe(&self, audio: &Bytes) -> Result<String> {
        let cursor = Cursor::new(audio);
        let mut reader = WavReader::new(cursor).map_err(|e| Error::AudioProcesing(e))?;

        let spec = reader.spec();
        if spec.channels != 1 || spec.sample_rate != 16000 {
            return Err(Error::Codec("WAV must be 16kHz mono".to_string()));
        }

        let samples: Vec<f32> = match spec.sample_format {
            SampleFormat::Float => reader
                .samples::<f32>()
                .map(|s| s.map_err(|e| Error::AudioProcesing(e)))
                .collect::<Result<_>>()?,
            SampleFormat::Int => reader
                .samples::<i16>()
                .map(|s| s.map_err(|e| Error::AudioProcesing(e)))
                .map(|s| Ok(s? as f32 / 32768.0))
                .collect::<Result<_>>()?,
        };

        let mut params = FullParams::new(whisper_rs::SamplingStrategy::Greedy { best_of: 5 });

        params.set_translate(true);
        params.set_language(Some("en"));
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        let mut state = self.context.create_state()?;
        state.full(params, &samples)?;

        let num_segments = state.full_n_segments()?;
        let mut text = String::new();

        for i in 0..num_segments {
            text.push_str(&state.full_get_segment_text(i)?);
            text.push(' ');
        }

        Ok(text.trim().to_string())
    }
}
