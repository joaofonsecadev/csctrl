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
    }

    fn get_new_packet_id(&mut self) -> i32 {
        let int_overflowed = self.current_packet_id.checked_add(1).is_none();
        if int_overflowed { self.current_packet_id = 0; }
        else { self.current_packet_id += 1; }
        return self.current_packet_id;
    }

    pub fn get_is_valid(&self) -> bool { return self.is_valid; }
}