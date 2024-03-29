use {parquet_derive::ParquetRecordWriter, serde::Serialize, std::sync::Arc};

#[derive(Debug, Clone, Serialize, ParquetRecordWriter)]
#[serde(rename_all = "camelCase")]
pub struct MessageInfo {
    pub msg_id: Arc<str>,
    pub region: Option<Arc<str>>,
    pub country: Option<Arc<str>>,
    pub continent: Option<Arc<str>>,
    pub project_id: Arc<str>,
    pub client_id: Arc<str>,
    pub topic: Arc<str>,
    pub push_provider: Arc<str>,
    pub always_raw: Option<bool>,
    pub tag: Option<u32>,
    pub encrypted: Option<bool>,
    pub flags: Option<u32>,
    pub status: u16,
    pub response_message: Option<Arc<str>>,
    pub received_at: chrono::NaiveDateTime,
}
