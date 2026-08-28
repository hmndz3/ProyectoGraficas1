// Audio sintetizado en memoria: efectos de sonido y música ambiental
// original por nivel. Se generan muestras y se empaquetan como WAV.

use macroquad::audio::{load_sound_from_bytes, play_sound, stop_sound, PlaySoundParams, Sound};

const RATE: u32 = 22050;

fn wav_bytes(samples: &[f32]) -> Vec<u8> {
    let n = samples.len() as u32;
    let data_len = n * 2;
    let mut out = Vec::with_capacity(44 + data_len as usize);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes()); // PCM
    out.extend_from_slice(&1u16.to_le_bytes()); // mono
    out.extend_from_slice(&RATE.to_le_bytes());
    out.extend_from_slice(&(RATE * 2).to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    for s in samples {
        out.extend_from_slice(&((s.clamp(-1.0, 1.0) * 32000.0) as i16).to_le_bytes());
    }
    out
}

fn rngf(state: &mut u32) -> f32 {
    *state = state.wrapping_mul(1664525).wrapping_add(1013904223);
    (*state >> 8) as f32 / 16777216.0 * 2.0 - 1.0
}

// ------------------------- efectos -------------------------

/// disparo: rafaga de ruido + barrido descendente
fn sfx_shoot() -> Vec<f32> {
    let n = (RATE as f32 * 0.22) as usize;
    let mut rng = 77u32;
    (0..n)
        .map(|i| {
            let t = i as f32 / RATE as f32;
            let env = (-t * 22.0).exp();
            let f = 750.0 - t * 2400.0;
            let sq = if (t * f.max(60.0)).fract() < 0.5 {
                0.5
            } else {
                -0.5
            };
            (rngf(&mut rng) * 0.6 + sq * 0.5) * env * 0.8
        })
        .collect()
}

/// recoger carta: arpegio brillante
fn sfx_pickup() -> Vec<f32> {
    let notes = [659.25f32, 880.0, 1318.5];
    let n = (RATE as f32 * 0.45) as usize;
    (0..n)
        .map(|i| {
            let t = i as f32 / RATE as f32;
            let idx = ((t / 0.13) as usize).min(2);
            let lt = t - idx as f32 * 0.13;
            let env = (-lt * 9.0).exp();
            ((lt * notes[idx] * std::f32::consts::TAU).sin()
                + 0.4 * (lt * notes[idx] * 2.0 * std::f32::consts::TAU).sin())
                * env
                * 0.5
        })
        .collect()
}

/// espiritu disipado: soplo descendente
fn sfx_poof() -> Vec<f32> {
    let n = (RATE as f32 * 0.4) as usize;
    let mut rng = 991u32;
    let mut lp = 0.0f32;
    (0..n)
        .map(|i| {
            let t = i as f32 / RATE as f32;
            let env = (-t * 7.0).exp();
            let alpha = 0.4 - t * 0.7;
            lp += (rngf(&mut rng) - lp) * alpha.clamp(0.02, 0.5);
            lp * env * 1.6
        })
        .collect()
}

/// portal: barrido ascendente con vibrato
fn sfx_portal() -> Vec<f32> {
    let n = (RATE as f32 * 0.9) as usize;
    let mut phase = 0.0f32;
    (0..n)
        .map(|i| {
            let t = i as f32 / RATE as f32;
            let env = (t * 12.0).min(1.0) * (1.0 - t / 0.9).max(0.0);
            let f = 180.0 + t * 750.0 + (t * 32.0).sin() * 28.0;
            phase += f * std::f32::consts::TAU / RATE as f32;
            (phase.sin() + 0.5 * (phase * 1.5).sin()) * env * 0.5
        })
        .collect()
}

/// fanfarria de victoria
fn sfx_win() -> Vec<f32> {
    let notes = [523.25f32, 659.25, 783.99, 1046.5];
    let n = (RATE as f32 * 1.4) as usize;
    (0..n)
        .map(|i| {
            let t = i as f32 / RATE as f32;
            let idx = ((t / 0.28) as usize).min(3);
            let lt = t - idx as f32 * 0.28;
            let hold = if idx == 3 { 2.2 } else { 6.0 };
            let env = (-lt * hold).exp();
            ((lt * notes[idx] * std::f32::consts::TAU).sin()
                + 0.5 * (lt * notes[idx] * 1.5 * std::f32::consts::TAU).sin()
                + 0.25 * (lt * notes[idx] * 2.0 * std::f32::consts::TAU).sin())
                * env
                * 0.45
        })
        .collect()
}

// ------------------------- música -------------------------

/// Pad ambiental generativo: progresión de acordes con seno + armónicos.
/// Cada nivel recibe su propia escala y carácter.
fn music_loop(chords: &[[f32; 3]], bass: &[f32], bright: f32, seed: u32) -> Vec<f32> {
    let chord_dur = 3.0f32;
    let total = chords.len() as f32 * chord_dur;
    let n = (RATE as f32 * total) as usize;
    let mut rng = seed;
    // campanitas aleatorias suaves
    let mut bells: Vec<(f32, f32)> = Vec::new();
    for i in 0..(chords.len() * 2) {
        let t0 = (rngf(&mut rng) * 0.5 + 0.5) * total;
        let ci = ((t0 / chord_dur) as usize).min(chords.len() - 1);
        let note = chords[ci][(i % 3)] * 2.0;
        bells.push((t0, note));
    }

    (0..n)
        .map(|i| {
            let t = i as f32 / RATE as f32;
            let ci = ((t / chord_dur) as usize).min(chords.len() - 1);
            let lt = (t % chord_dur) / chord_dur;
            // crossfade triangular dentro del acorde
            let env = (lt * std::f32::consts::PI).sin() * 0.85 + 0.15;
            let mut s = 0.0f32;
            for (k, f) in chords[ci].iter().enumerate() {
                let det = 1.0 + (k as f32 - 1.0) * 0.0012;
                s += (t * f * det * std::f32::consts::TAU).sin() * 0.30;
                s += (t * f * 2.0 * std::f32::consts::TAU).sin() * 0.06 * bright;
            }
            s += (t * bass[ci % bass.len()] * std::f32::consts::TAU).sin() * 0.35;
            s *= env;
            for (bt, bf) in &bells {
                let dt = t - bt;
                if dt > 0.0 && dt < 1.2 {
                    s +=
                        (dt * bf * std::f32::consts::TAU).sin() * (-dt * 4.0).exp() * 0.12 * bright;
                }
            }
            s * 0.55
        })
        .collect()
}

pub struct Audio {
    pub shoot: Sound,
    pub pickup: Sound,
    pub poof: Sound,
    pub portal: Sound,
    pub win: Sound,
    pub music: Vec<Sound>,
    playing: Option<usize>,
}

impl Audio {
    pub async fn load() -> Audio {
        let load = |s: Vec<f32>| wav_bytes(&s);
        // I The Fool: mayor pentatónica, luminosa
        let fool = music_loop(
            &[
                [261.63, 329.63, 392.0],
                [293.66, 392.0, 440.0],
                [329.63, 440.0, 523.25],
                [261.63, 392.0, 523.25],
            ],
            &[65.41, 73.42, 82.41, 65.41],
            1.0,
            42,
        );
        // XII The Hanged Man: suspendida, en vaivén
        let hanged = music_loop(
            &[
                [293.66, 349.23, 440.0],
                [261.63, 349.23, 415.3],
                [293.66, 369.99, 440.0],
                [246.94, 329.63, 415.3],
            ],
            &[73.42, 65.41, 61.74, 55.0],
            0.5,
            77,
        );
        // IX The Hermit: drone menor, profundo
        let hermit = music_loop(
            &[
                [220.0, 261.63, 329.63],
                [196.0, 246.94, 329.63],
                [174.61, 220.0, 293.66],
                [164.81, 220.0, 329.63],
            ],
            &[55.0, 49.0, 43.65, 41.2],
            0.25,
            13,
        );

        Audio {
            shoot: load_sound_from_bytes(&load(sfx_shoot())).await.unwrap(),
            pickup: load_sound_from_bytes(&load(sfx_pickup())).await.unwrap(),
            poof: load_sound_from_bytes(&load(sfx_poof())).await.unwrap(),
            portal: load_sound_from_bytes(&load(sfx_portal())).await.unwrap(),
            win: load_sound_from_bytes(&load(sfx_win())).await.unwrap(),
            music: vec![
                load_sound_from_bytes(&wav_bytes(&fool)).await.unwrap(),
                load_sound_from_bytes(&wav_bytes(&hanged)).await.unwrap(),
                load_sound_from_bytes(&wav_bytes(&hermit)).await.unwrap(),
            ],
            playing: None,
        }
    }

    pub fn sfx(&self, s: &Sound) {
        play_sound(
            s,
            PlaySoundParams {
                looped: false,
                volume: 0.8,
            },
        );
    }

    pub fn play_music(&mut self, level: usize) {
        if self.playing == Some(level) {
            return;
        }
        self.stop_music();
        play_sound(
            &self.music[level],
            PlaySoundParams {
                looped: true,
                volume: 0.4,
            },
        );
        self.playing = Some(level);
    }

    pub fn stop_music(&mut self) {
        if let Some(i) = self.playing.take() {
            stop_sound(&self.music[i]);
        }
    }
}
