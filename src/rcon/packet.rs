use tokio::io::AsyncReadExt;

#[derive(Clone, Copy)]
pub enum RconPacketType {
    Auth,
    AuthResponse,
    ExecCommand,
    ResponseValue,
    Undefined(i32)
}

impl RconPacketType {
    pub fn to_i32(self) -> i32 {
        match self {
            RconPacketType::Auth => { 3 }
            RconPacketType::AuthResponse => { 2 }
            RconPacketType::ExecCommand => { 2 }
            RconPacketType::ResponseValue => { 0 }
            RconPacketType::Undefined(rcon_packet_type_number) => { rcon_packet_type_number }
        }
    }

    pub fn from_i32(rcon_packet_type_number: i32, is_server_response: bool) -> RconPacketType {
        match rcon_packet_type_number {
            3 => RconPacketType::Auth,
            2 if is_server_response => RconPacketType::AuthResponse,
            2 => RconPacketType::ExecCommand,
            0 => RconPacketType::ResponseValue,
            rcon_packet_type_number => RconPacketType::Undefined(rcon_packet_type_number)
        }
    }
}

pub struct RconPacket {
    size: i32,
    id: i32,
    packet_type: RconPacketType,
    body: String
}

impl RconPacket {
    pub fn new(id: i32, packet_type: RconPacketType, body: String) -> RconPacket {
        RconPacket {
            size: 10 + body.len() as i32,
            id,
            packet_type,
            body,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.size as usize);
        buffer.extend_from_slice(&self.size.to_le_bytes());
        buffer.extend_from_slice(&self.id.to_le_bytes());
        buffer.extend_from_slice(&self.packet_type.to_i32().to_le_bytes());
        buffer.extend_from_slice(&self.body.as_bytes());
        buffer.extend_from_slice(&[0x00, 0x00]);
        return buffer;
    }

    pub async fn deserialize<T: Unpin + tokio::io::AsyncRead>(incoming_stream :&mut T) -> RconPacket {
        let mut buffer = [0u8; 4];

        let error_packet = RconPacket::new(-90, RconPacketType::Undefined(-90), "".to_string());
        let size = match incoming_stream.read_exact(&mut buffer).await {
            Ok(_) => { i32::from_le_bytes(buffer) }
            Err(error) => {
                tracing::error!("Can't deserialize size of packet. Error: {}", error);
                return error_packet;
            }
        };

        let id = match incoming_stream.read_exact(&mut buffer).await {
            Ok(_) => { i32::from_le_bytes(buffer) }
            Err(error) => {
                tracing::error!("Can't deserialize id of packet. Error: {}", error);
                return error_packet;
            }
        };

        let packet_type = match incoming_stream.read_exact(&mut buffer).await {
            Ok(_) => { i32::from_le_bytes(buffer) }
            Err(error) => {
                tracing::error!("Can't deserialize type of packet. Error: {}", error);
                return error_packet;
            }
        };

        let body_size = size - 10;
        let mut body_buffer = Vec::with_capacity(body_size as usize);

        let body = match incoming_stream.read_exact(&mut body_buffer).await {
            Ok(_) => { match String::from_utf8(body_buffer) {
                Ok(b_buffer) => { b_buffer }
                Err(error) => {
                    tracing::error!("Can't read string from deserialized string buffer. Error: {}", error);
                    return error_packet;
                }
            }}
            Err(error) => {
                tracing::error!("Can't deserialize body of packet. Error: {}", error);
                return error_packet;
            }
        };
        
        let mut terminating_buffer = [0u8; 2];
        match incoming_stream.read_exact(&mut terminating_buffer).await {
            Ok(_) => {}
            Err(_) => {
                tracing::error!("Can't deserialize terminating zeros of packet. Error: {}", error);
                return error_packet;
            }
        }
        
        RconPacket {
            size,
            id,
            packet_type: RconPacketType::from_i32(packet_type, true),
            body,
        }
    }

    pub fn get_id(&self) -> i32 { self.id }
    pub fn get_type(&self) -> RconPacketType { self.packet_type }
    pub fn get_body(&self) -> &str { &self.body }
    pub fn is_error(&self) -> bool { return self.id < 0; }
}