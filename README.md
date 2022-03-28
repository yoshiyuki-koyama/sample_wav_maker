# Under Development

# sample_wav_maker
Making sample wav files

## Caution
\* Misconfigurarions or bugs may generate unexpected waveforms. So be careful with the volume when playing.

\* 設定ミスやバグによって予想外の波形が生成されることがあります。再生時は音量に気をつけましょう。

# sample_wav_maker
Making sample wav files

## Default Setting
If you do not set options, the following values are set.

### Default of Wave Format
* Format id: 1 (PCM)
* Channel: 1 (Monoral)
* Sampling rate: 16000Hz
* Bits per sample: 16

### Default of Wave Length
* 1.0 seconds

### Default of Wave Shape
* Wave Shape Kind: Sine wave
* Frequency: 440.0 (Hz)
* Phase: 0.0 * PI (rad)
* Amplitude: -3.0 (dB)
* Span: equal to "Default of Wave Length"

### Path
* Output File: ."/new_{x}.wav"

## Usage of Command Line Arguments
sample_wav_maker new [--id \<n\>] [--channel \<n\>] [--sampling \<n\>] [--bits \<n\>] [--seclen \<f\> | --numlen \<n\>] [--sin | --tri | --square] [--amp \<f\> ] [--freq \<f\>] [--secspan \<f\> \<f\> | --numspan \<n\>  \<n\>] [--piphase \<f\>] [--output "file"]

sample_wav_maker addwave --input "file" [--sin | --tri | --square] [--amp \<f\> ] [--freq \<f\>] [--secspan \<f\> \<f\> | --numspan \<n\>  \<n\>] [--piphase \<f\>] [--output "file"]

### Wave Format
#### --id \<n\>
* change format id.
* n = 1 (PCM) or 3 (IEEE float)
* If you select 3, bits per sample is forced to 32.

#### --channel \<n\>
* change number of channels.
* default: 1
* n = 1 (Monoral) or 2 (Stereo)

#### --sampling \<n\>
* change sampling rate(Hz).
* n = 8000, 16000, 32000, 48000, 96000, 192000, 22050 or 44100

#### --bits \<n\>
* change bits per sample.
* n = 8, 16, 24 or 32
* If you select 3 for format id (IEEE float), bits per sample is forced to 32.
* If you select 1 for format id (PCM), audio data is as follows:
    * 8 : Unsigned 8bit PCM
    * 16, 24, 32 : Signed 16,24,32bit PCM

### Wave Length
### --seclen \<f\>
* change file length (seconds)
* f range : 0.0 /<= f && f \<= ((0xffffffff - 0x2E) / (bits per sample * channel * sampling rate)).floor()

#### --numlen \<n\>
* change file length (number of sampled data).
* n range : 0 /<= n && n \<= (0xffffffff - 0x2E) / (bits per sample * channel)

### Wave Shape
#### --sin
* change wave shape to sine wave.

### --tri
* change wave shape to triangle wave.

### --square
* change wave shape to square wave.

### --amp \<f\>
* change amplitude (dB)
* f range : -100.0 /<= f && f \<= 100.0

### --freq <\f\>
* change frequency (Hz)
* f range : 0.0 /<= f

### --secspan \<f\> \<f\>
* change wave span (seconds)
* first arguments is start seconds, and second one is end seconds
* f range : 0.0 /<= f && f \<= ((0xffffffff - 0x2E) / (bits per sample * channel * sampling rate)).floor()

#### --numlen \<n\> \<n\>
* change wave span (index of sampled data).
* first arguments is start data's index, and second one is end data's index
* n range : 0 /<= n && n \<= (0xffffffff - 0x2E) / (bits per sample * channel)

### --piphase <\f\>
* change phase when span starts (PI * rad)
* f range : 0.0 /<= f && f \< 2.0

### File Path
### --output "file"
* save to "file"

### --input "file"
* load from "file" as source waves.


