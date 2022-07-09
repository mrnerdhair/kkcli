use super::{MessageHandler, ProtocolAdapter, Transport};
use crate::messages::{self, Message};
use anyhow::{anyhow, bail, Result};
use std::io::{stdin, stdout, Write};

pub struct DeviceProtocolAdapter<T: Transport> {
    transport: T,
    pub verbose: bool,
}

impl<T: Transport<Error = E>, E: std::error::Error + Send + Sync + 'static>
    DeviceProtocolAdapter<T>
{
    pub const fn new(transport: T) -> Self {
        Self {
            transport,
            verbose: false,
        }
    }

    pub fn take(self) -> T {
        self.transport
    }

    fn standard_handler(msg: &Message) -> Result<Option<Message>> {
        Ok(match msg {
            Message::ButtonRequest(_) => {
                println!("Confirm action on device...");
                Some(messages::ButtonAck::default().into())
            }
            Message::PinMatrixRequest(x) => {
                match x.r#type {
                    Some(t) => match messages::PinMatrixRequestType::from_i32(t)
                        .ok_or_else(|| anyhow!("unrecognized PinMatrixRequestType ({})", t))?
                    {
                        messages::PinMatrixRequestType::Current => print!("Enter current PIN: "),
                        messages::PinMatrixRequestType::NewFirst => print!("Enter new PIN: "),
                        messages::PinMatrixRequestType::NewSecond => print!("Re-enter new PIN: "),
                    },
                    None => bail!("expected PinMatrixRequestType"),
                }
                stdout().flush().unwrap();
                let mut pin = String::new();
                stdin().read_line(&mut pin)?;
                let pin = pin.trim();
                Some(
                    messages::PinMatrixAck {
                        pin: pin.to_owned(),
                    }
                    .into(),
                )
            }
            Message::PassphraseRequest(_) => {
                print!("Enter BIP-39 passphrase: ");
                stdout().flush().unwrap();
                let passphrase = passterm::read_password()?;
                Some(
                    messages::PassphraseAck {
                        passphrase: passphrase,
                    }
                    .into(),
                )
            }
            Message::Failure(x) => bail!("Failure: {}", x.message()),
            _ => None,
        })
    }
}

impl<T: Transport<Error = E>, E: std::error::Error + Send + Sync + 'static> ProtocolAdapter
    for DeviceProtocolAdapter<T>
{
    fn send(&self, msg: Message) -> Result<Message> {
        let read_timeout = msg.read_timeout();
        self.send_one_way(msg)?;

        let mut in_buf = Vec::<u8>::new();
        self.transport.read(&mut in_buf, read_timeout)?;

        let out = Message::decode(&mut in_buf.as_slice()).map_err(|x| anyhow!(x))?;
        if self.verbose {
            println!("<- {:?}", out);
        }
        Ok(out)
    }

    fn send_one_way(&self, msg: Message) -> Result<()> {
        if self.verbose {
            println!("-> {:?}", msg);
        }
        let mut out_buf = Vec::<u8>::with_capacity(msg.encoded_len());
        msg.encode(&mut out_buf)?;
        self.transport.write(&mut out_buf, msg.write_timeout())?;

        Ok(())
    }

    fn send_and_handle(&self, msg: Message) -> Result<Message> {
        self.send_and_handle_or(msg, &mut |_| Ok(None))
    }

    fn send_and_handle_or(
        &self,
        mut msg: Message,
        handler: &mut MessageHandler,
    ) -> Result<Message> {
        loop {
            let out = self.send(msg)?;
            match Self::standard_handler(&out).map(|x| {
                x.ok_or(())
                    .map(|y| Some(y))
                    .map_or_else(|_| handler(&out), |y| Ok(y))
            })?? {
                Some(x) => msg = x,
                None => return Ok(out),
            }
        }
    }
}
