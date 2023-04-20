use {parquet_derive::ParquetRecordWriter, serde::Serialize, std::sync::Arc};

#[derive(Debug, Clone, Serialize, ParquetRecordWriter)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    pub region: Option<Arc<str>>,
    pub country: Option<Arc<str>>,
    pub continent: Option<Arc<str>>,
    pub project_id: Arc<str>,
    pub client_id: Arc<str>,
    pub push_provider: Arc<str>,
    pub registered_at: chrono::NaiveDateTime,
}
