use std::{
    io::{stdin, stdout, Write},
    time::Duration,
};

use crate::{
    messages::{self, Message},
    transport::Transport,
};
use anyhow::{anyhow, bail, Result};

const TIMEOUT: Duration = Duration::from_millis(5000);
const LONG_TIMEOUT: Duration = Duration::from_millis(5 * 60 * 1000);

pub trait StateMachine {
    fn send(&self, msg: Message) -> Result<Message>;
    fn send_and_handle(&self, msg: Message) -> Result<Message>;
    fn send_and_handle_or(
        &self,
        msg: Message,
        handler: &mut dyn FnMut(&Message) -> Result<Option<Message>>,
    ) -> Result<Message>;
}

pub struct TransportStateMachine<'a, T: Transport> {
    transport: &'a T,
    pub verbose: bool,
}

impl<'a, T: Transport<Error = E>, E: std::error::Error + Send + Sync + 'static>
    TransportStateMachine<'a, T>
{
    pub const fn new(transport: &'a T) -> Self {
        Self {
            transport,
            verbose: false,
        }
    }

    fn read_timeout_for_message(x: &Message) -> Duration {
        match x {
            Message::ButtonAck(_) => LONG_TIMEOUT,
            _ => TIMEOUT,
        }
    }

    fn write_timeout_for_message(x: &Message) -> Duration {
        match x {
            Message::FirmwareUpload(_) => LONG_TIMEOUT,
            _ => TIMEOUT,
        }
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

impl<'a, T: Transport<Error = E>, E: std::error::Error + Send + Sync + 'static> StateMachine
    for TransportStateMachine<'a, T>
{
    fn send(&self, msg: Message) -> Result<Message> {
        if self.verbose {
            println!("-> {:?}", msg);
        }
        let mut out_buf = Vec::<u8>::with_capacity(msg.encoded_len());
        msg.encode(&mut out_buf)?;
        self.transport
            .write(&mut out_buf, Self::write_timeout_for_message(&msg))?;

        let mut in_buf = Vec::<u8>::new();
        self.transport
            .read(&mut in_buf, Self::read_timeout_for_message(&msg))?;

        let out = Message::decode(&mut in_buf.as_slice()).map_err(|x| anyhow!(x))?;
        if self.verbose {
            println!("<- {:?}", out);
        }
        Ok(out)
    }

    fn send_and_handle(&self, msg: Message) -> Result<Message> {
        self.send_and_handle_or(msg, &mut |_| Ok(None))
    }

    fn send_and_handle_or(
        &self,
        mut msg: Message,
        handler: &mut dyn FnMut(&Message) -> Result<Option<Message>>,
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
