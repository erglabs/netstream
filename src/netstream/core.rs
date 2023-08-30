use std::collections::VecDeque;

use crate::{
    netframe::{consts::NETFRAME_HEADER_SIZE_BYTES, types::NetFrame},
    netstream::{
        consts::*,
        error::{NetStreamErr, NetStreamErrorType},
        types::{FramingStream, NetStream, NetStreamState},
    },
};

impl NetStream {
    pub fn new() -> Self {
        Self {
            frames: VecDeque::with_capacity(NETSTREAM_EXTERNAL_CAPACITY),
            buffer: Vec::with_capacity(NETSTREAM_INTERNAL_CAPACITY),
            state: NetStreamState::Empty,
        }
    }
}

impl FramingStream for NetStream {
    fn next(&mut self) -> Result<NetFrame, NetStreamErr> {
        if self.frames.is_empty() {
            return Err(NetStreamErr {
                category: NetStreamErrorType::StreamMessageCountZero,
            });
        }

        self.frames.pop_front().ok_or(NetStreamErr {
            category: NetStreamErrorType::StreamFailure,
        })
    }

    fn write(
        &mut self,
        data: Vec<u8>,
    ) -> Result<(), NetStreamErr> {
        // we do not have enough data to even ask for metadata
        if data.len() + self.buffer.len() < NETFRAME_HEADER_SIZE_BYTES {
            self.buffer.extend(data);

            self.state = NetStreamState::InProgress;

            return Ok(());
        }

        match self.state {
            NetStreamState::Empty => {
                match NetFrame::get_metadata(&data) {
                    Ok(metadata) => {
                        match data
                            .len()
                            .cmp(&(metadata.size as usize + NETFRAME_HEADER_SIZE_BYTES))
                        {
                            std::cmp::Ordering::Equal => {
                                let frame: NetFrame = NetFrame {
                                    tag: metadata.tag,
                                    data: data[NETFRAME_HEADER_SIZE_BYTES..].to_vec(),
                                };
                                self.frames.push_back(frame);
                                Ok(())
                            }
                            std::cmp::Ordering::Less => {
                                self.buffer.extend(data);
                                self.state = NetStreamState::InProgress;

                                Ok(())
                            }
                            std::cmp::Ordering::Greater => {
                                let frame: NetFrame = NetFrame {
                                    tag: metadata.tag,
                                    data: data[NETFRAME_HEADER_SIZE_BYTES
                                        ..NETFRAME_HEADER_SIZE_BYTES + (metadata.size as usize)]
                                        .to_vec(),
                                };

                                self.frames.push_back(frame);

                                let remain = data
                                    [NETFRAME_HEADER_SIZE_BYTES + (metadata.size as usize)..]
                                    .to_vec();

                                self.buffer = remain;

                                self.state = NetStreamState::InProgress;

                                Ok(())
                            }
                        }
                    }
                    Err(err) => Err(err.into()),
                }
            }
            NetStreamState::InProgress => {
                self.buffer.extend(data);

                match NetFrame::get_metadata(&self.buffer) {
                    Ok(metadata) => {
                        match self
                            .buffer
                            .len()
                            .cmp(&(metadata.size as usize + NETFRAME_HEADER_SIZE_BYTES))
                        {
                            std::cmp::Ordering::Equal => {
                                let frame: NetFrame = NetFrame {
                                    tag: metadata.tag,
                                    data: self.buffer[NETFRAME_HEADER_SIZE_BYTES..].to_vec(),
                                };

                                self.frames.push_back(frame);

                                self.buffer.clear();

                                self.state = NetStreamState::Empty;

                                Ok(())
                            }
                            std::cmp::Ordering::Less => {
                                self.state = NetStreamState::InProgress;

                                Ok(())
                            }
                            std::cmp::Ordering::Greater => {
                                let frame: NetFrame = NetFrame {
                                    tag: metadata.tag,
                                    data: self.buffer[NETFRAME_HEADER_SIZE_BYTES
                                        ..NETFRAME_HEADER_SIZE_BYTES + (metadata.size as usize)]
                                        .to_vec(),
                                };

                                let remain = self.buffer
                                    [NETFRAME_HEADER_SIZE_BYTES + (metadata.size as usize)..]
                                    .to_vec();

                                self.frames.push_back(frame);

                                self.buffer = remain;

                                self.state = NetStreamState::InProgress;

                                Ok(())
                            }
                        }
                    }
                    Err(err) => Err(err.into()),
                }
            }
            NetStreamState::Failure => {
                Err(NetStreamErr {
                    category: NetStreamErrorType::StreamFailure,
                })
            }
        }
    }
}
