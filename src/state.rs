use std::sync::Arc;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use crate::services::{create_option_service::CreateOptionService};
#[derive(Clone)]
pub struct AppState {
    pub create_option_service: Arc<CreateOptionService>,
}
