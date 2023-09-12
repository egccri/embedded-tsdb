use cli::api::exec::{ExecReply, ExecRequest};
use tonic::{Request, Response, Status};

pub struct ExecSvc {}

impl ExecSvc {
    pub fn new() -> ExecSvc {
        ExecSvc {}
    }
}

#[tonic::async_trait]
impl cli::api::exec::exec_server::Exec for ExecSvc {
    async fn exec(&self, request: Request<ExecRequest>) -> Result<Response<ExecReply>, Status> {
        let result = embedded_tsdb::exec(request.into_inner().sql);
        println!("{:?}", result);
        Err(Status::ok("Not implement yet!"))
    }
}
