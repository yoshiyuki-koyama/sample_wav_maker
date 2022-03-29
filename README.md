# Under Development

# sample_wav_maker
Making sample wav files
(Only for 64bit OS.)

## Caution
\* Misconfigurarions or bugs may generate unexpected waveforms. So be careful with the volume when playing.

\* 設定ミスやバグによって予想外の波形が生成されることがあります。再生時は音量に気をつけましょう。

# sample_wav_maker
Making sample wav files

## Default Setting
If you do not set options, the following values are set.

### Default of Wav File
* Wave Format
    * Format id: 1 (PCM)
    * Channel: 1 (Monoral)
    * Sampling rate: 16000Hz
    * Bits per sample: 16

* Wave Length
    * 1.0 seconds

* Path
    * Output File: ."/new_{x}.wav"

### Default of Wave Shape
* Wave Shape Kind: Sine wave
* Frequency: 440.0 (Hz)
* Phase: 0.0 * PI (rad)
* Amplitude: -3.0 (dB)
* Span: equal to "Default of Wave Length"



## Usage of Command Line Arguments
### Make new file
```
sample_wav_maker.exe new [--out "file"] [--format <n(format id)> <n(channel)> <n(sampling rate)> <n(bits per sample)>] [--seclen <f> | --numlen <n>] [--sin | --tri | --square] [--amp <f> ] [--freq <f>] [--secspan <f> <f> | --numspan <n>  <n>] [--piphase <f>]
```

### Add wave to a existing file
```
sample_wav_maker.exe addwave --in "file" [--out "file"| --ow] [--sin | --tri | --square] [--amp <f> ] [--freq <f>] [--secspan <f> <f> | --numspan <n>  <n>] [--piphase <f>]
```

### Arguments about Format and File Length
You can only set the following options to "new".
#### `--format <n> <n> <n> <n>`
* 1st argument: format id
    * n = 1 (PCM) or 3 (IEEE float)
    * If you select 3, bits per sample is forced to 32.
* 2nd argument: channel
    * n = 1 (Monoral) or 2 (Stereo)
* 3rd argument: sampling rate (Hz)
    * n = 8000, 16000, 32000, 48000, 96000, 192000, 22050 or 44100
* 4th argument: bits per sample
    * n = 8, 16, 24 or 32
    * If you select 3 for format id (IEEE float), bits per sample must be set to 32.
    * If you select 1 for format id (PCM), audio data is as follows:
        * 8 : Unsigned 8bit PCM
        * 16, 24, 32 : Signed 16,24,32bit PCM

#### `--seclen <f>`
* change file length (seconds)
* f range : 0.0 /<= f && f \<= ((0xffffffff - 0x2E) / (bits per sample * channel * sampling rate)).floor()
* You can only set either "--seclen" or "--numlen".

#### `--numlen <n>`
* change file length (number of sampled data).
* n range : 0 /<= n && n \<= (0xffffffff - 0x2E) / (bits per sample * channel)
* You can only set either "--seclen" or "--numlen".

### Arguments about File Path
#### `--in "file"`
* load from "file" as source waves.
* You have to set this option when "addwave". And you can not set this to "new".

#### `--out "file"`
* save to "file"
* You can only set either "--out" or "--ow".

#### `--ow`
* overwrite to "--in" file.
* You can only set either "--out" or "--ow".
* You can only set this option to "addwave".


### Arguments about Wave Shape
#### `--sin`
* change wave shape to sine wave.

#### `--tri`
* change wave shape to triangle wave.

#### `--square`
* change wave shape to square wave.

#### `--amp <f>`
* change amplitude (dB)
* f range : -100.0 /<= f && f \<= 100.0

#### `--freq <f>`
* change frequency (Hz)
* f range : 0.0 /<= f

#### `--secspan <f> <f>`
* change wave span (seconds)
* first arguments is start seconds, and second one is end seconds
* f range : 0.0 /<= f && f \<= ((0xffffffff - 0x2E) / (bits per sample * channel * sampling rate)).floor()

#### `--numlen <n> <n>`
* change wave span (index of sampled data).
* first arguments is start data's index, and second one is end data's index
* n range : 0 /<= n && n \<= (0xffffffff - 0x2E) / (bits per sample * channel)

#### `--piphase <f>`
* change phase when span starts (PI * rad)
* f range : 0.0 /<= f && f \< 2.0





