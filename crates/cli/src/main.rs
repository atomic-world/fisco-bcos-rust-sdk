use fisco_bcos_service::create_web3_service;

#[tokio::main]
async fn main() {
    let web3_service = create_web3_service("configs/config.json").unwrap();
    let client_version = web3_service.get_client_version(1).await.unwrap();
    println!("Client Version: {}", client_version);

    let pbft_view = web3_service.get_pbft_view(1).await.unwrap();
    println!("PBFT View: {}", pbft_view);

    let block_number = web3_service.get_block_number(1).await.unwrap();
    println!("Block Number: {:?}", block_number);

    let sealer_list = web3_service.get_sealer_list(1).await.unwrap();
    println!("Sealer List: {:?}", sealer_list);

    let observer_list = web3_service.get_observer_list(1).await.unwrap();
    println!("Observer List: {:?}", observer_list);

    let consensus_status = web3_service.get_consensus_status(1).await.unwrap();
    println!("Consensus Status: {}", consensus_status);

    let sync_status = web3_service.get_sync_status(1).await.unwrap();
    println!("Sync Status: {}", sync_status);

    let peers = web3_service.get_peers(1).await.unwrap();
    println!("Peers: {:?}", peers);

    let group_peers = web3_service.get_group_peers(1).await.unwrap();
    println!("Group Peers: {:?}", group_peers);

    let node_id_list = web3_service.get_node_id_list(1).await.unwrap();
    println!("Node ID List: {:?}", node_id_list);

    let group_list = web3_service.get_group_list().await.unwrap();
    println!("Group List: {:?}", group_list);

    let block_from_number = web3_service.get_block_by_number(
        1,
        &block_number,
        true
    ).await.unwrap();
    println!("Block from number : {:?}", block_from_number);

    let block_hash = block_from_number["hash"].as_str().unwrap();
    let block_from_hash = web3_service.get_block_by_hash(
        1,
        block_hash,
        true
    ).await.unwrap();
    println!("Block from hash : {:?}", block_from_hash);

    let block_header_from_hash = web3_service.get_block_header_by_hash(
        1,
        block_hash,
        true
    ).await.unwrap();
    println!("Block Header from hash: {:?}", block_header_from_hash);

    let block_header_from_number = web3_service.get_block_header_by_number(
        1,
        &block_number,
        true
    ).await.unwrap();
    println!("Block Header from number: {:?}", block_header_from_number);

    let block_hash_from_number = web3_service.get_block_hash_by_number(
        1,
        &block_number
    ).await.unwrap();
    println!("Block Hash from number: {:?}", block_hash_from_number);
}