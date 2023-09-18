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

use crate::netframe::{
    error::NetFrameError,
    types::{NetFrame, NetFrameMetadata},
};


#[test]
fn frame_metadata_failure_bad_delimiter() {
    // delimiter is 0x00, but we have 0x01
    // rest of the data is invalid
    let buffer: Vec<u8> = vec![0x01, 0x00, 0x00, 0x00, 0x00];
    let wanted_result: Result<NetFrameMetadata, NetFrameError> =
        Err(NetFrameError::DelimiterMismatch);
    // expect error
    assert_eq!(NetFrame::get_metadata(&buffer), wanted_result);
}

#[test]
fn frame_metadata_ok_proper_metadata_zero_size() {
    // delimiter is 0x00, correct
    // tag is 0x00, correct
    // size is 0x0000, correct
    // rest of the data is valid but since size==0, it's not used to de-frame
    let buffer: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00, 0x00];
    // expected frame
    let wanted_result: Result<NetFrameMetadata, NetFrameError> = Ok(NetFrameMetadata {
        tag: 0x00,
        size: 0x0000,
    });
    // expect Ok
    assert_eq!(NetFrame::get_metadata(&buffer), wanted_result);
}

#[test]
fn frame_metadata_ok_byteorder_test() {
    // delimiter is 0x00, correct
    // tag is 0x00, correct
    // size is 0x0100, correct (its in network byteorder, usize 256 bytes)
    // rest of the data is invalid, but we are testing metadata only
    let buffer: Vec<u8> = vec![0x00, 0x00, 0x01, 0x00, 0x00];
    // expected frame
    let wanted_result: Result<NetFrameMetadata, NetFrameError> = Ok(NetFrameMetadata {
        tag: 0x00,
        size: 0x0100,
    });
    // expect Ok
    assert_eq!(NetFrame::get_metadata(&buffer), wanted_result);
}
