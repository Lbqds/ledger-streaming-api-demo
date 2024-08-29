use ledger_device_sdk::io;
use ledger_device_sdk::nbgl::*;
use crate::error_code::ErrorCode;
use include_gif::include_gif;

pub static APP_ICON: NbglGlyph = NbglGlyph::from_include(include_gif!("alph_64x64.gif", NBGL));
// pub const APP_ICON: NbglGlyph = NbglGlyph::from_include(include_gif!("alph_64x64.gif", NBGL));

#[repr(u8)]
pub enum Ins {
    StreamingApiTest,
}

impl TryFrom<io::ApduHeader> for Ins {
    type Error = ErrorCode;
    fn try_from(header: io::ApduHeader) -> Result<Self, Self::Error> {
        match header.ins {
            0 => Ok(Ins::StreamingApiTest),
            _ => Err(ErrorCode::BadIns),
        }
    }
}

pub struct Handler {
    started: bool,
    reviewer: NbglStreamingReview,
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            started: false,
            reviewer: NbglStreamingReview::new().glyph(&APP_ICON),
        }
    }

    pub fn reset(&mut self) {
        self.started = false;
    }

    fn display(&mut self, data_size: usize) -> Result<(), ErrorCode> {
        if !self.started {
            self.started = true;

            if !self.reviewer.start("Start Review", "") {
                NbglReviewStatus::new().show(false);
                self.reset();
                return Err(ErrorCode::UserCancelled);
            }
        }

        if data_size == 255 { // max payload size
            let fields = &[Field {
                name: "Continue Review",
                value: "Continue Review"
            }];
            if self.reviewer.continue_review(fields) {
                return Ok(());
            } else {
                NbglReviewStatus::new().show(false);
                self.reset();
                return Err(ErrorCode::UserCancelled);
            }
        }

        if self.reviewer.finish("Finish Review") {
            NbglReviewStatus::new().show(true);
            self.reset();
            Ok(())
        } else {
            NbglReviewStatus::new().show(false);
            self.reset();
            Err(ErrorCode::UserCancelled)
        }
    }

    pub fn handle_apdu(&mut self, comm: &mut io::Comm, ins: Ins) -> Result<(), io::Reply> {
        if comm.rx == 0 {
            return Err(ErrorCode::BadLen.into());
        }
    
        // Common instructions
        match ins {
            Ins::StreamingApiTest => {
                let data = comm.get_data()?;
                match self.display(data.len()) {
                    Ok(()) => Ok(()),
                    Err(code) => Err(code.into()),
                }
            }
        }
    }
}
