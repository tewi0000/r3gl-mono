use fragile::Sticky;

use std::collections::VecDeque;
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

use color_eyre::eyre::{Report, Result, ContextCompat};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat, SupportedStreamConfigRange};
use log::{error, info, warn};
use samplerate::{ConverterType, Samplerate};
use symphonia::core::audio::{SampleBuffer, AudioBufferRef, SignalSpec};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::{MediaSource, MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;
use symphonia::default;

pub use symphonia::core::probe::Hint;

#[derive(Debug)]
struct AudioBuffer {
    channel  : Mutex<Receiver<Option<Vec<f32>>>>,
    buffer   : VecDeque<f32>,
    
    done     : bool,
    len      : usize,
}

impl AudioBuffer {
    fn new(song: &Audio, sample_rate: u32, channel_count: usize) -> Result<AudioBuffer> {
        // Should produce blocks of at least about 1024 samples x 2 channels.
        const DECODE_BLOCK_SIZE: usize = 1029 * 48000 / 44100 * 2;

        // Reorder samples, this is really fast and gets us ownership over the song
        let samples = {
            let sample_count = song.samples[0].len();
            let mut samples = vec![0.0; channel_count * sample_count];
            for chan in 0 .. channel_count {
                if chan < 2 || chan < song.samples.len() {
                    for sample in 0..sample_count {
                        samples[sample * channel_count as usize + chan] = song.samples[chan % song.samples.len()][sample];
                    }
                };
            }

            samples
        };
        
        let len = samples.len() * sample_rate as usize / song.sample_rate as usize;
        let (tx, rx) = mpsc::channel();
        let source_sample_rate = song.sample_rate;
        thread::spawn(move || {
            let converter = Samplerate::new(
                ConverterType::SincBestQuality,
                source_sample_rate,
                sample_rate,
                channel_count,
            ).unwrap();

            let last = samples.len() / DECODE_BLOCK_SIZE;
            for i in 0 ..= last {
                let pos = i * DECODE_BLOCK_SIZE;
                let samples = &samples[pos..((pos + DECODE_BLOCK_SIZE).min(samples.len()))];
                let processed_samples = if   i == last { converter.process_last(samples) }
                                        else 		   { converter.process	   (samples) }.unwrap();
                
                // Dropping the other end of the channel will stop decoding
                if tx.send(Some(processed_samples)).is_err() {
                    break;
                }
            }
        });

        return Ok(AudioBuffer {
            channel	 : Mutex::new(rx),
            buffer	 : VecDeque::new(),
            done	 : false,
            len,
        });
    }

    fn read_samples(&mut self, pos: usize, count: usize) -> (Vec<f32>, bool) {
        let channel = self.channel.lock().unwrap();
        if !self.done {
            while self.buffer.len() < pos + count {
                if let Ok(Some(buf)) = channel.recv() {
                    self.buffer.append(&mut VecDeque::from(buf));
                } else {
                    self.done = true;
                    break;
                }
            }
        }

        let mut vec = Vec::new();
        let mut done = false;
        for i in 0 .. count {
            if let Some(sample) = self.buffer.get(pos + i) {
                vec.push(*sample);
            } else {
                done = true;
                break;
            }
        }
        
        return (vec, done);
    }

    fn len(&self) -> usize {
        return self.len;
    }
}

struct PlayerState {
    audio_buffer  : RwLock<Option<AudioBuffer>>,
    
    position      : RwLock<usize>,
    paused        : RwLock<bool>,
    finished      : RwLock<bool>,

    sample_rate	  : u32,
    channel_count : usize,
}

impl PlayerState {
    fn new(channel_count: u32, sample_rate: u32) -> PlayerState {
        return PlayerState {
            audio_buffer  : RwLock::new(None),
            position      : RwLock::new(0),
            paused        : RwLock::new(true),
            finished      : RwLock::new(false),
            sample_rate   : sample_rate,
            channel_count : channel_count as usize,
        };
    }
    
    fn write_samples<T: Sample>(&self, data: &mut [T]) {
        if *self.paused.read().unwrap() {
            for sample in data.iter_mut() {
                *sample = Sample::from(&0.0);
            }

            return;
        }
        
        let mut audio_buffer = self.audio_buffer.write().unwrap();
        if let Some(audio_buffer) = audio_buffer.as_mut() {
            let data_len = data.len();
            let mut position = self.position.write().unwrap();
            let (samples, is_final) = audio_buffer.read_samples(*position, data_len);
            for (i, sample) in data.iter_mut().enumerate() {
                if i >= samples.len() {
                    break;
                }

                *sample = Sample::from(&samples[i]);
            }
            
            *position += data_len;

            if is_final {
                *self.paused.write().unwrap() = true;
                *self.finished.write().unwrap() = true;
            }
        }
    }

    fn decode_song(&self, song: &Audio) -> Result<AudioBuffer> {
        return AudioBuffer::new(song, self.sample_rate, self.channel_count);
    }
    
    fn load(&self, song: &Audio) -> Result<()> {
        let samples = self.decode_song(song)?;
        *self.position.write().unwrap() = 0; 
        *self.audio_buffer.write().unwrap() = Some(samples);
        return Ok(());
    }
    fn play(&self, song: &Audio) -> Result<()> {
        self.set_paused(false);
        return self.load(song);
    }
    fn stop(&self) {
        self.set_paused(true);

        *self.audio_buffer.write().unwrap() = None;
    }
    fn pause(&self) {
        let mut paused = self.paused.write().unwrap();
        *paused = !*paused;
    }
    fn set_paused(&self, state: bool) {
        *self.paused.write().unwrap() = state;
    }
    fn seek(&self, position: usize) {
        *self.position.write().unwrap() = position;
    }
}

pub struct Player {
    _stream		 : Sticky<Box<dyn StreamTrait>>,
    player_state : Arc<PlayerState>,
}

impl Player {
    pub fn new() -> Result<Player> {
        let device = {
            let mut selected_host = cpal::default_host();
            for host in cpal::available_hosts() {
                if host.name().to_lowercase().contains("jack") {
                    selected_host = cpal::host_from_id(host)?;
                }
            }

            info!("Selected Host: {:?}", selected_host.id());
            let mut selected_device = selected_host
                .default_output_device()
                .ok_or_else(|| Report::msg("No output device found"))?;

            for device in selected_host.output_devices()? {
                if let Ok(name) = device.name().map(|s| s.to_lowercase()) {
                    if name.contains("pipewire") || name.contains("pulse") || name.contains("jack")
                    {
                        selected_device = device;
                    }
                }
            }

            info!("Selected Device: {}", selected_device.name().unwrap_or_else(|_| "Unknown".to_string()));
            selected_device
        };

        let mut supported_configs = device.supported_output_configs()?.collect::<Vec<_>>();
        fn rank_supported_config(config: &SupportedStreamConfigRange) -> u32 {
            let chans = config.channels() as u32;
            let channel_rank = match chans {
                0 => 0,
                1 => 1,
                2 => 4,
                4 => 3,
                _ => 2,
            };
            
            let min_sample_rank = if config.min_sample_rate().0 <= 48000 { 3 } else { 0 };
            let max_sample_rank = if config.max_sample_rate().0 >= 48000 { 3 } else { 0 };
            let sample_format_rank = if config.sample_format() == SampleFormat::F32 { 4 } else { 0 };
            channel_rank + min_sample_rank + max_sample_rank + sample_format_rank
        }
        
        supported_configs.sort_by_key(|c_2| std::cmp::Reverse(rank_supported_config(c_2)));
        let supported_config = supported_configs.into_iter().next().ok_or_else(|| Report::msg("No supported output config"))?;

        let sample_rate_range = supported_config.min_sample_rate().0..supported_config.max_sample_rate().0;
        let supported_config = match sample_rate_range {
            rate if rate.contains(&48000) => supported_config.with_sample_rate(cpal::SampleRate(48000)),
            rate if rate.contains(&44100) => supported_config.with_sample_rate(cpal::SampleRate(48000)),
            rate if rate.end <= 48000     => supported_config.with_sample_rate(cpal::SampleRate(48000)),
                                        _ => supported_config.with_sample_rate(cpal::SampleRate(sample_rate_range.start)) };

        let sample_format = supported_config.sample_format();
        let sample_rate = supported_config.sample_rate().0;
        let channel_count = supported_config.channels();
        let config = supported_config.into();
        let err_fn = |err| error!("Playback error: {}", err);
        let player_state = Arc::new(PlayerState::new(channel_count as u32, sample_rate));
        info!("SR, CC, SF: {sample_rate}, {channel_count}, {sample_format:?}");

        let stream = {
            let player_state = player_state.clone();
            match sample_format {
                SampleFormat::F32 => device.build_output_stream(&config, move |data, _| player_state.write_samples::<f32>(data), err_fn)?,
                SampleFormat::I16 => device.build_output_stream(&config, move |data, _| player_state.write_samples::<i16>(data), err_fn)?,
                SampleFormat::U16 => device.build_output_stream(&config, move |data, _| player_state.write_samples::<u16>(data), err_fn)?, } };

        stream.play()?;

        return Ok(Player {
            _stream: Sticky::new(Box::new(stream)),
            player_state: player_state,
        });
    }
    
    fn sample_length(&self) -> Duration {
        return Duration::from_nanos(1000000000 / (self.player_state.sample_rate as u64 * self.player_state.channel_count as u64));
    }

    pub fn finished(&self) -> bool {
        let finished = *self.player_state.finished.read().unwrap();
        return finished;
    }
    pub fn length(&self) -> Result<Duration> {
        let duration_per_sample = self.sample_length();
        let audio_buffer = self.player_state.audio_buffer.read().unwrap();
        return audio_buffer.as_ref().map(|audio_buffer| {
            audio_buffer.len() as u32 * duration_per_sample
        }).context("No audio loaded");
    }
    pub fn get_time(&self) -> Duration {
        let duration_per_sample = self.sample_length();
        let position = *self.player_state.position.read().unwrap();
        return position as u32 * duration_per_sample;
    }
    pub fn set_time(&mut self, time: Duration) {
        let duration_per_sample = self.sample_length();
        let samples = (time.as_nanos() / duration_per_sample.as_nanos()) as usize;
        self.player_state.seek(samples);
    }

    pub fn load(&self, song: &Audio) -> Result<()> {
        return self.player_state.load(song);
    }
    pub fn play(&self, song: &Audio) -> Result<()> {
        return self.player_state.play(song);
    }
    pub fn stop(&self) {
        return self.player_state.stop();
    }
    pub fn pause(&self) {
        self.player_state.pause();
    }
    pub fn set_paused(&self, state: bool) {
        self.player_state.set_paused(state);
    }
    pub fn is_paused(&self) -> bool {
        return *self.player_state.paused.read().unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct Audio {
    samples: Vec<Vec<f32>>,
    sample_rate: u32,
    channel_count: usize,
}

impl Audio {
    pub fn new(reader: Box<dyn MediaSource>, hint: &Hint) -> Result<Audio> {
        let media_source_stream = MediaSourceStream::new(reader, MediaSourceStreamOptions::default());
        let options = FormatOptions { enable_gapless: true, ..FormatOptions::default() };
        let meta = MetadataOptions::default();
        let mut probe = default::get_probe().format(hint, media_source_stream, &options, &meta)?;

        let mut decoder = default::get_codecs().make(
            &probe
                .format
                .default_track()
                .ok_or_else(|| Report::msg("No default track in media file"))?
                .codec_params,
            &DecoderOptions::default(),
        )?;

        fn decode_buffer(buffer: AudioBufferRef, spec: SignalSpec, song_samples: &mut Vec<Vec<f32>>) {
            if buffer.frames() > 0 {
                let mut samples = SampleBuffer::new(buffer.frames() as u64, spec);
                samples.copy_interleaved_ref(buffer);
                for frame in samples.samples().chunks(spec.channels.count()) {
                    for (chan, sample) in frame.iter().enumerate() {
                        song_samples[chan].push(*sample)
                    }
                }
            } else {
                warn!("Empty packet encountered while loading song!");
            }
        }

        let mut song = loop {
            match probe.format.next_packet() {
                Ok(packet) => {
                    let buffer = decoder.decode(&packet)?;
                    let spec = *buffer.spec();

                    let mut song_samples = vec![Vec::new(); spec.channels.count()];
                    decode_buffer(buffer, spec, &mut song_samples);
                    
                    break Audio {
                        samples: song_samples,
                        sample_rate: spec.rate,
                        channel_count: spec.channels.count(),
                    };
                }
                
                Err(SymphoniaError::IoError(_)) => return Err(Report::msg("No song data decoded")),
                Err(e) => return Err(e.into()),
            }
        };


        loop {
            match probe.format.next_packet() {
                Ok(packet) => {
                    let buffer = decoder.decode(&packet)?;
                    let spec = *buffer.spec();

                    if spec.rate != song.sample_rate || spec.channels.count() != song.channel_count {
                        return Err(Report::msg("Sample rate or channel count of decoded does not match previous sample rate"));
                    }

                    decode_buffer(buffer, spec, &mut song.samples);
                }
                
                Err(SymphoniaError::IoError(_)) => break,
                Err(e) => return Err(e.into()),
            }
        }
        
        return Ok(song);
    }

    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Audio> {
        let mut hint = Hint::new();
        if let Some(extension) = path.as_ref().extension().and_then(|s| s.to_str()) {
            hint.with_extension(extension);
        }

        Self::new(Box::new(std::fs::File::open(path)?), &hint)
    }
}
