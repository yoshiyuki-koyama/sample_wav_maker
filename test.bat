@ if "%1"=="debug" (
    echo "cargo build"
    cargo build
    set exe_path=.\target\debug\
) else (
    echo "cargo build --release"
    cargo build --release
    set exe_path=.\target\release\
)

rem remove & recreate "test_output" directory
set test_dir=.\test_output\
rmdir /s /q %test_dir%
mkdir %test_dir%

rem default
%exe_path%sample_wav_maker.exe new
%exe_path%sample_wav_maker.exe addwave --in new.wav
rem error
%exe_path%sample_wav_maker.exe
%exe_path%sample_wav_maker.exe addwave
rem output path
%exe_path%sample_wav_maker.exe new --out %test_dir%new.wav
%exe_path%sample_wav_maker.exe addwave --in %test_dir%new.wav --out %test_dir%addwave.wav
rem forced output
%exe_path%sample_wav_maker.exe new --out %test_dir%forcedout.wav
%exe_path%sample_wav_maker.exe new --framenum 16000 --outf %test_dir%forcedout.wav
%exe_path%sample_wav_maker.exe addwave --in %test_dir%new.wav --outf %test_dir%forcedout.wav
rem overwrite
%exe_path%sample_wav_maker.exe new --framenum 16000 --out %test_dir%overwrite.wav
%exe_path%sample_wav_maker.exe addwave --in %test_dir%overwrite.wav --ow
rem format
%exe_path%sample_wav_maker.exe new --out %test_dir%format.wav --format 3 2 96000 32 --sin --seclen 1.0
rem shape
%exe_path%sample_wav_maker.exe new --out %test_dir%sin.wav --sin --seclen 1.0
%exe_path%sample_wav_maker.exe new --out %test_dir%tri.wav --tri --seclen 1.0
%exe_path%sample_wav_maker.exe new --out %test_dir%square.wav --square --seclen 1.0
%exe_path%sample_wav_maker.exe addwave --in %test_dir%square.wav --out %test_dir%square_sin.wav --sin
%exe_path%sample_wav_maker.exe addwave --in %test_dir%sin.wav --out %test_dir%sin_tri.wav --tri
%exe_path%sample_wav_maker.exe addwave --in %test_dir%tri.wav --out %test_dir%tri_square.wav --square
rem amp
%exe_path%sample_wav_maker.exe new --out %test_dir%amp.wav --format 1 2 48000 16 --seclen 1.0 --amp -1
%exe_path%sample_wav_maker.exe addwave --in %test_dir%amp.wav --out %test_dir%amp_add.wav --square --amp -12
rem frequency & seclen & secspan & phase
%exe_path%sample_wav_maker.exe new --out %test_dir%seq.wav --format 1 2 48000 16 --seclen 1.2 --secspan 0.01 1.0
%exe_path%sample_wav_maker.exe addwave --in %test_dir%seq.wav --out %test_dir%seq_add.wav --square --amp -12 --secspan 0.005 1.0 --piphase 1.5
rem frequency & framenum & framespan & phase
%exe_path%sample_wav_maker.exe new --out %test_dir%frame.wav --format 1 2 48000 16 --framenum 60000 --framespan 48 48000
%exe_path%sample_wav_maker.exe addwave --in %test_dir%frame.wav --out %test_dir%frame_add.wav --square --amp -12 --framespan 24 48000 --piphase 1.5
%exe_path%sample_wav_maker.exe addwave --in %test_dir%frame.wav --out %test_dir%frame_add2.wav --tri --amp -6 --framespan 48000 96000 --piphase 1.5