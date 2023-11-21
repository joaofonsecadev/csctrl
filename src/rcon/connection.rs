use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::rcon::packet::{RconPacket, RconPacketType};

pub struct RconConnection {
    address: String,
    password: String,
    current_packet_id: i32,
    is_valid: bool,
}

impl RconConnection {
    pub fn create_rcon_connection(address: &str, password: &str) -> RconConnection {
        RconConnection {
            address: address.to_string(),
            password: password.to_string(),
            current_packet_id: -1,
            is_valid: false,
        }
    }

    async fn get_tcp_stream(&self) -> Result<TcpStream, String> {
        return match tokio::net::TcpStream::connect(&self.address).await {
            Ok(stream) => {
                Ok(stream)
            }
            Err(error) => {
                Err(format!("Can't establish a TCP stream to the server. Error: {}", error))
            }
        };
    }

    async fn authenticate(&mut self, tcp_stream: &mut TcpStream) -> bool {
        if self.send_packet(tcp_stream, RconPacketType::Auth, &self.password.clone()).await < 0 {
            tracing::error!("Failed authentication: can't send authentication packet");
            return false;
        }

        let start_time = std::time::SystemTime::now();
        let received_packet = loop {
            if start_time.elapsed().unwrap().as_secs() > 5 {
                tracing::error!("Failed authentication: receiving response timed out");
                return false;
            }
            let received_packet = self.receive_packet(tcp_stream).await;
            if received_packet.get_type() == RconPacketType::AuthResponse {
                break received_packet
            }
        };

        if received_packet.is_error() {
            tracing::error!("Failed authentication: the provided password is incorrect");
            return false;
        }

        return true;
    }

    pub async fn execute_command(&mut self, command: &str) -> Result<String, String> {
        let mut tcp_stream = match self.get_tcp_stream().await {
            Ok(stream) => { stream }
            Err(error) => {
                tracing::error!(error);
                return Err(error.to_string());
            }
        };
        let authenticated = self.authenticate(&mut tcp_stream).await;
        if !authenticated {
            return Err("Failed command execution: can't authenticate".to_string());
        }

        if self.send_packet(&mut tcp_stream, RconPacketType::ExecCommand, command).await < 0 {
            return Err("Failed command execution: can't send command packet".to_string());
        }

        let empty_request_id = self.send_packet(&mut tcp_stream, RconPacketType::ExecCommand, "echo CsctrlTerminatingRconCommand").await;
        if empty_request_id < 0 {
            return Err("Failed command execution: can't send empty command packet".to_string());
        }

        let mut response_body = "".to_string();
        let start_time = std::time::SystemTime::now();
        loop {
            if start_time.elapsed().unwrap().as_secs() > 5 {
                return Err("Failed command execution: didn't receive response before timeout".to_string());
            }

            let received_packet = self.receive_packet(&mut tcp_stream).await;
            if received_packet.get_body() == "CsctrlTerminatingRconCommand\n" {
                tcp_stream.shutdown().await.expect("Failed to shutdown TCP stream after sending a packet");
                return Ok(response_body);
            }

            response_body += received_packet.get_body();
        }
    }

    async fn send_packet(&mut self, tcp_stream: &mut TcpStream, packet_type: RconPacketType, body: &str) -> i32 {
        let id = self.get_new_packet_id();
        let packet = RconPacket::new(id, packet_type, body.to_string());
        let serialized_packet = packet.serialize();

        match tcp_stream.write_all(&serialized_packet).await {
            Ok(_) => { }
            Err(error) => {
                tracing::error!("Can't send packet. Error: {}", error);
                return -999;
            }
        };

        return id;
    }

    async fn receive_packet(&mut self, tcp_stream: &mut TcpStream) -> RconPacket {
        return RconPacket::deserialize(tcp_stream).await;
    }

    fn get_new_packet_id(&mut self) -> i32 {
        let int_overflowed = self.current_packet_id.checked_add(1).is_none();
        if int_overflowed { self.current_packet_id = 0; }
        else { self.current_packet_id += 1; }
        return self.current_packet_id;
    }

    pub fn get_is_valid(&self) -> bool { return self.is_valid; }
}
