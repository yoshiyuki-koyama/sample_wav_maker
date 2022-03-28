use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[derive(Debug, Clone, PartialEq)]
pub struct SampleWavError {
    pub err_kind: SampleWavErrorKind,
    pub op_additional_message: Option<String>,
}

impl SampleWavError {
    pub fn new(
        err_kind: SampleWavErrorKind,
        op_additional_message: Option<String>,
    ) -> Box<dyn std::error::Error + Send + Sync + 'static> {
        Box::<SampleWavError>::new(SampleWavError {
            err_kind: err_kind,
            op_additional_message: op_additional_message,
        })
    }
}

impl fmt::Display for SampleWavError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        for err_message in WAVE_AUDIO_ERR_MESSAGE {
            if err_message.err_kind == self.err_kind {
                if let Some(additional_message) = &self.op_additional_message {
                    return write!(f, "{}", format!("{} : {}", err_message.message, additional_message));
                } else {
                    return write!(f, "{}", format!("{}", err_message.message));
                }
            }
        }
        panic!()
    }
}

impl std::error::Error for SampleWavError {}

#[derive(Debug, Clone, PartialEq)]
pub enum SampleWavErrorKind {
    TooManyArguments,
    TooFewArguments,
    IrregalArguments,
    DuplicatedArguments,
    PathIsNotFile,
}

struct SampleWavErrorMessage {
    err_kind: SampleWavErrorKind,
    message: &'static str,
}

const WAVE_AUDIO_ERR_MESSAGE: [SampleWavErrorMessage; 5] = [
    SampleWavErrorMessage {
        err_kind: SampleWavErrorKind::TooManyArguments,
        message: "Too many arguments.",
    },
    SampleWavErrorMessage {
        err_kind: SampleWavErrorKind::TooFewArguments,
        message: "Too few arguments.",
    },
    SampleWavErrorMessage {
        err_kind: SampleWavErrorKind::IrregalArguments,
        message: "Irregal arguments.",
    },
    SampleWavErrorMessage {
        err_kind: SampleWavErrorKind::DuplicatedArguments,
        message: "duplicated arguments.",
    },
    SampleWavErrorMessage {
        err_kind: SampleWavErrorKind::PathIsNotFile,
        message: "Specified path is not file.",
    },
];
