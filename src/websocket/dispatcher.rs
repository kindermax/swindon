use tokio_core::channel::{channel, Receiver, Sender};
use tokio_core::reactor::Handle;
use tk_bufstream::{Buf};

use super::{Frame, Error};
use websocket::write::WriteExt;

pub enum OutFrame {
    Text(String),
    Binary(Vec<u8>),
}


pub trait Dispatcher {
    /// Temporary solution is to output data directly
    fn dispatch(&mut self, frame: Frame, replier: &mut ImmediateReplier)
        -> Result<(), Error>;
}

pub struct ImmediateReplier<'a>(&'a mut Buf);

#[derive(Clone)]
pub struct RemoteReplier {
    channel: Sender<OutFrame>,
}

impl<'a> ImmediateReplier<'a> {
    pub fn new(buf: &'a mut Buf) -> ImmediateReplier<'a> {
        ImmediateReplier(buf)
    }
    pub fn pong(&mut self, data: &[u8]) {
        self.0.write_packet(0xA, data);
    }
    pub fn text(&mut self, data: &str) {
        self.0.write_packet(0x1, data.as_bytes());
    }
    pub fn binary(&mut self, data: &[u8]) {
        self.0.write_packet(0x2, data);
    }
}

impl RemoteReplier {
    pub fn pair(handle: &Handle) -> (RemoteReplier, Receiver<OutFrame>) {
        let (tx, rx) = channel(handle).expect("channel created");
        return (
            RemoteReplier {
                channel: tx,
            },
            rx);
    }
    pub fn send_text<S: Into<String>>(&self, s: S) -> Result<(), Error> {
        // TODO(tailhook) this error type is misleading
        self.channel.send(OutFrame::Text(s.into())).map_err(|e| e.into())
    }
    pub fn send_binary<B: Into<Vec<u8>>>(&self, b: B) -> Result<(), Error> {
        // TODO(tailhook) this error type is misleading
        self.channel.send(OutFrame::Binary(b.into())).map_err(|e| e.into())
    }
}