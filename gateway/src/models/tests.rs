#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::models::packet::{Data, Hello, Packet};

    #[test]
    fn packet_deserialize() {
        let json = json!({
            "op": 10,
            "d": {"heartbeat_interval": 100},
        });

        let test: Packet = serde_json::from_str(&json.to_string()).unwrap();

        let serialized = test.to_json().unwrap();

        println!("{}", serialized);

        assert_eq!(
            test,
            Packet {
                op: 10,
                d: Data::Hello(Hello {
                    heartbeat_interval: 100,
                }),
            }
        );
    }
}
