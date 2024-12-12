// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use grammers_mtproto::{mtp, transport};
use grammers_mtsender::Sender;
use grammers_tl_types as tl;

pub struct Stepper {
    pub sender: Sender<transport::Full, mtp::Encrypted>,
    pub updates_tx: tokio::sync::mpsc::Sender<Vec<tl::enums::Updates>>,
}

pub enum StepperError {
    UpdatesHandlerGone,
    StepFailed(grammers_mtsender::ReadError),
}

impl Stepper {
    pub async fn run(self) -> Result<core::convert::Infallible, StepperError> {
        let Self {
            mut sender,
            updates_tx,
        } = self;

        let mut step = Box::pin(sender.step());
        loop {
            tokio::select! {
                // Read the updates while not dropping the step future.
                result = &mut step => {
                    let updates = result.map_err(StepperError::StepFailed)?;
                    updates_tx.send(updates).await.map_err(|_| StepperError::UpdatesHandlerGone)?;
                    // std::mem::
                    // step = Box::pin(sender.step());
                }
            }
        }
    }
}
