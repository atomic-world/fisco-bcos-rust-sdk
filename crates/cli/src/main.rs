use fisco_bcos_service::{ ServiceTrait, RPCService };

#[tokio::main]
async fn main() {
    let rpc_service = RPCService::new("http://127.0.0.1:8545");
    let block_number = rpc_service.get_block_number(1).await.unwrap();
    println!("Block Number: {}", block_number);
}