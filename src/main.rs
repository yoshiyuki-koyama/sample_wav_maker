extern crate wav_f64vec;
use wav_f64vec::*;

mod create_wave;
use create_wave::*;

mod error;
use error::*;

use std::path::PathBuf;

const ARG_NUM_MAX: usize = 20;

const DEFAULT_WAVE_FORMAT: WaveFormat = WaveFormat {
    id: 1,
    channel: 1,
    sampling_rate: 16000,
    bits: 16,
};

#[derive(Clone, Copy, PartialEq)]
enum CommandKind {
    New,
    Add,
}

#[derive(Clone, Copy, PartialEq)]
enum OptionID {
    Format,
    SecLength,
    NumLength,
    InputPath,
    OutputPath,
    OverWrite,
    WaveShape,
    Amplitude,
    Frequency,
    SecSpan,
    NumSpan,
    PiPhase,
}

#[derive(Clone, Copy, PartialEq)]
enum OptionKind {
    FormatOption,
    WaveOption,
    PathOption,
}

struct CommandOption {
    id: OptionID,
    kind: OptionKind,
    op_num: Option<usize>,
    count: usize,
}

impl CommandOption {
    fn new(id: OptionID) -> CommandOption {
        CommandOption {
            id: id,
            kind: match id {
                OptionID::Format => OptionKind::FormatOption,
                OptionID::SecLength => OptionKind::FormatOption,
                OptionID::NumLength => OptionKind::FormatOption,
                OptionID::InputPath => OptionKind::PathOption,
                OptionID::OutputPath => OptionKind::PathOption,
                OptionID::OverWrite => OptionKind::PathOption,
                OptionID::WaveShape => OptionKind::WaveOption,
                OptionID::Amplitude => OptionKind::WaveOption,
                OptionID::Frequency => OptionKind::WaveOption,
                OptionID::SecSpan => OptionKind::WaveOption,
                OptionID::NumSpan => OptionKind::WaveOption,
                OptionID::PiPhase => OptionKind::WaveOption,
            },
            op_num: match id {
                OptionID::Format => Some(4),
                OptionID::SecLength => Some(1),
                OptionID::NumLength => Some(1),
                OptionID::InputPath => Some(1),
                OptionID::OutputPath => Some(1),
                OptionID::OverWrite => Some(0),
                OptionID::WaveShape => Some(0),
                OptionID::Amplitude => Some(1),
                OptionID::Frequency => Some(1),
                OptionID::SecSpan => Some(2),
                OptionID::NumSpan => Some(2),
                OptionID::PiPhase => Some(1),
            },
            count: 0,
        }
    }
}

struct FormatOptions {
    op_format: Option<WaveFormat>,
    op_sec_len: Option<f64>,
    op_num_len: Option<usize>,
}

impl FormatOptions {
    fn new() -> FormatOptions {
        FormatOptions {
            op_format: None,
            op_sec_len: None,
            op_num_len: None,
        }
    }
}

struct PathOptions {
    op_output_path: Option<PathBuf>,
    op_input_path: Option<PathBuf>,
    overwrite: bool,
}

impl PathOptions {
    fn new() -> PathOptions {
        PathOptions {
            op_output_path: None,
            op_input_path: None,
            overwrite: false,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum WaveShape {
    Sine,
    Triangle,
    Square,
}
struct WaveOptions {
    op_shape: Option<WaveShape>,
    op_amplitude: Option<f64>,
    op_frequency: Option<f64>,
    op_pi_phase: Option<f64>,
    op_sec_span: Option<(f64, f64)>,
    op_num_span: Option<(usize, usize)>,
}

impl WaveOptions {
    fn new() -> WaveOptions {
        WaveOptions {
            op_shape: None,
            op_amplitude: None,
            op_frequency: None,
            op_pi_phase: None,
            op_sec_span: None,
            op_num_span: None,
        }
    }
}

fn set_option_val_format(command_option: &CommandOption, format_options: &mut FormatOptions, arg: &str) -> Result<()> {
    match command_option.id {
        OptionID::Format => {
            if format_options.op_format.is_none() {
                format_options.op_format = Some(DEFAULT_WAVE_FORMAT);
            }
            let num = usize::from_str_radix(arg, 10)?;
            match command_option.count {
                0 => {
                    // format id
                    if num == 1 || num == 3 {
                        format_options.op_format.as_mut().unwrap().id = num;
                    } else {
                        return Err(SampleWavError::new(
                            SampleWavErrorKind::IrregalArguments,
                            Some("format id must be 1 or 3".to_string()),
                        ));
                    }
                }
                1 => {
                    // channel
                    if num == 1 || num == 2 {
                        format_options.op_format.as_mut().unwrap().channel = num;
                    } else {
                        return Err(SampleWavError::new(
                            SampleWavErrorKind::IrregalArguments,
                            Some("number of channels must be 1 or 2".to_string()),
                        ));
                    }
                }
                2 => {
                    // sampling rate
                    if num == 8000
                        || num == 16000
                        || num == 22050
                        || num == 44100
                        || num == 32000
                        || num == 48000
                        || num == 96000
                        || num == 192000
                    {
                        format_options.op_format.as_mut().unwrap().sampling_rate = num;
                    } else {
                        return Err(SampleWavError::new(
                            SampleWavErrorKind::IrregalArguments,
                            Some(
                                "sampling rate must be one of 8000, 16000, 32000, 48000, 96000, 192000, 22050, or 44100"
                                    .to_string(),
                            ),
                        ));
                    }
                }
                3 => {
                    // bits per sample
                    if num == 8 || num == 16 || num == 24 || num == 32 {
                        if format_options.op_format.as_mut().unwrap().id == 3 && num != 32 {
                            return Err(SampleWavError::new(
                                SampleWavErrorKind::IrregalArguments,
                                Some("bits per sample is forced to 32, because foramt id is 3.".to_string()),
                            ));
                        }
                        format_options.op_format.as_mut().unwrap().bits = num;
                    } else {
                        return Err(SampleWavError::new(
                            SampleWavErrorKind::IrregalArguments,
                            Some("bits per suample must be one of 8, 16, 24, 32".to_string()),
                        ));
                    }
                }
                _ => {
                    return Err(SampleWavError::new(
                        SampleWavErrorKind::TooManyArguments,
                        Some(arg.to_string()),
                    ));
                }
            }
            return Ok(());
        }

        OptionID::SecLength => {
            let sec_len = arg.parse::<f64>()?;
            if format_options.op_num_len.is_some() || format_options.op_sec_len.is_some() {
                return Err(SampleWavError::new(
                    SampleWavErrorKind::DuplicatedArguments,
                    Some("length".to_string()),
                ));
            } else if sec_len >= 0.0 {
                format_options.op_sec_len = Some(sec_len);
            } else {
                return Err(SampleWavError::new(
                    SampleWavErrorKind::IrregalArguments,
                    Some("seconds must be more than 0.0".to_string()),
                ));
            }
            return Ok(());
        }
        OptionID::NumLength => {
            let num_len = usize::from_str_radix(arg, 10)?;
            if format_options.op_sec_len.is_some() || format_options.op_num_len.is_some() {
                return Err(SampleWavError::new(
                    SampleWavErrorKind::DuplicatedArguments,
                    Some("length".to_string()),
                ));
            } else {
                format_options.op_num_len = Some(num_len);
            }
            return Ok(());
        }
        _ => {
            panic!()
        }
    }
}

fn set_option_val_path(command_option: &CommandOption, path_options: &mut PathOptions, arg: &str) -> Result<()> {
    match command_option.id {
        OptionID::InputPath => {
            let input_path = PathBuf::from(arg);
            if path_options.op_input_path.is_some() {
                return Err(SampleWavError::new(
                    SampleWavErrorKind::DuplicatedArguments,
                    Some("input".to_string()),
                ));
            } else {
                path_options.op_input_path = Some(input_path);
            }
            return Ok(());
        }
        OptionID::OutputPath => {
            let output_path = PathBuf::from(arg);
            if path_options.op_output_path.is_some() || path_options.overwrite {
                return Err(SampleWavError::new(
                    SampleWavErrorKind::DuplicatedArguments,
                    Some("output".to_string()),
                ));
            } else {
                path_options.op_output_path = Some(output_path);
            }
            return Ok(());
        }
        _ => {
            panic!()
        }
    }
}

fn set_option_val_wave(command_option: &CommandOption, wave_options: &mut WaveOptions, arg: &str) -> Result<()> {
    match command_option.id {
        OptionID::Amplitude => {
            let amplitude = arg.parse::<f64>()?;
            if wave_options.op_amplitude.is_some() {
                return Err(SampleWavError::new(
                    SampleWavErrorKind::DuplicatedArguments,
                    Some("amplitude".to_string()),
                ));
            } else if -100.0 <= amplitude && amplitude <= 100.0 {
                wave_options.op_amplitude = Some(amplitude);
            } else {
                return Err(SampleWavError::new(
                    SampleWavErrorKind::IrregalArguments,
                    Some("amplitude must be >= -100.0 and <= 100.0".to_string()),
                ));
            }
            return Ok(());
        }
        OptionID::Frequency => {
            let frequency = arg.parse::<f64>()?;
            if wave_options.op_frequency.is_some() {
                return Err(SampleWavError::new(
                    SampleWavErrorKind::DuplicatedArguments,
                    Some("frequency".to_string()),
                ));
            } else if 0.0 <= frequency {
                wave_options.op_frequency = Some(frequency);
            } else {
                return Err(SampleWavError::new(
                    SampleWavErrorKind::IrregalArguments,
                    Some("frequency must be >= 0.0".to_string()),
                ));
            }
            return Ok(());
        }
        OptionID::SecSpan => {
            if wave_options.op_num_span.is_some() {
                return Err(SampleWavError::new(
                    SampleWavErrorKind::DuplicatedArguments,
                    Some("span".to_string()),
                ));
            }
            let sec = arg.parse::<f64>()?;
            if command_option.count == 0 {
                if wave_options.op_sec_span.is_some() {
                    return Err(SampleWavError::new(
                        SampleWavErrorKind::DuplicatedArguments,
                        Some("span".to_string()),
                    ));
                } else {
                    wave_options.op_sec_span = Some((sec, sec));
                }
            } else if command_option.count == 1 {
                let sec_span = wave_options.op_sec_span.as_mut().unwrap();
                *sec_span = (sec_span.0, sec);
            } else {
                panic!()
            }
            return Ok(());
        }
        OptionID::NumSpan => {
            if wave_options.op_sec_span.is_some() {
                return Err(SampleWavError::new(
                    SampleWavErrorKind::DuplicatedArguments,
                    Some("span".to_string()),
                ));
            }
            let num = usize::from_str_radix(arg, 10)?;
            if command_option.count == 0 {
                if wave_options.op_num_span.is_some() {
                    return Err(SampleWavError::new(
                        SampleWavErrorKind::DuplicatedArguments,
                        Some("span".to_string()),
                    ));
                } else {
                    wave_options.op_num_span = Some((num, num));
                }
            } else if command_option.count == 1 {
                let num_span = wave_options.op_num_span.as_mut().unwrap();
                *num_span = (num_span.0, num);
            } else {
                panic!()
            }
            return Ok(());
        }
        OptionID::PiPhase => {
            let pi_phase = arg.parse::<f64>()?;
            if wave_options.op_amplitude.is_some() {
                return Err(SampleWavError::new(
                    SampleWavErrorKind::DuplicatedArguments,
                    Some("phasepi".to_string()),
                ));
            } else if 0.0 <= pi_phase && pi_phase < 2.0 {
                wave_options.op_pi_phase = Some(pi_phase);
            } else {
                return Err(SampleWavError::new(
                    SampleWavErrorKind::IrregalArguments,
                    Some("phasepi must be  >= 0.0 and < 2.0".to_string()),
                ));
            }
            return Ok(());
        }
        _ => {
            panic!()
        }
    }
}

fn parse_options(arg_vec: &Vec<String>, command_kind: CommandKind) -> Result<(FormatOptions, WaveOptions, PathOptions)> {
    let mut format_options = FormatOptions::new();
    let mut wave_options = WaveOptions::new();
    let mut path_options = PathOptions::new();

    let mut op_command_option: Option<CommandOption> = None;
    for arg in arg_vec.iter().skip(1) {
        if arg.starts_with("--") {
            if let Some(command_option) = &op_command_option {
                if let Some(num) = command_option.op_num {
                    if num != command_option.count {
                        return Err(SampleWavError::new(
                            SampleWavErrorKind::IrregalArguments,
                            Some("option value has not yet been set".to_string()),
                        ));
                    }
                } else {
                    if command_option.count == 0 {
                        return Err(SampleWavError::new(
                            SampleWavErrorKind::IrregalArguments,
                            Some("option value has not yet been set".to_string()),
                        ));
                    }
                }
            }
            match arg.as_str() {
                "--format" => {
                    if command_kind == CommandKind::New {
                        op_command_option = Some(CommandOption::new(OptionID::Format));
                    } else {
                        return Err(SampleWavError::new(
                            SampleWavErrorKind::IrregalArguments,
                            Some(format!("\"{}\" does not exit in this command", arg)),
                        ));
                    }
                }
                "--seclen" => {
                    if command_kind == CommandKind::New {
                        op_command_option = Some(CommandOption::new(OptionID::SecLength));
                    } else {
                        return Err(SampleWavError::new(
                            SampleWavErrorKind::IrregalArguments,
                            Some(format!("\"{}\" does not exit in this command", arg)),
                        ));
                    }
                }
                "--numlen" => {
                    if command_kind == CommandKind::New {
                        op_command_option = Some(CommandOption::new(OptionID::NumLength));
                    } else {
                        return Err(SampleWavError::new(
                            SampleWavErrorKind::IrregalArguments,
                            Some(format!("\"{}\" does not exit in this command", arg)),
                        ));
                    }
                }
                "--in" => {
                    if command_kind == CommandKind::Add {
                        op_command_option = Some(CommandOption::new(OptionID::InputPath));
                    } else {
                        return Err(SampleWavError::new(
                            SampleWavErrorKind::IrregalArguments,
                            Some(format!("\"{}\" does not exit in this command", arg)),
                        ));
                    }
                }
                "--out" => {
                    op_command_option = Some(CommandOption::new(OptionID::OutputPath));
                }
                "--ow" => {
                    if command_kind == CommandKind::Add {
                        if path_options.op_output_path.is_some() || path_options.overwrite {
                            return Err(SampleWavError::new(
                                SampleWavErrorKind::DuplicatedArguments,
                                Some("output or over write".to_string()),
                            ));
                        } else {
                            op_command_option = Some(CommandOption::new(OptionID::OverWrite));
                            path_options.overwrite = true;
                        }
                    } else {
                        return Err(SampleWavError::new(
                            SampleWavErrorKind::IrregalArguments,
                            Some(format!("\"{}\" does not exit in this command", arg)),
                        ));
                    }
                }
                "--sin" => {
                    op_command_option = Some(CommandOption::new(OptionID::WaveShape));
                    if wave_options.op_shape.is_some() {
                        return Err(SampleWavError::new(
                            SampleWavErrorKind::DuplicatedArguments,
                            Some("wave shape".to_string()),
                        ));
                    } else {
                        wave_options.op_shape = Some(WaveShape::Sine);
                    }
                }
                "--tri" => {
                    op_command_option = Some(CommandOption::new(OptionID::WaveShape));
                    if wave_options.op_shape.is_some() {
                        return Err(SampleWavError::new(
                            SampleWavErrorKind::DuplicatedArguments,
                            Some("wave shape".to_string()),
                        ));
                    } else {
                        wave_options.op_shape = Some(WaveShape::Triangle);
                    }
                }
                "--square" => {
                    op_command_option = Some(CommandOption::new(OptionID::WaveShape));
                    if wave_options.op_shape.is_some() {
                        return Err(SampleWavError::new(
                            SampleWavErrorKind::DuplicatedArguments,
                            Some("wave shape".to_string()),
                        ));
                    } else {
                        wave_options.op_shape = Some(WaveShape::Square);
                    }
                }
                "--amp" => {
                    op_command_option = Some(CommandOption::new(OptionID::Amplitude));
                }
                "--freq" => {
                    op_command_option = Some(CommandOption::new(OptionID::Frequency));
                }
                "--secspan" => {
                    op_command_option = Some(CommandOption::new(OptionID::SecSpan));
                }
                "--numspan" => {
                    op_command_option = Some(CommandOption::new(OptionID::NumSpan));
                }
                "--piphase" => {
                    op_command_option = Some(CommandOption::new(OptionID::PiPhase));
                }
                _ => {
                    return Err(SampleWavError::new(SampleWavErrorKind::IrregalArguments, Some(arg.clone())));
                }
            }
        } else if let Some(command_option) = &mut op_command_option {
            match command_option.kind {
                OptionKind::FormatOption => {
                    set_option_val_format(command_option, &mut format_options, arg)?;
                }
                OptionKind::WaveOption => {
                    set_option_val_wave(command_option, &mut wave_options, arg)?;
                }
                OptionKind::PathOption => {
                    set_option_val_path(command_option, &mut path_options, arg)?;
                }
            }
            command_option.count += 1;
        }
    }
    if let Some(command_option) = &op_command_option {
        if let Some(num) = command_option.op_num {
            if num != command_option.count {
                return Err(SampleWavError::new(
                    SampleWavErrorKind::IrregalArguments,
                    Some("option value has not yet been set".to_string()),
                ));
            }
        } else {
            if command_option.count == 0 {
                return Err(SampleWavError::new(
                    SampleWavErrorKind::IrregalArguments,
                    Some("option value has not yet been set".to_string()),
                ));
            }
        }
    }
    Ok((format_options, wave_options, path_options))
}

fn make_wav_format_and_len(format_options: &FormatOptions) -> (WaveFormat, usize) {
    let wave_format = if let Some(some_wave_format) = &format_options.op_format {
        some_wave_format.clone()
    } else {
        DEFAULT_WAVE_FORMAT
    };
    let sampling_num: usize;
    if let Some(sec_len) = format_options.op_sec_len {
        sampling_num = (sec_len * wave_format.sampling_rate as f64) as usize;
    } else if let Some(num_len) = format_options.op_num_len {
        sampling_num = num_len;
    } else {
        sampling_num = 5 * wave_format.sampling_rate;
    }
    (wave_format, sampling_num)
}

fn make_wave(
    wave_options: &WaveOptions,
    wave_format: &WaveFormat,
    op_src_channel_data_vec: Option<Vec<Vec<f64>>>,
    src_len: usize,
) -> Result<Vec<Vec<f64>>> {
    let shape = if let Some(some_shape) = &wave_options.op_shape {
        (*some_shape).clone()
    } else {
        WaveShape::Sine
    };
    let amplitude = if let Some(some_amplitude) = wave_options.op_amplitude {
        some_amplitude
    } else {
        -3.0
    };
    let frequency = if let Some(some_frequency) = wave_options.op_frequency {
        some_frequency
    } else {
        440.0
    };
    let pi_phase = if let Some(some_pi_phase) = wave_options.op_pi_phase {
        some_pi_phase
    } else {
        0.0
    };
    let num_span = if let Some(some_sec_span) = wave_options.op_sec_span {
        (
            (some_sec_span.0 * wave_format.sampling_rate as f64) as usize,
            (some_sec_span.1 * wave_format.sampling_rate as f64) as usize,
        )
    } else if let Some(some_num_span) = wave_options.op_num_span {
        some_num_span
    } else {
        (0, src_len)
    };
    if num_span.0 > num_span.1 || num_span.1 > AUDIO_DATA_LENGTH_MAX {
        return Err(SampleWavError::new(
            SampleWavErrorKind::IrregalArguments,
            Some("span".to_string()),
        ));
    }
    let additional_channel_data_vec = match shape {
        WaveShape::Sine => create_sin_wave(&wave_format, amplitude, frequency, pi_phase, num_span.1 - num_span.0),
        WaveShape::Triangle => create_triangle_wave(&wave_format, amplitude, frequency, pi_phase, num_span.1 - num_span.0),
        WaveShape::Square => create_square_wave(&wave_format, amplitude, frequency, pi_phase, num_span.1 - num_span.0),
    };

    // add created wave to source wave.
    let mut channel_data_vec = if let Some(some_src_channel_data_vec) = op_src_channel_data_vec {
        some_src_channel_data_vec
    } else {
        vec![vec![0.0; src_len]; wave_format.channel]
    };

    if num_span.1 > src_len {
        for data_vec in &mut channel_data_vec {
            data_vec.append(&mut vec![0.0; num_span.1 - src_len]);
        }
    }

    for channel_idx in 0..wave_format.channel {
        for data_idx in num_span.0..num_span.1 {
            channel_data_vec[channel_idx][data_idx] += additional_channel_data_vec[channel_idx][data_idx - num_span.0]
        }
    }
    Ok(channel_data_vec)
}

fn make_wav_file(
    op_src_wav_file: Option<WavFile>,
    path_options: PathOptions,
    wave_format: WaveFormat,
    channel_data_vec: Vec<Vec<f64>>,
) -> Result<()> {
    let mut wav_file = if let Some(src_wav_file) = op_src_wav_file {
        src_wav_file
    } else {
        WavFile::new()
    };
    let output_path = if let Some(some_output_path) = path_options.op_output_path {
        some_output_path
    } else if path_options.overwrite {
        if let Some(some_input_path) = path_options.op_input_path {
            some_input_path
        } else {
            return Err(SampleWavError::new(
                SampleWavErrorKind::IrregalArguments,
                Some("Input path is must be set".to_string()),
            ));
        }
    } else {
        PathBuf::from("./new.wav")
    };
    if output_path.exists() && !path_options.overwrite {
        let stdin = std::io::stdin();
        let mut answer = String::new();
        println!(
            "{} is already exist. Do you want to overwrite it?(y/n)",
            output_path.to_string_lossy()
        );
        stdin.read_line(&mut answer)?;
        let mut iter = answer.trim().split_ascii_whitespace();
        if iter.next() != Some("y") && iter.next() != Some("Y") {
            return Ok(());
        }
    }
    wav_file.update_audio_for_channel_data_vec(&wave_format, &channel_data_vec)?;
    wav_file.save_as(&output_path)?;
    println!("Saved : {}", output_path.to_string_lossy());
    Ok(())
}

fn open_wav_file(path_options: &PathOptions) -> Result<WavFile> {
    if let Some(input_path) = &path_options.op_input_path {
        WavFile::open(input_path)
    } else {
        return Err(SampleWavError::new(
            SampleWavErrorKind::IrregalArguments,
            Some("Input path is must be set".to_string()),
        ));
    }
}

fn command_new(arg_vec: &Vec<String>) -> Result<()> {
    let (format_options, wave_options, path_options) = parse_options(arg_vec, CommandKind::New)?;
    let (wave_format, sampling_num) = make_wav_format_and_len(&format_options);
    let channel_data_vec = make_wave(&wave_options, &wave_format, None, sampling_num)?;
    make_wav_file(None, path_options, wave_format, channel_data_vec)?;
    Ok(())
}

fn command_addwave(arg_vec: &Vec<String>) -> Result<()> {
    let (_, wave_options, path_options) = parse_options(arg_vec, CommandKind::Add)?;
    let wav_file = open_wav_file(&path_options)?;
    let (wave_format, src_channel_data_vec) = wav_file.get_audio_for_channel_data_vec()?;
    let src_len = src_channel_data_vec[0].len();
    let channel_data_vec = make_wave(&wave_options, &wave_format, Some(src_channel_data_vec), src_len)?;
    make_wav_file(Some(wav_file), path_options, wave_format, channel_data_vec)?;
    Ok(())
}

fn main_return_result() -> Result<()> {
    let mut arg_vec: Vec<String> = Vec::new();
    for (idx, arg) in std::env::args().skip(1).enumerate() {
        if idx > ARG_NUM_MAX {
            return Err(SampleWavError::new(SampleWavErrorKind::TooManyArguments, None));
        }
        dbg!(idx, arg.clone());
        arg_vec.push(arg);
    }
    if arg_vec.len() > 0 {
        match arg_vec[0].as_str() {
            "new" => {
                command_new(&arg_vec)?;
            }
            "addwave" => {
                command_addwave(&arg_vec)?;
            }
            _ => {
                return Err(SampleWavError::new(
                    SampleWavErrorKind::IrregalArguments,
                    Some("1st arguments is \"new\" or \"addwave\".".to_string()),
                ));
            }
        }
    } else {
        return Err(SampleWavError::new(SampleWavErrorKind::TooFewArguments, None));
    }
    Ok(())
}

fn main() {
    match main_return_result() {
        Ok(_) => {}
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
