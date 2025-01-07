use crate::error::Result;
use bytes::Bytes;
use whisper_rs::{FullParams, WhisperContext, WhisperContextParameters};

#[derive(Clone)]
pub struct WhisperTranscriber {
    ctx: std::sync::Arc<WhisperContext>,
}

impl WhisperTranscriber {
    pub fn new() -> Result<Self> {
        let ctx =
            WhisperContext::new_with_params("ggml-tiny.bin", WhisperContextParameters::default())?;
        Ok(Self {
            ctx: std::sync::Arc::new(ctx),
        })
    }

    pub fn transcribe(&self, audio: &Bytes) -> Result<String> {
        let mut params = FullParams::new(whisper_rs::SamplingStrategy::Greedy { best_of: 0 });
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        let mut state = self.ctx.create_state()?;

        let samples: Vec<f32> = audio
            .chunks(4)
            .filter_map(|chunk| {
                if chunk.len() == 4 {
                    Some(f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                } else {
                    None
                }
            })
            .collect();

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
