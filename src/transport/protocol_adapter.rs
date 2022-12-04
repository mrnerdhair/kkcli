use super::{ProtocolAdapter, Transport};
use crate::messages::Message;
use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    pub static ref VERBOSE: RwLock<bool> = RwLock::new(false);
}

impl<T, E> ProtocolAdapter for T
where
    T: Transport<Error = E>,
    E: std::error::Error + Send + Sync + 'static,
{
    fn reset(&mut self) -> Result<()> {
        Ok(<T as Transport>::reset(self)?)
    }

    fn send(&mut self, msg: Message) -> Result<()> {
        if *VERBOSE.read().unwrap() {
            println!("-> {:?}", msg);
        }
        let mut out_buf = Vec::<u8>::with_capacity(msg.encoded_len());
        msg.encode(&mut out_buf)?;
        self.write(&out_buf, msg.write_timeout())?;

        Ok(())
    }

    fn as_mut_dyn(&mut self) -> &mut dyn ProtocolAdapter {
        self
    }

    fn handle(&mut self, msg: Message) -> Result<Message> {
        let read_timeout = msg.read_timeout();
        self.send(msg)?;

        let mut in_buf = Vec::<u8>::new();
        self.read(&mut in_buf, read_timeout)?;

        let out = Message::decode(&mut in_buf.as_slice()).map_err(|x| anyhow!(x))?;
        if *VERBOSE.read().unwrap() {
            println!("<- {:?}", out);
        }
        Ok(out)
    }
}
