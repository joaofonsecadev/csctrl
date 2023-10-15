use tokio::io::AsyncWriteExt;
use crate::rcon::packet::{RconPacket, RconPacketType};

pub struct RconConnection {
    address: String,
    password: String,
    current_packet_id: i32,
    is_valid: bool,
    tcp_stream: std::sync::OnceLock<tokio::net::TcpStream>,
}

impl RconConnection {
    pub fn create_rcon_connection(address: &str, password: &str) -> RconConnection {
        RconConnection {
            address: address.to_string(),
            password: password.to_string(),
            current_packet_id: -1,
            is_valid: false,
            tcp_stream: std::sync::OnceLock::new(),
        }
    }

    pub async fn init_rcon_connection(&mut self) {
        match tokio::net::TcpStream::connect(&self.address).await {
            Ok(stream) => {
                self.tcp_stream.get_or_init(|| stream);
                tracing::trace!("TCP stream established");
            }
            Err(error) => {
                tracing::error!("Can't establish a TCP stream to the server. Error: {}", error);
                return;
            }
        }
        self.authenticate().await;
    }

    async fn authenticate(&mut self) {
        if self.send_packet(RconPacketType::Auth, &self.password.clone()).await < 0 {
            tracing::error!("Failed authentication: can't send authentication packet");
            return;
        }

        let start_time = std::time::SystemTime::now();
        let received_packet = loop {
            if start_time.elapsed().unwrap().as_secs() > 5 {
                tracing::error!("Failed authentication: receiving response timed out");
                return;
            }
            let received_packet = self.receive_packet().await;
            if received_packet.get_type() == RconPacketType::AuthResponse {
                break received_packet
            }
        };

        if received_packet.is_error() {
            tracing::error!("Failed authentication: the provided password is incorrect");
            return;
        }

        self.is_valid = true;
    }

    pub async fn execute_command(&mut self, command: &str) -> Result<String, String> {
        if self.send_packet(RconPacketType::ExecCommand, command).await < 0 {
            return Err("Failed command execution: can't send command packet".to_string());
        }

        let empty_request_id = self.send_packet(RconPacketType::ResponseValue, "").await;
        if empty_request_id < 0 {
            return Err("Failed command execution: can't send empty command packet".to_string());
        }

        let mut response_body = "".to_string();
        let start_time = std::time::SystemTime::now();
        loop {
            if start_time.elapsed().unwrap().as_secs() > 5 {
                return Err("Failed command execution: didn't receive response before timeout".to_string());
            }

            let received_packet = self.receive_packet().await;
            if received_packet.get_id() == empty_request_id {
                return Ok(response_body);
            }

            response_body += received_packet.get_body();
        }
    }

    async fn send_packet(&mut self, packet_type: RconPacketType, body: &str) -> i32 {
        let id = self.get_new_packet_id();
        let packet = RconPacket::new(id, packet_type, body.to_string());
        let serialized_packet = packet.serialize();
        let tcp_stream = self.tcp_stream.get_mut().unwrap();
        match tcp_stream.write_all(&serialized_packet).await {
            Ok(_) => { }
            Err(error) => {
                tracing::error!("Can't send packet. Error: {}", error);
                return -999;
            }
        };
        return id;
    }

    async fn receive_packet(&mut self) -> RconPacket {
        return RconPacket::deserialize(self.tcp_stream.get_mut().unwrap()).await;
    }

    fn get_new_packet_id(&mut self) -> i32 {
        let int_overflowed = self.current_packet_id.checked_add(1).is_none();
        if int_overflowed { self.current_packet_id = 0; }
        else { self.current_packet_id += 1; }
        return self.current_packet_id;
    }

    pub fn get_is_valid(&self) -> bool { return self.is_valid; }
}