use std::io::Read;

use tokio_util::bytes::{BufMut, BytesMut};
use tungstenite::Message;

use crate::api::typedef::BackendError;

/**
* Read NUL-terminated string from AsyncRead implementation
*/
pub fn read_string(reader: &mut (impl Read + Unpin)) -> Result<String, BackendError> {
    const MAX_LEN: usize = 1024;

    let mut data = [0u8; 1];
    let mut res = vec![];
    loop {
        reader.read_exact(&mut data)?;
        if data[0] == 0 {
            break;
        }
        res.push(data[0]);

        if res.len() > MAX_LEN { return Err("String is way too large (1024+ bytes)".into()); }
    }

    Ok(String::from_utf8(res)?)
}

pub fn encode_msg(source: &str, reader: &mut (impl Read + Unpin)) -> Result<Message, BackendError> {
    let mut bytebuf = BytesMut::new();

    bytebuf.put_slice(source.as_bytes());
    bytebuf.put_u8(0);

    let mut buf = [0u8; 2048];
    while let rd = reader.read(&mut buf)? && rd > 0 {
        bytebuf.put_slice(&mut buf[..rd]);
    }

    Ok(Message::Binary(bytebuf.freeze()))
}
