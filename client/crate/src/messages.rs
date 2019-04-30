use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use oxygengine::prelude::*;
use std::io::{Cursor, Read, Write};

#[derive(Debug, Clone)]
pub enum MessageData {
    Unknown,
    InitPlayer(MsgPlayerInfo),
    NewPlayer(MsgPlayerInfo),
    PlayerState(MsgPlayerState),
    PlayerDisconnected(u32),
    PlayerEat(f32),
}

impl MessageData {
    pub fn id(&self) -> u32 {
        match self {
            MessageData::InitPlayer(_) => 1,
            MessageData::NewPlayer(_) => 2,
            MessageData::PlayerState(_) => 3,
            MessageData::PlayerDisconnected(_) => 4,
            MessageData::PlayerEat(_) => 5,
            _ => 0,
        }
    }
}

impl From<(MessageID, &[u8])> for MessageData {
    fn from((id, data): (MessageID, &[u8])) -> Self {
        let stream = &mut Cursor::new(data);
        match id.id() {
            1 => MessageData::InitPlayer(MsgPlayerInfo::msg_read(stream).unwrap()),
            2 => MessageData::NewPlayer(MsgPlayerInfo::msg_read(stream).unwrap()),
            3 => MessageData::PlayerState(MsgPlayerState::msg_read(stream).unwrap()),
            4 => MessageData::PlayerDisconnected(stream.read_u32::<BigEndian>().unwrap()),
            5 => MessageData::PlayerEat(stream.read_f32::<BigEndian>().unwrap()),
            _ => MessageData::Unknown,
        }
    }
}

impl Into<Vec<u8>> for MessageData {
    fn into(self) -> Vec<u8> {
        let mut stream = Cursor::new(vec![]);
        match self {
            MessageData::PlayerState(state) => state.msg_write(&mut stream),
            _ => {}
        }
        stream.into_inner()
    }
}

pub trait Message: Sized {
    fn msg_read<R>(_stream: &mut R) -> Option<Self>
    where
        R: Read,
    {
        None
    }

    fn msg_write<W>(&self, _stream: &mut W)
    where
        W: Write,
    {
    }
}

impl Message for String {
    fn msg_read<R>(stream: &mut R) -> Option<Self>
    where
        R: Read + ReadBytesExt,
    {
        let size = stream.read_u32::<BigEndian>().unwrap();
        let mut buff = vec![0; size as usize];
        stream.read_exact(&mut buff).unwrap();
        Some(String::from_utf8(buff).unwrap())
    }

    fn msg_write<W>(&self, stream: &mut W)
    where
        W: Write + WriteBytesExt,
    {
        let bytes = self.as_bytes();
        stream.write_u32::<BigEndian>(bytes.len() as u32).unwrap();
        stream.write(bytes).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct MsgPlayerInfo {
    pub id: u32,
    pub name: String,
    pub time: f32,
    pub position: Vec2,
}

impl Message for MsgPlayerInfo {
    fn msg_read<R>(stream: &mut R) -> Option<Self>
    where
        R: Read + ReadBytesExt,
    {
        let id = stream.read_u32::<BigEndian>().unwrap();
        let name = String::msg_read(stream).unwrap();
        let time = stream.read_f32::<BigEndian>().unwrap();
        let position = {
            let x = stream.read_f32::<BigEndian>().unwrap();
            let y = stream.read_f32::<BigEndian>().unwrap();
            Vec2::new(x, y)
        };
        Some(Self {
            id,
            name,
            time,
            position,
        })
    }

    fn msg_write<W>(&self, stream: &mut W)
    where
        W: Write + WriteBytesExt,
    {
        stream.write_u32::<BigEndian>(self.id).unwrap();
        self.name.msg_write(stream);
        stream.write_f32::<BigEndian>(self.time).unwrap();
        stream.write_f32::<BigEndian>(self.position.x).unwrap();
        stream.write_f32::<BigEndian>(self.position.y).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct MsgPlayerState {
    pub id: u32,
    pub time: f32,
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Message for MsgPlayerState {
    fn msg_read<R>(stream: &mut R) -> Option<Self>
    where
        R: Read + ReadBytesExt,
    {
        let id = stream.read_u32::<BigEndian>().unwrap();
        let time = stream.read_f32::<BigEndian>().unwrap();
        let position = {
            let x = stream.read_f32::<BigEndian>().unwrap();
            let y = stream.read_f32::<BigEndian>().unwrap();
            Vec2::new(x, y)
        };
        let velocity = {
            let x = stream.read_f32::<BigEndian>().unwrap();
            let y = stream.read_f32::<BigEndian>().unwrap();
            Vec2::new(x, y)
        };
        Some(Self {
            id,
            time,
            position,
            velocity,
        })
    }

    fn msg_write<W>(&self, stream: &mut W)
    where
        W: Write + WriteBytesExt,
    {
        stream.write_u32::<BigEndian>(self.id).unwrap();
        stream.write_f32::<BigEndian>(self.time).unwrap();
        stream.write_f32::<BigEndian>(self.position.x).unwrap();
        stream.write_f32::<BigEndian>(self.position.y).unwrap();
        stream.write_f32::<BigEndian>(self.velocity.x).unwrap();
        stream.write_f32::<BigEndian>(self.velocity.y).unwrap();
    }
}
