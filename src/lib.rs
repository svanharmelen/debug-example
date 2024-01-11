include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

#[cfg(test)]
mod test {
    use crate::example::{test_message::Value, TestMessage};

    #[test]
    fn test() {
        let json = r#"{
          "name": "test",
          "value": 10.3
        }"#;

        let message: TestMessage = serde_json::from_str(json).expect("failed to parse JSON");

        assert_eq!(message.name, Some("test".to_string()));
        assert_eq!(message.value, Some(Value::FloatValue(10.3)));
    }
}
