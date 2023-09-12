pub fn exec(sql: String) -> Result<(), TsdbError> {
    tracing::info!("Exec sql: {}", &sql);
    Err(TsdbError::NotImplError)
}

#[derive(thiserror::Error, Debug)]
pub enum TsdbError {
    #[error("Not implement yet!")]
    NotImplError,
}
