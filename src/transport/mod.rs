pub mod usb;
pub mod protocol_adapter;

pub use usb::*;
pub use protocol_adapter::*;

use anyhow::Result;
use core::time::Duration;
use crate::messages::Message;

pub trait Transport {
    type Error: std::error::Error;
    fn write(&self, msg: &[u8], timeout: Duration) -> Result<usize, Self::Error>;
    fn read(&self, buf: &mut Vec<u8>, timeout: Duration) -> Result<(), Self::Error>;
}

pub trait ProtocolAdapter {
    fn send(&self, msg: Message) -> Result<Message>;
    fn send_one_way(&self, msg: Message) -> Result<()>;
    fn send_and_handle(&self, msg: Message) -> Result<Message>;
    fn send_and_handle_or(
        &self,
        msg: Message,
        handler: &mut MessageHandler<'_>,
    ) -> Result<Message>;
}

pub type MessageHandler<'a> = dyn FnMut(&Message) -> Result<Option<Message>> + 'a;
