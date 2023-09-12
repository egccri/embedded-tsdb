use crate::service::exec_service::ExecSvc;
use cli::api::exec::exec_server::ExecServer;
use tonic::transport::Server;

pub async fn start(server_addr: String) {
    tracing::info!("Start tsdb server with address: {}.", &server_addr);
    let addr = server_addr.parse().unwrap();
    let exec_svc = ExecSvc::new();
    Server::builder()
        .add_service(ExecServer::new(exec_svc))
        .serve(addr)
        .await
        .expect("Tsdb server start failure!");
}
