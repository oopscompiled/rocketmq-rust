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

    #[test]
    fn test_default_values() {
        let body = QueryAssignmentRequestBody::default();

        assert!(body.topic.is_empty());
        assert!(body.consumer_group.is_empty());
    }

    #[test]
    fn test_serialization_camel_case() {
        let body = QueryAssignmentRequestBody {
            topic: CheetahString::from("test-topic"),
            consumer_group: CheetahString::from("group-a"),
            client_id: CheetahString::from("client-123"),
            strategy_name: CheetahString::from("avg"),
            message_model: MessageModel::Clustering,
        };

        let json = serde_json::to_string(&body).unwrap();

        assert!(json.contains("\"topic\""));
        assert!(json.contains("\"consumerGroup\""));
        assert!(json.contains("\"clientId\""));
        assert!(json.contains("\"strategyName\""));
        assert!(json.contains("\"messageModel\""));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{
            "topic": "rocketmq-rust",
            "consumerGroup": "test-group",
            "clientId": "node-1",
            "strategyName": "round-robin",
            "messageModel": "MESSAGE"
        }"#;

        let decoded: QueryAssignmentRequestBody = serde_json::from_str(json).expect("Should deserialize");

        assert_eq!(decoded.topic.as_str(), "rocketmq-rust");
        assert_eq!(decoded.consumer_group.as_str(), "test-group");
        assert_eq!(decoded.client_id.as_str(), "node-1");
    }

    #[test]
    fn test_cheetah_string_from_string() {
        let raw = String::from("performance_matters");
        let cheetah = CheetahString::from(raw.clone());

        assert_eq!(cheetah.as_str(), raw.as_str());
        assert_eq!(cheetah.len(), raw.len());
    }
}
