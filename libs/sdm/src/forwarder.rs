// Copyright 2022. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

use std::{fmt, pin::Pin};

use anyhow::Error;
use futures::{Stream, StreamExt};
use tokio::sync::mpsc;

use crate::utils::TaskGuard;

pub trait Converter<I, O>: Sync + Send + 'static {
    fn convert(&self, res: Result<I, Error>) -> Option<O>;
}

pub struct Forwarder<I, O> {
    stream: Pin<Box<dyn Stream<Item = Result<I, Error>> + Send>>,
    converter: Box<dyn Converter<I, O>>,
    sender: mpsc::UnboundedSender<O>,
}

impl<I, O> Forwarder<I, O>
where
    I: Send + 'static,
    I: fmt::Debug,
    O: Send + 'static,
    O: fmt::Debug,
{
    pub fn start<S, C>(stream: S, converter: C, sender: mpsc::UnboundedSender<O>) -> TaskGuard<()>
    where
        S: Stream<Item = Result<I, Error>>,
        S: Send + 'static,
        C: Converter<I, O>,
    {
        let this = Self {
            sender,
            converter: Box::new(converter),
            stream: stream.boxed(),
        };
        tokio::spawn(this.entrypoint()).into()
    }

    async fn entrypoint(mut self) {
        while let Some(event) = self.stream.next().await {
            log::trace!("Event in forwarder: {:?}", event);
            if let Some(sdm_event) = self.converter.convert(event) {
                log::debug!("Sending event: {:?}", sdm_event);
                if self.sender.send(sdm_event).is_err() {
                    break;
                }
            }
        }
    }
}
