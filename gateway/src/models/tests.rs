#[cfg(test)]
mod test {
    use serde::Deserialize;
    use serde_json::json;

    use crate::models::packet::{Hello, Packet};

    #[test]
    fn packet_deserialize() {
        let json = json!({
            "op": 10,
            "d": {"heartbeat_interval": 0}
        });

        let packet = serde_json::from_str::<Packet>(&json.to_string()).unwrap();

        println!("{:?}", packet);

        assert_eq!(
            packet,
            Packet::Hello(Hello {
                heartbeat_interval: 0
            })
        )
    }
}
