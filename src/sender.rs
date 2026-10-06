use async_trait::async_trait;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;

pub type DynMtbFileSender = Arc<dyn MtbFileSender + Send + Sync>;

use crate::AppResponse;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait MtbFileSender {
    async fn send(&self, json: &str) -> Result<(), AppResponse>;
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct DefaultMtbFileSender {
    topic: String,
    producer: FutureProducer,
}

impl DefaultMtbFileSender {
    pub fn new(topic: &str, producer: FutureProducer) -> Self {
        Self {
            topic: topic.to_string(),
            producer,
        }
    }
}

#[async_trait]
impl MtbFileSender for DefaultMtbFileSender {
    async fn send(&self, json: &str) -> Result<(), AppResponse> {
        #[derive(Deserialize, Clone)]
        struct BasicJson {
            report_fields: ReportFields,
        }

        #[derive(Deserialize, Clone)]
        struct ReportFields {
            #[serde(rename = "Hnummer")]
            hnummer: String,

            #[serde(rename = "PID")]
            pid: String,
        }

        let basic_json =
            serde_json::from_str::<BasicJson>(json).map_err(|_| AppResponse::BadRequest)?;

        let record_key = format!(
            "{}_PID{}",
            basic_json.report_fields.hnummer, basic_json.report_fields.pid
        );

        self.producer
            .send(
                FutureRecord::to(&self.topic)
                    .key(&record_key.to_string())
                    .payload(json),
                Duration::from_secs(1),
            )
            .await
            .map_err(|_| AppResponse::InternalServerError)
            .map(|_| ())?;

        Ok(())
    }
}
