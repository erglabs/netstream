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

// ┌───────────┬───────┬────────┬──────────┐
// │   8bit    │ 8bit  │ 16bit  │ (length) │
// │           │       │        │          │
// │ delimiter │  tag  │ length │   data   │
// └───────────┴───────┴────────┴──────────┘
//
// * delimiter is always 0x00
// * tag is optional and is passed to handlers
// * length is the length of the data
// * data is the actual data, max 65535 bytes
//
// During write, stream will try to decode the messages and push
// them to the message buffer
//
// NetStream can hold up to 256 messages, subsequent writes will fail
// NetStream will buffer up to 3*65535 bytes, if the buffer is full
// or can not be decoded, supsequent writes will fail.
//
// call to next() will take one message from the stack and return it in
// decoded form (as a struct)
//
//
// transformations on the data from socket:
// - length field will be converted from big endian to little endian on
//   receiving
// - length field will be converted from little endian to big endian on sending
// the rest of the data is passed as is
// tags are passed to the conection handler with the message
// those are optional


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NetFrameTag {
    // Undefiled, no special tags, just a message, use if you are too lazy
    GenericMessage,

    // self enclosed message, no need to wait for more packets
    SingleMessage,

    // message is split into multiple packets
    // first N bytes in data should be packet identifiers,
    // but its on the protocol handler to define that
    MultiMessage,

    // message to the connection handler
    Control,

    // connection established
    Hello,

    // connection closed
    Goodbye,

    // ping
    Ping,

    // ping response
    Pong,

    // reset request/notification
    Reset,

    // undefined tag
    Undefined,
}

impl From<u8> for NetFrameTag {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => NetFrameTag::GenericMessage,
            0x01 => NetFrameTag::SingleMessage,
            0x02 => NetFrameTag::MultiMessage,
            0x03 => NetFrameTag::Control,
            0x04 => NetFrameTag::Hello,
            0x05 => NetFrameTag::Goodbye,
            0x06 => NetFrameTag::Ping,
            0x07 => NetFrameTag::Pong,
            0x08 => NetFrameTag::Reset,
            _ => NetFrameTag::Undefined,
        }
    }
}

impl From<NetFrameTag> for u8 {
    fn from(what: NetFrameTag) -> Self {
        match what {
            NetFrameTag::GenericMessage => 0x00,
            NetFrameTag::SingleMessage => 0x01,
            NetFrameTag::MultiMessage => 0x02,
            NetFrameTag::Control => 0x03,
            NetFrameTag::Hello => 0x04,
            NetFrameTag::Goodbye => 0x05,
            NetFrameTag::Ping => 0x06,
            NetFrameTag::Pong => 0x07,
            NetFrameTag::Reset => 0x08,
            NetFrameTag::Undefined => 0xFF,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct NetFrameMetadata {
    pub tag: u8,
    pub size: u16,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NetFrame {
    pub tag: u8,
    pub data: Vec<u8>,
}
