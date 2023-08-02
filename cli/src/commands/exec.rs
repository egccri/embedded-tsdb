use crate::api;
use tonic::transport::Channel;

/// Exec command
#[derive(clap::Parser, Debug)]
pub struct ExecCommand {
    /// exec sql
    sql: String,
}

impl ExecCommand {
    pub async fn execute(&self, channel: Channel) -> anyhow::Result<()> {
        tracing::info!("Exec sql: {}", &self.sql);
        ExecClient::exec_sql(self.sql.clone(), channel).await?;
        Ok(())
    }
}

pub struct ExecClient;

impl ExecClient {
    pub async fn exec_sql(sql: String, channel: Channel) -> anyhow::Result<()> {
        let mut client = api::exec::exec_client::ExecClient::new(channel);
        let request = api::exec::ExecRequest { sql };
        let _ = client.exec(tonic::Request::new(request)).await;
        // FIXME return result
        Ok(())
    }
}
