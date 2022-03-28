use wav_f64vec::*;

pub fn create_sin_wave(wave_format: &WaveFormat, amplitude:f64, frequency:f64, pi_phase:f64, len:usize) -> Vec<Vec<f64>> {
    dbg!(wave_format);
    dbg!(amplitude, frequency, pi_phase, len);
    let amp_coef = 10.0_f64.powf(amplitude / 10.0);

    let mut channel_data_vec: Vec<Vec<f64>> = Vec::new();
    for _ in 0..wave_format.channel {
        let mut data_vec: Vec<f64> = Vec::new();
        for idx in 0..len {
            let level = (2.0 * std::f64::consts::PI * frequency * (idx as f64) / wave_format.sampling_rate as f64 + pi_phase * std::f64::consts::PI).sin() * amp_coef;
            data_vec.push(level);
        }
        channel_data_vec.push(data_vec);
    }
    channel_data_vec
}

pub fn create_triangle_wave(wave_format: &WaveFormat, amplitude:f64, frequency:f64, pi_phase:f64, len:usize) -> Vec<Vec<f64>> {
    dbg!(wave_format);
    dbg!(amplitude, frequency, pi_phase, len);
    let amp_coef = 10.0_f64.powf(amplitude / 10.0);

    let mut channel_data_vec: Vec<Vec<f64>> = Vec::new();
    for _ in 0..wave_format.channel {
        let mut data_vec: Vec<f64> = Vec::new();
        for idx in 0..len {
            let mut pi_radian = 2.0 * frequency * (idx as f64) / wave_format.sampling_rate as f64 + pi_phase;
            pi_radian = pi_radian - (pi_radian / 2.0).floor() * 2.0;
            let level = if pi_radian < 0.5 {
                pi_radian * 2.0 * amp_coef
            } else if pi_radian < 1.5 {
                (2.0 - pi_radian * 2.0) * amp_coef
            } else {
                (-4.0 + pi_radian * 2.0) * amp_coef
            };
            data_vec.push(level);
        }
        channel_data_vec.push(data_vec);
    }
    channel_data_vec
}

pub fn create_square_wave(wave_format: &WaveFormat, amplitude:f64, frequency:f64, pi_phase:f64, len:usize) -> Vec<Vec<f64>> {
    dbg!(wave_format);
    dbg!(amplitude, frequency, pi_phase, len);
    let amp_coef = 10.0_f64.powf(amplitude / 10.0);

    let mut channel_data_vec: Vec<Vec<f64>> = Vec::new();
    for _ in 0..wave_format.channel {
        let mut data_vec: Vec<f64> = Vec::new();
        for idx in 0..len {
            let level = if ((2.0 * frequency * (idx as f64) / wave_format.sampling_rate as f64 + pi_phase).floor() as usize % 2) == 0 {
                amp_coef
            }
            else {
                - amp_coef
            };
            data_vec.push(level);
        }
        channel_data_vec.push(data_vec);
    }
    channel_data_vec
}


