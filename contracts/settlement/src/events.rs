use ink::prelude::vec::Vec;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

use crate::settlement::{NettingEntry, SettlementType, TradeStatus};

use super::AccountId;

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[ink::event]
pub struct TradeSubmitted {
    #[ink(topic)]
    pub trade_id: u64,
    pub buyer: AccountId,
    pub seller: AccountId,
    pub asset_id: u128,
    pub amount: u128,
    pub price: u128,
    pub settlement_type: SettlementType,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[ink::event]
pub struct TradeMatched {
    #[ink(topic)]
    pub trade_id: u64,
    #[ink(topic)]
    pub matching_trade_id: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[ink::event]
pub struct SettlementWindowCreated {
    #[ink(topic)]
    pub window_id: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub settlement_type: SettlementType,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[ink::event]
pub struct SettlementWindowClosed {
    #[ink(topic)]
    pub window_id: u64,
    pub total_trades: u32,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[ink::event]
pub struct NettingCompleted {
    #[ink(topic)]
    pub window_id: u64,
    pub netting_entries: Vec<NettingEntry>,
    pub total_netted_volume: u128,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[ink::event]
pub struct TradeSettled {
    #[ink(topic)]
    pub trade_id: u64,
    #[ink(topic)]
    pub buyer: AccountId,
    #[ink(topic)]
    pub seller: AccountId,
    pub asset_id: u128,
    pub amount: u128,
    pub total_payment: u128,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[ink::event]
pub struct SettlementFailed {
    #[ink(topic)]
    pub trade_id: u64,
    pub fail_reason: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[ink::event]
pub struct SettlementReportGenerated {
    #[ink(topic)]
    pub report_id: u64,
    #[ink(topic)]
    pub window_id: u64,
    pub total_trades: u32,
    pub settled_trades: u32,
    pub failed_trades: u32,
    pub total_volume: u128,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[ink::event]
pub struct ContractPaused {
    #[ink(topic)]
    pub account: AccountId,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[ink::event]
pub struct ContractUnpaused {
    #[ink(topic)]
    pub account: AccountId,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[ink::event]
pub struct TradeCancelled {
    #[ink(topic)]
    pub trade_id: u64,
    pub cancelled_by: AccountId,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[ink::event]
pub struct TradeRetried {
    #[ink(topic)]
    pub trade_id: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[ink::event]
pub struct DVPExecutionStarted {
    #[ink(topic)]
    pub trade_id: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[ink::event]
pub struct DVPExecutionCompleted {
    #[ink(topic)]
    pub trade_id: u64,
    pub asset_transfer_success: bool,
    pub payment_transfer_success: bool,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[ink::event]
pub struct PaymentVerified {
    #[ink(topic)]
    pub account: AccountId,
    pub amount: u128,
    pub verification_success: bool,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[ink::event]
pub struct AssetsVerified {
    #[ink(topic)]
    pub account: AccountId,
    pub asset_id: u128,
    pub amount: u128,
    pub verification_success: bool,
    pub timestamp: u64,
}