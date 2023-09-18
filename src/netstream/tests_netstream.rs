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

use crate::{
    netframe::types::NetFrame,
    netstream::{
        error::NetStreamErr,
        types::{FramingStream, NetStream, NetStreamState},
    },
};


#[test]
fn netstream_ok_write_frame_and_next() {
    let mut stream = NetStream::new();
    // delimiter is 0x00, correct
    // tag is 0x00, correct
    // size is 1 in network byte order
    let buffer: Vec<u8> = vec![0x00, 0x00, 0x00, 0x01, 0xAB];
    let wanted_result: Result<(), NetStreamErr> = Ok(());
    // write succeseded
    assert_eq!(stream.write(buffer), wanted_result);
    // desired frame
    let wanted_frame: NetFrame = NetFrame {
        tag: 0x00,
        data: vec![0xAB],
    };
    // next frame is ready, pop it off
    assert_eq!(stream.clone().next(), Ok(wanted_frame));
    // no more frames
    assert_eq!(stream.clone().state, NetStreamState::Empty);
}

#[test]
fn netstream_ok_write_in_progress() {
    let mut stream = NetStream::new();
    // delimiter is 0x00, correct
    // tag is 0x00, correct
    // size is 1 in network byte order
    let buffer0: Vec<u8> = vec![0x00, 0x00];
    let buffer1: Vec<u8> = vec![0x00, 0x01];
    let buffer2: Vec<u8> = vec![0xAB];
    // desired frame
    let wanted_frame: NetFrame = NetFrame {
        tag: 0x00,
        data: vec![0xAB],
    };
    // too little data
    assert_eq!(stream.write(buffer0), Ok(()));
    assert_eq!(
        stream.next(),
        Err(NetStreamErr {
            category: crate::netstream::error::NetStreamErrorType::StreamMessageCountZero,
        })
    );
    // too little data
    assert_eq!(stream.write(buffer1), Ok(()));
    assert_eq!(
        stream.next(),
        Err(NetStreamErr {
            category: crate::netstream::error::NetStreamErrorType::StreamMessageCountZero,
        })
    );
    // enough for one frame
    assert_eq!(stream.write(buffer2), Ok(()));
    // next frame is ready, pop it off
    assert_eq!(stream.clone().next(), Ok(wanted_frame));
    // no more frames
    assert_eq!(stream.clone().state, NetStreamState::Empty);
}
