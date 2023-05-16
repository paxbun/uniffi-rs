/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[derive(Debug, thiserror::Error)]
pub enum TelephoneError {
    #[error("Busy")]
    Busy,
    #[error("InternalTelephoneError")]
    InternalTelephoneError,
}

// Need to implement this From<> impl in order to handle unexpected callback errors.  See the
// Callback Interfaces section of the handbook for more info.
impl From<uniffi::UnexpectedUniFFICallbackError> for TelephoneError {
    fn from(_: uniffi::UnexpectedUniFFICallbackError) -> Self {
        Self::InternalTelephoneError
    }
}

pub trait CallAnswerer {
    fn answer(&self) -> Result<String, TelephoneError>;
}

#[derive(Debug, Default, Clone)]
pub struct Telephone;
impl Telephone {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn call(&self, answerer: Box<dyn CallAnswerer>) -> Result<String, TelephoneError> {
        answerer.answer()
    }
}


trait ProgressReporter: Send + Sync {
    fn report_progress(&self, progress: f32);
}

fn run_huge_task(reporter: Box<dyn ProgressReporter>) {
    fn report_progress(reporter: Box<dyn ProgressReporter>, current_progress: i32) {
        reporter.report_progress(current_progress as f32 / 1000.0);
        if current_progress < 1000 {
            std::thread::spawn(move || {
                report_progress(reporter, current_progress + 1);
            });
        }
    }
    std::thread::spawn(move || {
        report_progress(reporter, 0);
    });
}

include!(concat!(env!("OUT_DIR"), "/callbacks.uniffi.rs"));
