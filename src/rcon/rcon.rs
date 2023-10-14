pub struct RconConnection {
    address: String,
    password: String,
    current_packet_id: i32,
    is_valid: bool,
    tcp_stream: std::sync::OnceLock<tokio::net::TcpStream>,
}

impl RconConnection {
    pub fn create_rcon_connection(address: &str, password: &str) -> RconConnection {
        let rcon = RconConnection {
            address: address.to_string(),
            password: password.to_string(),
            current_packet_id: -1,
            is_valid: false,
            tcp_stream: std::sync::OnceLock::new(),
        };

        return rcon;
    }

    async fn init_rcon_connection(&mut self) {

    }

    fn get_new_packet_id(&mut self) -> i32 {
        let int_overflowed = self.current_packet_id.checked_add(1).is_none();
        if int_overflowed { self.current_packet_id = 0; }
        else { self.current_packet_id += 1; }
        return self.current_packet_id;
    }
}