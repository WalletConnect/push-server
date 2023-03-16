use {
    parquet_derive::ParquetRecordWriter,
    serde::Serialize,
    std::sync::Arc,
};

#[derive(Debug, Clone, Serialize, ParquetRecordWriter)]
#[serde(rename_all = "camelCase")]
pub struct MessageInfo {
    pub region: Option<Arc<str>>,
    pub country: Option<Arc<str>>,
    pub continent: Option<Arc<str>>,
}
