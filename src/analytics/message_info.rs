use {parquet_derive::ParquetRecordWriter, serde::Serialize, std::sync::Arc};

#[derive(Debug, Clone, Serialize, ParquetRecordWriter)]
#[serde(rename_all = "camelCase")]
pub struct MessageInfo {
    pub region: Option<Arc<str>>,
    pub country: Option<Arc<str>>,
    pub continent: Option<Arc<str>>,
    pub project_id: Arc<str>,
    pub client_id: Arc<str>,
    pub topic: Option<Arc<str>>,
    pub push_provider: Arc<str>,
    pub encrypted: bool,
    pub flags: u32,
    pub received_at: chrono::NaiveDateTime,
}
