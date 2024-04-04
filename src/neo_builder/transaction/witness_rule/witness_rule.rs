use serde::{Deserialize, Serialize};

use neo::prelude::*;

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Clone)]
pub struct WitnessRule {
	#[serde(rename = "action")]
	pub action: WitnessAction,
	#[serde(rename = "condition")]
	pub condition: WitnessCondition,
}

impl WitnessRule {
	pub fn new(action: WitnessAction, condition: WitnessCondition) -> Self {
		Self { action, condition }
	}
}

impl NeoSerializable for WitnessRule {
	type Error = TransactionError;

	fn size(&self) -> usize {
		1 + self.condition.size()
	}

	fn encode(&self, writer: &mut Encoder) {
		writer.write_u8(self.action as u8);
		writer.write_serializable_fixed(&self.condition);
	}

	fn decode(reader: &mut Decoder) -> Result<Self, Self::Error> {
		let action = reader.read_u8();
		let condition = WitnessCondition::decode(reader)?;
		Ok(Self { action: WitnessAction::try_from(action).unwrap(), condition })
	}
	fn to_array(&self) -> Vec<u8> {
		let mut writer = Encoder::new();
		self.encode(&mut writer);
		writer.to_bytes()
	}
}

#[cfg(test)]
mod tests {
	use primitive_types::H160;

	use neo::prelude::*;

	#[test]
	fn test_decode_boolean_condition() {
		let json = r#"{"action": "Allow","condition": {"type": "Boolean","expression": "false"}}"#;
		let rule: WitnessRule = serde_json::from_str(json).unwrap();
		assert!(matches!(rule.condition, WitnessCondition::Boolean(_)));
		assert!(!rule.condition.boolean_expression().unwrap());
	}

	#[test]
	fn test_script_hash_condition_serialize_deserialize() {
		let hash = TestConstants::DEFAULT_ACCOUNT_SCRIPT_HASH;
		let condition = WitnessCondition::ScriptHash(H160::from_hex(hash).unwrap());

		let bytes = hex::decode(format!("18{}", hash)).unwrap();

		let deserialized = WitnessCondition::from_bytes(&bytes).unwrap();
		assert_eq!(condition, deserialized);

		let mut writer = Encoder::new();
		condition.encode(&mut writer);
		assert_eq!(bytes, writer.to_bytes());
	}

	#[test]
	fn test_decode_not_condition() {
		let json = r#"{
        "action": "Allow",
        "condition": {
            "type": "Not",
            "expression": {
                "type": "Not",
                "expression": {
                    "type": "CalledByEntry"
                }
            }
        }
    }"#;

		let rule: WitnessRule = serde_json::from_str(json).unwrap();

		assert!(matches!(
			rule.condition,
			WitnessCondition::Not(boxed) if matches!(*boxed, WitnessCondition::Not(_))
		));
	}

	#[test]
	fn test_and_condition_serialize_deserialize() {
		let condition = WitnessCondition::And(vec![
			WitnessCondition::Boolean(true),
			WitnessCondition::Boolean(false),
		]);

		let bytes = hex::decode("020200010000").unwrap();

		let deserialized = WitnessCondition::from_bytes(&bytes).unwrap();
		assert_eq!(condition, deserialized);

		let mut writer = Encoder::new();
		condition.encode(&mut writer);

		assert_eq!(bytes, writer.to_bytes());
	}

	#[test]
	fn test_boolean_nil_values() {
		let json = r#"{
        "action": "Deny",
        "condition": {
            "type": "CalledByGroup",
            "group": "035a1ced7ae274a881c3f479452c8bca774c89f653d54c5c5959a01371a8c696fd"
        }
    }"#;

		let rule: WitnessRule = serde_json::from_str(json).unwrap();

		assert!(rule.condition.boolean_expression().is_none());
		assert!(rule.condition.expression().is_none());
	}

	#[test]
	fn test_decode_or_condition() {
		let json = r#"{
        "action": "Deny",
        "condition": {
            "type": "Or",
            "expressions": [
                {"type": "Group", "group": "023be7b6742268f4faca4835718f3232ddc976855d5ef273524cea36f0e8d102f3"},
                {"type": "CalledByEntry"}
            ]
        }
    }"#;

		let rule: WitnessRule = serde_json::from_str(json).unwrap();

		assert!(matches!(
			rule.condition,
			WitnessCondition::Or(conditions) if conditions.len() == 2
		));
	}

	#[test]
	fn test_called_by_group_condition_serialize_deserialize() {
		let key = TestConstants::DEFAULT_ACCOUNT_PUBLIC_KEY;
		let condition =
			WitnessCondition::CalledByGroup(Secp256r1PublicKey::from_encoded(&key).unwrap());

		let bytes = hex::decode(format!("29{}", key)).unwrap();

		let deserialized = WitnessCondition::from_bytes(&bytes).unwrap();
		assert_eq!(condition, deserialized);

		let mut writer = Encoder::new();
		condition.encode(&mut writer);

		assert_eq!(bytes, writer.to_bytes());
	}

	#[test]
	fn test_called_by_entry_serialize_deserialize() {
		let condition = WitnessCondition::CalledByEntry;

		let bytes = hex::decode("20").unwrap();

		let deserialized = WitnessCondition::from_bytes(&bytes).unwrap();
		assert_eq!(condition, deserialized);

		let mut writer = Encoder::new();
		condition.encode(&mut writer);

		assert_eq!(bytes, writer.to_bytes());
	}

	#[test]
	fn test_called_by_contract_serialize_deserialize() {
		let hash = TestConstants::DEFAULT_ACCOUNT_SCRIPT_HASH;
		let condition = WitnessCondition::CalledByContract(H160::from_hex(&hash).unwrap());

		let bytes = hex::decode(format!("28{}", hash)).unwrap();

		let deserialized = WitnessCondition::from_bytes(&bytes).unwrap();
		assert_eq!(condition, deserialized);

		let mut writer = Encoder::new();
		condition.encode(&mut writer);

		assert_eq!(bytes, writer.to_bytes());
	}

	#[test]
	fn test_decode_script_hash_condition() {
		let json = r#"{
        "action": "Allow",
        "condition": {
            "type": "ScriptHash",
            "hash": "ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5"
        }
    }"#;

		let rule: WitnessRule = serde_json::from_str(json).unwrap();

		assert!(matches!(rule.condition, WitnessCondition::ScriptHash(_)));
	}

	#[test]
	fn test_decode_group_condition() {
		let json = r#"{
        "action": "Allow",
        "condition": {
            "type": "Group",
            "group": "0352321377ac7b4e1c4c2ebfe28f4d82fa3c213f7ccfcc9dac62da37fb9b433f0c"
        }
    }"#;

		let rule: WitnessRule = serde_json::from_str(json).unwrap();

		assert!(matches!(rule.condition, WitnessCondition::Group(_),));
	}

	#[test]
	fn test_decode_called_by_entry_condition() {
		let json = r#"{
        "action": "Deny",
        "condition": {
            "type": "CalledByEntry"
        }
    }"#;

		let rule: WitnessRule = serde_json::from_str(json).unwrap();

		assert_eq!(rule.condition, WitnessCondition::CalledByEntry,);
	}

	#[test]
	fn test_decode_called_by_contract_condition() {
		let json = r#"{
        "action": "Allow",
        "condition": {
            "type": "CalledByContract",
            "hash": "ef4073a0f2b305a38ec4050e4d3d28bc40ea63e4"
        }
    }"#;

		let rule: WitnessRule = serde_json::from_str(json).unwrap();

		assert!(matches!(rule.condition, WitnessCondition::CalledByContract(_),));
	}

	#[test]
	fn test_and_condition_decode() {
		let json = r#"{
        "action": "Allow",
        "condition": {
            "type": "And",
            "expressions": [
                {"type": "CalledByEntry"},
                {"type": "Group", "group": "021821807f923a3da004fb73871509d7635bcc05f41edef2a3ca5c941d8bbc1231"},
                {"type": "Boolean", "expression": "true"}
            ]
        }
    }"#;

		let rule: WitnessRule = serde_json::from_str(json).unwrap();

		assert!(matches!(
			rule.condition,
			WitnessCondition::And(expressions) if expressions.len() == 3
		));
	}

	#[test]
	fn test_not_condition_decode() {
		let json = r#"{
        "action": "Allow",
        "condition": {
            "type": "Not",
            "expression": {
                "type": "CalledByEntry"
            }
        }
    }"#;

		let rule: WitnessRule = serde_json::from_str(json).unwrap();

		let bo = Box::new(WitnessCondition::CalledByEntry);
		assert!(matches!(rule.condition, WitnessCondition::Not(bo)));
	}

	#[test]
	fn boolean_expression() {
		let json = r#"{
        "condition": {
            "type": "Boolean",
            "expression": "false"
        }
    }"#;

		let condition = parse_condition(json);
		assert!(!condition.boolean_expression().unwrap());
	}

	fn parse_condition(_: &str) -> WitnessCondition {
		WitnessCondition::Boolean(false)
	}
}
