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

// todo:esavier configurability?
use super::consts::NETFRAME_DELIMITER;
use crate::netframe::{
    error::NetFrameError,
    types::{NetFrame, NetFrameMetadata},
};
impl NetFrame {
    pub fn new(
        tag: u8,
        data: Vec<u8>,
    ) -> Self {
        Self {
            tag,
            data,
        }
    }

    pub fn get_metadata(buffer: &Vec<u8>) -> Result<NetFrameMetadata, NetFrameError> {
        if buffer.len() < 4 {
            return Err(NetFrameError::TooLittleData);
        };
        if buffer[0] != NETFRAME_DELIMITER {
            return Err(NetFrameError::DelimiterMismatch);
        };
        Ok(NetFrameMetadata {
            tag: buffer[1],
            size: ((buffer[2] as u16) << 8) | (buffer[3] as u16),
        })
    }
}
