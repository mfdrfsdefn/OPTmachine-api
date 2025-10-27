use crate::clients::swap::*;
use crate::dto::add_liquidity::PoolAccount;
use crate::dto::swap::{SwapRequest, SwapResponse};
use crate::utils::to_pubkey::to_pubkey;
use crate::utils::sdk_instructions::to_sdk_instruction;
use anyhow::Result;
use bincode::config::standard;
use bincode::serde::encode_to_vec;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    hash::Hash, instruction::Instruction, pubkey::Pubkey, signature::Keypair,
    transaction::Transaction,
};
use spl_associated_token_account::{get_associated_token_address, instruction::create_associated_token_account};
use spl_token::{state::Mint, ID as TOKEN_PROGRAM_ID};
use spl_token::solana_program::program_pack::Pack;
pub struct SwapService {
    rpc: RpcClient,
    program_id: Pubkey,
}

impl SwapService {
    pub fn new(rpc_url: &str, program_id: Pubkey) -> Self {
        let rpc = RpcClient::new(rpc_url.to_string());
        Self { rpc, program_id }
    }

    pub async fn build_swap_tx(&self, req: SwapRequest) -> Result<SwapResponse> {
        let provider = req.provider;
        let pool = req.pool;

        // --- 读取池账户数据 ---
        let pool_account_raw = self.rpc.get_account(&pool).await?;
        let pool_account_data = pool_account_raw.data;
        let pool_account = PoolAccount::try_from_slice(&pool_account_data[8..]).unwrap();

        // --- 解析各种 pubkey ---
        let provider_pubkey = to_pubkey(provider.to_bytes());
        let pool_account_pubkey = to_pubkey(pool.to_bytes());
        let mint_a_pubkey = to_pubkey(pool_account.mint_a.to_bytes());
        let mint_b_pubkey = to_pubkey(pool_account.mint_b.to_bytes());
        let vault_a_ata = get_associated_token_address(&pool_account_pubkey, &mint_a_pubkey);
        let vault_b_ata = get_associated_token_address(&pool_account_pubkey, &mint_b_pubkey);
        let provider_a_ata = get_associated_token_address(&provider_pubkey, &mint_a_pubkey);
        let provider_b_ata = get_associated_token_address(&provider_pubkey, &mint_b_pubkey);

        let vault_a = Pubkey::new_from_array(vault_a_ata.to_bytes());
        let vault_b = Pubkey::new_from_array(vault_b_ata.to_bytes());
        let provider_token_a = Pubkey::new_from_array(provider_a_ata.to_bytes());
        let provider_token_b = Pubkey::new_from_array(provider_b_ata.to_bytes());

        // --- 根据 swap 方向决定输入 mint ---
        let mint_in = if req.a_to_b {
            pool_account.mint_a
        } else {
            pool_account.mint_b
        };

        // --- 读取 mint decimals ---
        let mint_in_acc = self.rpc.get_account(&mint_in).await?;
        let mint_in_data = Mint::unpack(&mint_in_acc.data)?;
        let decimals_in = mint_in_data.decimals;

        // --- 将人类可读数值换算为最小单位 ---
        let real_amount_in = req
            .amount_in
            .checked_mul(10u64.pow(decimals_in as u32))
            .ok_or_else(|| anyhow::anyhow!("Overflow when scaling amount_in"))?;

        // --- 预创建 ATA ---
        let mut ixs = vec![];
        if self.rpc.get_account(&provider_token_a).await.is_err() {
            let ix = create_associated_token_account(
                &provider_pubkey,
                &provider_pubkey,
                &mint_a_pubkey,
                &TOKEN_PROGRAM_ID,
            );
            ixs.push(to_sdk_instruction(ix));
        }
        if self.rpc.get_account(&provider_token_b).await.is_err() {
            let ix = create_associated_token_account(
                &provider_pubkey,
                &provider_pubkey,
                &mint_b_pubkey,
                &TOKEN_PROGRAM_ID,
            );
            ixs.push(to_sdk_instruction(ix));
        }

        // --- 构造 swap 指令 ---
        let ix = build_swap_ix(
            self.program_id,
            provider,
            pool,
            vault_a,
            vault_b,
            provider_token_a,
            provider_token_b,
            real_amount_in, // ✅ 使用换算后的真实数量
            req.a_to_b,
        )?;
        ixs.push(ix);

        // --- 打包交易 ---
        let recent_blockhash = self.rpc.get_latest_blockhash().await?;
        let mut tx = Transaction::new_with_payer(&ixs, Some(&provider));
        tx.message.recent_blockhash = recent_blockhash;

        let bytes = encode_to_vec(&tx, standard())?;
        let base64_tx = base64::encode(bytes);

        Ok(SwapResponse { unsigned_tx: base64_tx })
    }
}
