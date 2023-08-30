use crate::netframe::error::NetFrameError;


#[derive(Debug, PartialEq, Clone, Default)]
pub enum NetStreamErrorType {
    Generic,

    #[default]
    Unknown,

    // |===| underlying stream errors or unknowns
    ReadFailure,
    WriteFailure,

    // |===| stream buffer errors
    StreamBytesEmpty,
    StreamBytesFull,

    // |===| stream contract errors
    StreamClosed,
    StreamOutputClosed,

    // |===| stream handling errors
    StreamMessageTooLong,
    StreamMessageCountZero,

    // |===| stream protocol errors
    // this means that frames can not be recreated from the current
    // stream state. User must decide how to handle this
    // error (i.e. drpo the connection or flush the stream)
    // frame heading consists of: (in order)
    // 1x 0x00 byte,
    // 2 bytes for frame length (LEN),
    // 1 byte for identifier,
    // LEN bytes for data
    FramingDelimiterMismatch,

    // not always an error, either we need to wait for more data or we lost
    // something and the stream has to be reset or dropped.
    // If for some reason we lost something, we won't be able to recover naturally.
    FramingTooLittleData,

    // fatal error, stream can not be recovered. Option is to either drop or reset.
    StreamFailure,
}

#[derive(Debug, Default, PartialEq)]
pub struct NetStreamErr {
    pub category: NetStreamErrorType,
}

impl NetStreamErr {
    pub fn new(category: NetStreamErrorType) -> Self {
        Self {
            category,
        }
    }
}

impl From<NetFrameError> for NetStreamErr {
    fn from(e: NetFrameError) -> Self {
        match e {
            NetFrameError::DelimiterMismatch => {
                Self {
                    category: NetStreamErrorType::FramingDelimiterMismatch,
                }
            }
            NetFrameError::TooLittleData => {
                Self {
                    category: NetStreamErrorType::FramingTooLittleData,
                }
            }
            _ => {
                Self {
                    category: NetStreamErrorType::Unknown,
                }
            }
        }
    }
}
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
