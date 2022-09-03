use list_reverser::{
    list_reverser_server::{ListReverser, ListReverserServer},
    ReverseListRequest, ReverseListResponse,
};
use tonic::{transport::Server, Request, Response, Status};

use clap::Parser;
use config::Config;
use env_logger::Env;
use serde::Deserialize;

pub mod list_reverser {
    tonic::include_proto!("list_reverser");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("list_reverser_descriptor");
}

#[derive(Default)]
pub struct ListReverserImpl {}

#[tonic::async_trait]
impl ListReverser for ListReverserImpl {
    async fn reverse_list(
        &self,
        request: Request<ReverseListRequest>,
    ) -> Result<Response<ReverseListResponse>, Status> {
        let request_list = request.into_inner().list_elements;
        log::info!("get_list_elements({:?})", request_list);
        let reply = ReverseListResponse {
            list_elements: request_list.into_iter().rev().collect(),
        };
        Ok(Response::new(reply))
    }
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    config_file: String,
}

#[derive(Deserialize)]
struct MyConfig {
    endpoint: String,
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config = Config::builder()
        .add_source(config::File::with_name(&args.config_file))
        .build()?;
    let config: MyConfig = config.try_deserialize()?;
    let env = Env::default().filter_or("eeee5b1e-2b25-11ed-a261-0242ac120002", config.log_level);
    env_logger::init_from_env(env);
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(list_reverser::FILE_DESCRIPTOR_SET)
        .build()?;
    let addr = config.endpoint.parse()?;
    let list_service = ListReverserImpl::default();

    Server::builder()
        .accept_http1(true)
        .add_service(reflection_service)
        .add_service(tonic_web::enable(ListReverserServer::new(list_service)))
        .serve(addr)
        .await?;

    Ok(())
}
