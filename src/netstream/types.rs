/*
Copyright (C) 2023 ErgLabs <dev@erglabs.org>.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::collections::VecDeque;

use crate::{netframe::types::NetFrame, netstream::error::NetStreamErr};


pub type FramingStreamResult = Result<(), NetStreamErr>;

pub trait FramingStream: Send + Sync {
    // returns next frame.
    // If no frame is available returns
    fn next(&mut self) -> Result<NetFrame, NetStreamErr>;
    // writes bulk data to stream, framing is happening on each write
    fn write(
        &mut self,
        data: Vec<u8>,
    ) -> Result<(), NetStreamErr>;
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum NetStreamState {
    #[default]
    Empty,
    InProgress,
    Failure,
}

// todo:esavier visibility?
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NetStream {
    pub frames: VecDeque<NetFrame>,
    pub buffer: Vec<u8>,
    pub state: NetStreamState,
}
