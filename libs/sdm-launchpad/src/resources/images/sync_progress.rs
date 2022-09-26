//  Copyright 2022, The Tari Project
//
//  Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
//  following conditions are met:
//
//  1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
//  disclaimer.
//
//  2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
//  following disclaimer in the documentation and/or other materials provided with the distribution.
//
//  3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
//  products derived from this software without specific prior written permission.
//
//  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
//  INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
//  SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//  SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
//  WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
//  USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::time::{Duration, Instant};

use serde::Serialize;
use tari_app_grpc::tari_rpc::{SyncProgressResponse, SyncState};

pub const BLOCKS_SYNC_EXPECTED_TIME: Duration = Duration::from_secs(4 * 3600);
pub const HEADERS_SYNC_EXPECTED_TIME: Duration = Duration::from_secs(2 * 3600);

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum SyncType {
    Startup,
    Block,
    Header,
    Done,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SyncProgressInfo {
    pub sync_type: SyncType,
    pub header_progress: u64,
    pub block_progress: u64,
    pub total_blocks: u64,
    pub estimated_time_sec: u64,
    pub done: bool,
    pub sync_time_sec: u64,
}

pub struct SyncProgress {
    sync_type: SyncType,
    header_sync: ItemCount,
    blocks_sync: ItemCount,
}

struct ItemCount {
    total_items: u64,
    start_item: u64,
    current: u64,
    started: Instant,
    initial_estimate: Duration,
    completed: Option<Duration>,
}

impl ItemCount {
    pub fn new(start_item: u64, total_items: u64, initial_estimate: Duration) -> Self {
        Self {
            total_items,
            start_item,
            current: 0,
            started: Instant::now(),
            initial_estimate,
            completed: None,
        }
    }

    pub fn update(&mut self, new_current: u64) {
        self.current = new_current;
        if self.is_done() && self.completed.is_none() {
            self.completed = Some(self.started.elapsed());
        }
    }

    pub fn is_done(&self) -> bool {
        self.current >= self.total_items
    }

    pub fn set_done(&mut self) {
        self.update(self.total_items);
    }

    pub fn estimate_remaining_time(&self) -> Duration {
        if self.is_done() {
            return Duration::from_secs(0);
        }
        let frac_complete = self.progress();
        // Use the first 1% to calibrate
        if frac_complete < 0.01 {
            self.initial_estimate
        } else {
            let elapsed = self.started.elapsed();
            Duration::from_secs_f64(elapsed.as_secs_f64() / frac_complete)
        }
    }

    pub fn progress(&self) -> f64 {
        let total_items_to_sync = self.total_items.saturating_sub(self.start_item).max(1);
        self.current.saturating_sub(self.start_item) as f64 / total_items_to_sync as f64
    }

    pub fn total_sync_time(&self) -> Duration {
        self.completed.unwrap_or(Duration::from_secs(0))
    }
}

impl SyncProgress {
    pub fn new(starting_block: u64, total_count: u64) -> Self {
        Self {
            sync_type: SyncType::Startup,
            header_sync: ItemCount::new(starting_block, total_count, HEADERS_SYNC_EXPECTED_TIME),
            blocks_sync: ItemCount::new(starting_block, total_count, BLOCKS_SYNC_EXPECTED_TIME),
        }
    }

    fn reset(item: &mut ItemCount, current: u64, total: u64) {
        item.start_item = current;
        item.current = current;
        item.total_items = total;
        item.started = Instant::now();
    }

    pub fn update(&mut self, progress: SyncProgressResponse) {
        // Update state machine based on local sync type, reported sync type combo
        match (&self.sync_type, progress.state()) {
            (SyncType::Startup, SyncState::Header) => {
                Self::reset(&mut self.header_sync, progress.local_height, progress.tip_height);
                Self::reset(&mut self.blocks_sync, 0, progress.tip_height);
                self.sync_type = SyncType::Header;
            },
            (SyncType::Startup, SyncState::Block) => {
                Self::reset(&mut self.blocks_sync, progress.local_height, progress.tip_height);
                self.header_sync.set_done();
                self.sync_type = SyncType::Block;
            },
            (SyncType::Startup, SyncState::Done) => {
                self.header_sync.set_done();
                self.blocks_sync.set_done();
                self.sync_type = SyncType::Done;
            },
            (SyncType::Header, SyncState::Header) => {
                self.header_sync.update(progress.local_height);
            },
            (SyncType::Header, SyncState::Block) => {
                self.header_sync.set_done();
                self.sync_type = SyncType::Block;
                let last_block = self.blocks_sync.current;
                Self::reset(&mut self.blocks_sync, last_block, progress.tip_height);
                self.blocks_sync.update(progress.local_height)
            },
            (SyncType::Block, SyncState::Block) => {
                self.blocks_sync.update(progress.local_height);
            },
            // Oh no, we've gone back to header syncs
            (SyncType::Block | SyncType::Done, SyncState::Header) => {
                self.sync_type = SyncType::Header;
                self.header_sync.total_items = progress.tip_height;
                self.blocks_sync.total_items = progress.tip_height;
                self.header_sync.update(progress.local_height);
                // Leave block sync where it was
            },
            // Oh no, we've gone back to block syncs
            (SyncType::Done, SyncState::Block) => {
                self.sync_type = SyncType::Block;
                self.blocks_sync.total_items = progress.tip_height;
                self.blocks_sync.update(progress.local_height);
            },
            (_, SyncState::Done) => {
                if !self.header_sync.is_done() {
                    self.header_sync.set_done();
                }
                if !self.blocks_sync.is_done() {
                    self.blocks_sync.set_done();
                }
                self.sync_type = SyncType::Done;
            },
            _ => {
                // no-op
            },
        }
    }

    pub fn is_done(&self) -> bool {
        self.header_sync.is_done() && self.blocks_sync.is_done()
    }

    pub fn estimated_time_remaining(&self) -> Duration {
        self.blocks_sync.estimate_remaining_time() + self.header_sync.estimate_remaining_time()
    }

    pub fn total_sync_time(&self) -> Duration {
        self.header_sync.total_sync_time() + self.blocks_sync.total_sync_time()
    }

    pub fn progress_info(&self) -> SyncProgressInfo {
        SyncProgressInfo {
            sync_type: self.sync_type.clone(),
            header_progress: (self.header_sync.progress() * 100.0) as u64,
            block_progress: (self.blocks_sync.progress() * 100.0) as u64,
            total_blocks: self.blocks_sync.total_items,
            estimated_time_sec: self.estimated_time_remaining().as_secs(),
            done: self.is_done(),
            sync_time_sec: self.total_sync_time().as_secs(),
        }
    }
}
