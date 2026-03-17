// Copyright 2023 The RocketMQ Rust Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cheetah_string::CheetahString;
use serde::Deserialize;
use serde::Serialize;

use crate::protocol::heartbeat::message_model::MessageModel;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryAssignmentRequestBody {
    pub topic: CheetahString,
    pub consumer_group: CheetahString,
    pub client_id: CheetahString,
    pub strategy_name: CheetahString,
    pub message_model: MessageModel,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_full_request() -> QueryAssignmentRequestBody {
        QueryAssignmentRequestBody {
            topic: CheetahString::from("test-topic"),
            consumer_group: CheetahString::from("group-a"),
            client_id: CheetahString::from("client-123"),
            strategy_name: CheetahString::from("avg"),
            message_model: MessageModel::Clustering,
        }
    }

    #[test]
    fn test_default_values() {
        let body = QueryAssignmentRequestBody::default();
        assert!(body.topic.is_empty());
        assert!(body.consumer_group.is_empty());
        assert!(body.client_id.is_empty());
        assert!(body.strategy_name.is_empty());
    }

    #[test]
    fn test_clone_preservation() {
        let original = create_full_request();
        let cloned = original.clone();

        assert_eq!(original.topic, cloned.topic);
        assert_eq!(original.consumer_group, cloned.consumer_group);
        assert_eq!(original.message_model, cloned.message_model);
    }

    #[test]
    fn test_debug_output() {
        let body = create_full_request();
        let debug_str = format!("{:?}", body);

        assert!(debug_str.contains("QueryAssignmentRequestBody"));
        assert!(debug_str.contains("test-topic"));
    }

    #[test]
    fn test_serde_roundtrip() {
        let original = create_full_request();

        let json = serde_json::to_string(&original).expect("Failed to serialize");
        let recovered: QueryAssignmentRequestBody = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(original.topic, recovered.topic);
        assert_eq!(original.strategy_name, recovered.strategy_name);
        assert_eq!(original.message_model, recovered.message_model);
    }

    #[test]
    fn test_deserialization_with_valid_enum() {
        let json = r#"{
            "topic": "rocketmq-rust",
            "consumerGroup": "test-group",
            "clientId": "node-1",
            "strategyName": "round-robin",
            "messageModel": "CLUSTERING"
        }"#;

        let decoded: QueryAssignmentRequestBody = serde_json::from_str(json).expect("Should deserialize");

        assert_eq!(decoded.topic.as_str(), "rocketmq-rust");
        assert_eq!(decoded.consumer_group.as_str(), "test-group");
        assert_eq!(decoded.client_id.as_str(), "node-1");
        assert_eq!(decoded.strategy_name.as_str(), "round-robin");
        assert_eq!(decoded.message_model, MessageModel::Clustering);
    }

    #[test]
    fn test_special_characters_and_empty_fields() {
        let body = QueryAssignmentRequestBody {
            topic: CheetahString::from(""),
            consumer_group: CheetahString::from("!@#$%^&*()"),
            client_id: CheetahString::from("   "),
            strategy_name: CheetahString::from("🚀-strategy"),
            message_model: MessageModel::Broadcasting,
        };

        let json = serde_json::to_string(&body).unwrap();
        let decoded: QueryAssignmentRequestBody = serde_json::from_str(&json).unwrap();

        assert_eq!(decoded.topic.as_str(), "");
        assert_eq!(decoded.consumer_group.as_str(), "!@#$%^&*()");
        assert_eq!(decoded.strategy_name.as_str(), "🚀-strategy");
        assert_eq!(decoded.message_model, MessageModel::Broadcasting);
    }

    #[test]
    fn test_serialization_camel_case_consistency() {
        let body = create_full_request();
        let json_value: serde_json::Value = serde_json::to_value(&body).unwrap();

        assert!(json_value.get("consumerGroup").is_some());
        assert!(json_value.get("strategyName").is_some());
        assert!(json_value.get("messageModel").is_some());
        assert!(json_value.get("consumer_group").is_none());
    }
}
