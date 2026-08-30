use ink::storage::Mapping;
use ink::prelude::vec::Vec;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

use super::types::{
    Trade, SettlementWindow, SettlementReport, SettlementRecord,
    NettingEntry, SettlementType,
};

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct ContractStorage {
    pub owner: AccountId,
    pub settlement_type: SettlementType,
    
    pub trades: Mapping<u64, Trade>,
    pub trade_counter: u64,
    
    pub settlement_windows: Mapping<u64, SettlementWindow>,
    pub window_counter: u64,
    
    pub reports: Mapping<u64, SettlementReport>,
    pub report_counter: u64,
    
    pub records: Mapping<u64, SettlementRecord>,
    pub record_counter: u64,
    
    pub pending_trades: Vec<u64>,
    pub failed_trades: Vec<u64>,
    pub settled_trades: Vec<u64>,
    
    pub paused: bool,
    
    pub t1_window_duration: u64,
    pub immediate_window_duration: u64,
    
    pub matched_trades: Mapping<u64, Vec<u64>>,
    pub account_trades: Mapping<AccountId, Vec<u64>>,
    
    pub asset_balances: Mapping<(AccountId, u128), u128>,
    
    pub window_retry_count: Mapping<u64, u32>,
    pub max_retries: u32,
}

impl Default for ContractStorage {
    fn default() -> Self {
        Self {
            owner: AccountId::from([0u8; 32]),
            settlement_type: SettlementType::Immediate,
            trades: Mapping::default(),
            trade_counter: 0,
            settlement_windows: Mapping::default(),
            window_counter: 0,
            reports: Mapping::default(),
            report_counter: 0,
            records: Mapping::default(),
            record_counter: 0,
            pending_trades: Vec::new(),
            failed_trades: Vec::new(),
            settled_trades: Vec::new(),
            paused: false,
            t1_window_duration: 86400,
            immediate_window_duration: 3600,
            matched_trades: Mapping::default(),
            account_trades: Mapping::default(),
            asset_balances: Mapping::default(),
            window_retry_count: Mapping::default(),
            max_retries: 3,
        }
    }
}

impl ContractStorage {
    pub fn new(owner: AccountId, settlement_type: SettlementType) -> Self {
        Self {
            owner,
            settlement_type,
            ..Default::default()
        }
    }

    pub fn add_trade(&mut self, trade: Trade) {
        self.trades.insert(trade.trade_id, &trade);
        self.trade_counter = self.trade_counter.max(trade.trade_id);
        self.pending_trades.push(trade.trade_id);
        
        let mut buyer_trades = self.account_trades.get(trade.buyer).unwrap_or_default();
        buyer_trades.push(trade.trade_id);
        self.account_trades.insert(trade.buyer, &buyer_trades);
        
        let mut seller_trades = self.account_trades.get(trade.seller).unwrap_or_default();
        seller_trades.push(trade.trade_id);
        self.account_trades.insert(trade.seller, &seller_trades);
    }

    pub fn add_settlement_window(&mut self, mut window: SettlementWindow) -> u64 {
        self.window_counter += 1;
        window.window_id = self.window_counter;
        self.settlement_windows.insert(window.window_id, &window);
        window.window_id
    }

    pub fn add_settlement_record(&mut self, mut record: SettlementRecord) -> u64 {
        self.record_counter += 1;
        record.record_id = self.record_counter;
        self.records.insert(record.record_id, &record);
        record.record_id
    }

    pub fn add_report(&mut self, mut report: SettlementReport) -> u64 {
        self.report_counter += 1;
        report.report_id = self.report_counter;
        self.reports.insert(report.report_id, &report);
        report.report_id
    }

    pub fn get_active_window(&self, current_time: u64) -> Option<u64> {
        for window_id in 1..=self.window_counter {
            if let Some(window) = self.settlement_windows.get(window_id) {
                if window.is_active && !window.is_processed && current_time < window.end_time {
                    return Some(window_id);
                }
            }
        }
        None
    }

    pub fn update_trade_status(&mut self, trade_id: u64, new_status: super::types::TradeStatus) -> Result<(), super::errors::Error> {
        let mut trade = self.trades.get(trade_id)
            .ok_or(super::errors::Error::TradeNotFound)?;
        
        if trade.status == super::types::TradeStatus::Settled {
            return Err(super::errors::Error::TradeAlreadySettled);
        }
        
        if let Some(pos) = self.pending_trades.iter().position(|&id| id == trade_id) {
            self.pending_trades.remove(pos);
        }
        
        match new_status {
            super::types::TradeStatus::Settled => self.settled_trades.push(trade_id),
            super::types::TradeStatus::Failed => self.failed_trades.push(trade_id),
            _ => self.pending_trades.push(trade_id),
        }
        
        trade.status = new_status;
        self.trades.insert(trade_id, &trade);
        
        Ok(())
    }

    pub fn get_account_balance(&self, account: AccountId, asset_id: u128) -> u128 {
        self.asset_balances.get((account, asset_id)).unwrap_or(0)
    }

    pub fn update_account_balance(&mut self, account: AccountId, asset_id: u128, delta: i128) -> Result<(), super::errors::Error> {
        let current = self.get_account_balance(account, asset_id);
        let new_balance = if delta >= 0 {
            current.saturating_add(delta as u128)
        } else {
            let abs_delta = (-delta) as u128;
            if current < abs_delta {
                return Err(super::errors::Error::InsufficientAssets);
            }
            current - abs_delta
        };
        
        self.asset_balances.insert((account, asset_id), &new_balance);
        Ok(())
    }
}