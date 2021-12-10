use serde_json::Value as JSONValue;

use crate::web3::service::Service as Web3Service;
use crate::precompiled::precompiled_service::{
    PrecompiledServiceError,
    call,
    send_transaction,
    parse_output,
    parse_string_token_to_json,
};

const ADDRESS: &str = "0x0000000000000000000000000000000000001008";
const ABI_CONTENT: &str = r#"[{"constant":true,"inputs":[],"name":"listOperators","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"user","type":"address"},{"name":"weight","type":"int256"}],"name":"updateCommitteeMemberWeight","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"queryThreshold","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"name":"member","type":"address"}],"name":"queryVotesOfMember","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"name":"user","type":"address"}],"name":"queryCommitteeMemberWeight","outputs":[{"name":"","type":"bool"},{"name":"","type":"int256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"user","type":"address"}],"name":"grantCommitteeMember","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"account","type":"address"}],"name":"unfreezeAccount","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"queryVotesOfThreshold","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"listCommitteeMembers","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"threshold","type":"int256"}],"name":"updateThreshold","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"user","type":"address"}],"name":"revokeCommitteeMember","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"user","type":"address"}],"name":"grantOperator","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"account","type":"address"}],"name":"freezeAccount","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"user","type":"address"}],"name":"revokeOperator","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"account","type":"address"}],"name":"getAccountStatus","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"}]"#;

pub struct ChainGovernanceService<'a> {
    web3_service: &'a Web3Service,
}

impl ChainGovernanceService<'_> {
    pub fn new(web3_service: &Web3Service) -> ChainGovernanceService {
        ChainGovernanceService {
            web3_service
        }
    }

    pub async fn grant_committee_member(&self, user_address: &str) -> Result<i32, PrecompiledServiceError> {
        let params = vec![user_address.to_owned()];
        send_transaction(
            self.web3_service,
            "ChainGovernancePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "grantCommitteeMember",
            &params
        ).await
    }

    pub async fn revoke_committee_member(&self, user_address: &str) -> Result<i32, PrecompiledServiceError> {
        let params = vec![user_address.to_owned()];
        send_transaction(
            self.web3_service,
            "ChainGovernancePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "revokeCommitteeMember",
            &params
        ).await
    }

    pub async fn list_committee_members(&self) -> Result<JSONValue, PrecompiledServiceError> {
        let params = vec![];
        let response = call(
            self.web3_service,
            "ChainGovernancePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "listCommitteeMembers",
            &params
        ).await?;
        parse_string_token_to_json(&response.output)
    }

    pub async fn query_committee_member_weight(&self, user_address: &str) -> Result<(bool, i32), PrecompiledServiceError> {
        let params = vec![user_address.to_owned()];
        let response = call(
            self.web3_service,
            "ChainGovernancePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "queryCommitteeMemberWeight",
            &params
        ).await?;
        let tokens = response.output.unwrap();
        let status = tokens[0].clone().into_bool().unwrap();
        let code = parse_output(&tokens[1].clone().into_int().unwrap())?;
        Ok((status , code))
    }

    pub async fn update_committee_member_weight(&self, user_address: &str, weight: i32) -> Result<i32, PrecompiledServiceError> {
        let params = vec![user_address.to_owned(), weight.to_string()];
        send_transaction(
            self.web3_service,
            "ChainGovernancePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "updateCommitteeMemberWeight",
            &params
        ).await
    }

    pub async fn query_votes_of_member(&self, user_address: &str) -> Result<JSONValue, PrecompiledServiceError> {
        let params = vec![user_address.to_owned()];
        let response = call(
            self.web3_service,
            "ChainGovernancePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "queryVotesOfMember",
            &params
        ).await?;
        parse_string_token_to_json(&response.output)
    }

    pub async fn query_votes_of_threshold(&self) -> Result<JSONValue, PrecompiledServiceError> {
        let params = vec![];
        let response = call(
            self.web3_service,
            "ChainGovernancePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "queryVotesOfThreshold",
            &params
        ).await?;
        parse_string_token_to_json(&response.output)
    }

    pub async fn update_threshold(&self, threshold: i32) -> Result<i32, PrecompiledServiceError> {
        let params = vec![threshold.to_string()];
        send_transaction(
            self.web3_service,
            "ChainGovernancePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "updateThreshold",
            &params
        ).await
    }

    pub async fn query_threshold(&self) -> Result<i32, PrecompiledServiceError> {
        let params = vec![];
        let response = call(
            self.web3_service,
            "ChainGovernancePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "queryThreshold",
            &params
        ).await?;
        let tokens = response.output.unwrap();
        parse_output(&tokens[0].clone().into_int().unwrap())
    }

    pub async fn grant_operator(&self, user_address: &str) -> Result<i32, PrecompiledServiceError> {
        let params = vec![user_address.to_owned()];
        send_transaction(
            self.web3_service,
            "ChainGovernancePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "grantOperator",
            &params
        ).await
    }

    pub async fn revoke_operator(&self, user_address: &str) -> Result<i32, PrecompiledServiceError> {
        let params = vec![user_address.to_owned()];
        send_transaction(
            self.web3_service,
            "ChainGovernancePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "revokeOperator",
            &params
        ).await
    }

    pub async fn list_operators(&self) -> Result<JSONValue, PrecompiledServiceError> {
        let params = vec![];
        let response = call(
            self.web3_service,
            "ChainGovernancePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "listOperators",
            &params
        ).await?;
        parse_string_token_to_json(&response.output)
    }

    pub async fn freeze_account(&self, address: &str) -> Result<i32, PrecompiledServiceError> {
        let params = vec![address.to_owned()];
        send_transaction(
            self.web3_service,
            "ChainGovernancePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "freezeAccount",
            &params
        ).await
    }

    pub async fn unfreeze_account(&self, address: &str) -> Result<i32, PrecompiledServiceError> {
        let params = vec![address.to_owned()];
        send_transaction(
            self.web3_service,
            "ChainGovernancePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "unfreezeAccount",
            &params
        ).await
    }

    pub async fn get_account_status(&self, address: &str) -> Result<String, PrecompiledServiceError> {
        let params = vec![address.to_owned()];
        let response = call(
            self.web3_service,
            "ChainGovernancePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "getAccountStatus",
            &params
        ).await?;
        let tokens = response.output.unwrap();
        Ok(tokens[0].clone().into_string().unwrap_or(String::from("")))
    }
}