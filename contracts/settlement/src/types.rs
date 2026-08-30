use ink::prelude::vec::Vec;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use ink::env::AccountId;

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum SettlementType {
    Immediate,
    TPlus1,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum TradeStatus {
    Submitted,
    Matched,
    PendingSettlement,
    Settled,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum SettlementStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Retried,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum SettlementError {
    InsufficientFunds,
    InsufficientAssets,
    DVPFailed,
    TransferFailed,
    VerificationFailed,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Trade {
    pub trade_id: u64,
    pub buyer: AccountId,
    pub seller: AccountId,
    pub asset_id: u128,
    pub amount: u128,
    pub price: u128,
    pub timestamp: u64,
    pub status: TradeStatus,
    pub settlement_type: SettlementType,
    pub settlement_window_id: Option<u64>,
    pub retry_count: u32,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct SettlementWindow {
    pub window_id: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub is_active: bool,
    pub is_processed: bool,
    pub trade_ids: Vec<u64>,
    pub total_volume: u128,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct NettingEntry {
    pub account: AccountId,
    pub asset_id: u128,
    pub net_amount: i128,
    pub net_payment: i128,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct SettlementReport {
    pub report_id: u64,
    pub window_id: u64,
    pub timestamp: u64,
    pub total_trades: u32,
    pub settled_trades: u32,
    pub failed_trades: u32,
    pub total_volume: u128,
    pub net_settled_volume: u128,
    pub netting_entries: Vec<NettingEntry>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct SettlementRecord {
    pub record_id: u64,
    pub trade_id: u64,
    pub account: AccountId,
    pub asset_transferred: u128,
    pub payment_processed: u128,
    pub timestamp: u64,
    pub status: SettlementStatus,
    pub fail_reason: Option<Vec<u8>>,
    pub settlement_window_id: u64,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct TradeMatch {
    pub primary_trade_id: u64,
    pub matching_trade_id: u64,
    pub matched_at: u64,
    pub asset_id: u128,
    pub total_amount: u128,
    pub total_value: u128,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct AccountSettlementSummary {
    pub account: AccountId,
    pub total_trades_submitted: u32,
    pub total_trades_settled: u32,
    pub total_trades_failed: u32,
    pub total_volume_in: u128,
    pub total_volume_out: u128,
    pub net_volume: i128,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct DVPSettlement {
    pub trade_id: u64,
    pub asset_transfer_complete: bool,
    pub payment_transfer_complete: bool,
    pub settlement_complete: bool,
    pub settlement_timestamp: Option<u64>,
}

impl Default for Trade {
    fn default() -> Self {
        Self {
            trade_id: 0,
            buyer: AccountId::from([0u8; 32]),
            seller: AccountId::from([0u8; 32]),
            asset_id: 0,
            amount: 0,
            price: 0,
            timestamp: 0,
            status: TradeStatus::Submitted,
            settlement_type: SettlementType::Immediate,
            settlement_window_id: None,
            retry_count: 0,
        }
    }
}

impl Default for SettlementWindow {
    fn default() -> Self {
        Self {
            window_id: 0,
            start_time: 0,
            end_time: 0,
            is_active: true,
            is_processed: false,
            trade_ids: Vec::new(),
            total_volume: 0,
        }
    }
}

impl Default for DVPSettlement {
    fn default() -> Self {
        Self {
            trade_id: 0,
            asset_transfer_complete: false,
            payment_transfer_complete: false,
            settlement_complete: false,
            settlement_timestamp: None,
        }
    }
}