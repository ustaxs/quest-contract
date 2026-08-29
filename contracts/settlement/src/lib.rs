#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::result_large_err)]

pub use self::settlement::*;

#[ink::contract]
mod settlement {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use parity_scale_codec::{Decode, Encode};
    use scale_info::TypeInfo;

    #[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(ink::scale_info::prelude::fmt::Debug))]
    pub enum SettlementType {
        Immediate,
        TPlus1,
    }

    #[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(ink::scale_info::prelude::fmt::Debug))]
    pub enum TradeStatus {
        Submitted,
        Matched,
        PendingSettlement,
        Settled,
        Failed,
        Cancelled,
    }

    #[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(ink::scale_info::prelude::fmt::Debug))]
    pub enum SettlementStatus {
        Pending,
        Processing,
        Completed,
        Failed,
        Retried,
    }

    #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::scale_info::prelude::fmt::Debug))]
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
    }

    #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::scale_info::prelude::fmt::Debug))]
    pub struct SettlementWindow {
        pub window_id: u64,
        pub start_time: u64,
        pub end_time: u64,
        pub is_active: bool,
        pub is_processed: bool,
        pub trade_ids: Vec<u64>,
    }

    #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::scale_info::prelude::fmt::Debug))]
    pub struct NettingEntry {
        pub account: AccountId,
        pub asset_id: u128,
        pub net_amount: i128,
        pub net_payment: i128,
    }

    #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::scale_info::prelude::fmt::Debug))]
    pub struct SettlementReport {
        pub report_id: u64,
        pub window_id: u64,
        pub timestamp: u64,
        pub total_trades: u32,
        pub settled_trades: u32,
        pub failed_trades: u32,
        pub total_volume: u128,
        pub netting_entries: Vec<NettingEntry>,
    }

    #[derive(Debug, Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::scale_info::prelude::fmt::Debug))]
    pub struct SettlementRecord {
        pub record_id: u64,
        pub trade_id: u64,
        pub account: AccountId,
        pub asset_transferred: u128,
        pub payment_processed: u128,
        pub timestamp: u64,
        pub status: SettlementStatus,
        pub fail_reason: Option<Vec<u8>>,
    }

    #[ink(storage)]
    pub struct TradeSettlement {
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
        pub is_paused: bool,
        pub t1_window_duration: u64,
        pub immediate_window_duration: u64,
        pub matched_trades: Mapping<u64, Vec<u64>>,
        pub account_trades: Mapping<AccountId, Vec<u64>>,
        pub paused: bool,
    }

    #[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
    #[ink::scale_derive(Debug)]
    pub enum Error {
        ContractPaused,
        Unauthorized,
        TradeNotFound,
        TradeAlreadyMatched,
        TradeAlreadySettled,
        InvalidTrade,
        SettlementWindowNotFound,
        SettlementWindowNotActive,
        SettlementWindowAlreadyProcessed,
        InsufficientFunds,
        InsufficientAssets,
        DVPNotSatisfied,
        PaymentVerificationFailed,
        TransferFailed,
        InvalidSettlementType,
        NettingCalculationFailed,
        ReportGenerationFailed,
        TradeMismatch,
        NoTradesToSettle,
        WindowStillOpen,
        RetryLimitExceeded,
        AccountNotFound,
        InvalidAmount,
        InvalidPrice,
        SameAccountTrade,
    }

    #[ink(event)]
    pub struct TradeSubmitted {
        #[ink(topic)]
        pub trade_id: u64,
        pub buyer: AccountId,
        pub seller: AccountId,
        pub asset_id: u128,
        pub amount: u128,
        pub price: u128,
    }

    #[ink(event)]
    pub struct TradeMatched {
        #[ink(topic)]
        pub trade_id: u64,
        pub matching_trade_id: u64,
    }

    #[ink(event)]
    pub struct SettlementWindowCreated {
        #[ink(topic)]
        pub window_id: u64,
        pub start_time: u64,
        pub end_time: u64,
    }

    #[ink(event)]
    pub struct NettingCompleted {
        #[ink(topic)]
        pub window_id: u64,
        pub netting_entries: Vec<NettingEntry>,
    }

    #[ink(event)]
    pub struct TradeSettled {
        #[ink(topic)]
        pub trade_id: u64,
        pub buyer: AccountId,
        pub seller: AccountId,
        pub amount: u128,
        pub total_payment: u128,
    }

    #[ink(event)]
    pub struct SettlementFailed {
        #[ink(topic)]
        pub trade_id: u64,
        pub reason: Error,
    }

    #[ink(event)]
    pub struct SettlementReportGenerated {
        #[ink(topic)]
        pub report_id: u64,
        pub window_id: u64,
        pub total_trades: u32,
        pub settled_trades: u32,
        pub total_volume: u128,
    }

    #[ink(event)]
    pub struct ContractPaused {
        pub account: AccountId,
    }

    #[ink(event)]
    pub struct ContractUnpaused {
        pub account: AccountId,
    }

    impl TradeSettlement {
        #[ink(constructor)]
        pub fn new(settlement_type: SettlementType) -> Self {
            let caller = Self::env().caller();
            Self {
                owner: caller,
                settlement_type,
                trades: Mapping::default(),
                trade_counter: 0,
                settlement_windows: Mapping::default(),
                window_counter: 0,
                reports: Mapping::default(),
                report_counter: 0,
                records: Mapping::default(),
                record_counter: 0,
                pending_trades: Vec::new(),
                is_paused: false,
                t1_window_duration: 86400,
                immediate_window_duration: 3600,
                matched_trades: Mapping::default(),
                account_trades: Mapping::default(),
                paused: false,
            }
        }

        #[ink(message)]
        pub fn submit_trade(
            &mut self,
            seller: AccountId,
            asset_id: u128,
            amount: u128,
            price: u128,
        ) -> Result<u64, Error> {
            if self.paused {
                return Err(Error::ContractPaused);
            }

            let caller = self.env().caller();
            if amount == 0 {
                return Err(Error::InvalidAmount);
            }
            if price == 0 {
                return Err(Error::InvalidPrice);
            }
            if caller == seller {
                return Err(Error::SameAccountTrade);
            }

            self.trade_counter += 1;
            let trade_id = self.trade_counter;
            let current_timestamp = self.env().block_timestamp();

            let trade = Trade {
                trade_id,
                buyer: caller,
                seller,
                asset_id,
                amount,
                price,
                timestamp: current_timestamp,
                status: TradeStatus::Submitted,
                settlement_type: self.settlement_type.clone(),
                settlement_window_id: None,
            };

            self.trades.insert(trade_id, &trade);
            self.pending_trades.push(trade_id);

            let mut buyer_trades = self.account_trades.get(caller).unwrap_or_default();
            buyer_trades.push(trade_id);
            self.account_trades.insert(caller, &buyer_trades);

            let mut seller_trades = self.account_trades.get(seller).unwrap_or_default();
            seller_trades.push(trade_id);
            self.account_trades.insert(seller, &seller_trades);

            self.env().emit_event(TradeSubmitted {
                trade_id,
                buyer: caller,
                seller,
                asset_id,
                amount,
                price,
            });

            Ok(trade_id)
        }

        #[ink(message)]
        pub fn match_trades(&mut self, trade_id1: u64, trade_id2: u64) -> Result<(), Error> {
            if self.paused {
                return Err(Error::ContractPaused);
            }

            let mut trade1 = self.trades.get(trade_id1).ok_or(Error::TradeNotFound)?;
            let mut trade2 = self.trades.get(trade_id2).ok_or(Error::TradeNotFound)?;

            if trade1.status != TradeStatus::Submitted || trade2.status != TradeStatus::Submitted {
                return Err(Error::TradeAlreadyMatched);
            }

            if !self.validate_trade_match(&trade1, &trade2) {
                return Err(Error::TradeMismatch);
            }

            trade1.status = TradeStatus::Matched;
            trade2.status = TradeStatus::Matched;

            self.trades.insert(trade_id1, &trade1);
            self.trades.insert(trade_id2, &trade2);

            let mut matches1 = self.matched_trades.get(trade_id1).unwrap_or_default();
            matches1.push(trade_id2);
            self.matched_trades.insert(trade_id1, &matches1);

            let mut matches2 = self.matched_trades.get(trade_id2).unwrap_or_default();
            matches2.push(trade_id1);
            self.matched_trades.insert(trade_id2, &matches2);

            self.add_to_settlement_window(trade_id1)?;
            self.add_to_settlement_window(trade_id2)?;

            self.env().emit_event(TradeMatched {
                trade_id: trade_id1,
                matching_trade_id: trade_id2,
            });
            self.env().emit_event(TradeMatched {
                trade_id: trade_id2,
                matching_trade_id: trade_id1,
            });

            Ok(())
        }

        fn validate_trade_match(&self, trade1: &Trade, trade2: &Trade) -> bool {
            trade1.buyer == trade2.seller
                && trade1.seller == trade2.buyer
                && trade1.asset_id == trade2.asset_id
                && trade1.amount == trade2.amount
                && trade1.price == trade2.price
                && trade1.trade_id != trade2.trade_id
        }

        #[ink(message)]
        pub fn create_settlement_window(&mut self) -> Result<u64, Error> {
            if self.paused {
                return Err(Error::ContractPaused);
            }

            self.window_counter += 1;
            let window_id = self.window_counter;
            let current_timestamp = self.env().block_timestamp();

            let duration = match self.settlement_type {
                SettlementType::Immediate => self.immediate_window_duration,
                SettlementType::TPlus1 => self.t1_window_duration,
            };

            let window = SettlementWindow {
                window_id,
                start_time: current_timestamp,
                end_time: current_timestamp + duration,
                is_active: true,
                is_processed: false,
                trade_ids: Vec::new(),
            };

            self.settlement_windows.insert(window_id, &window);

            self.env().emit_event(SettlementWindowCreated {
                window_id,
                start_time: current_timestamp,
                end_time: current_timestamp + duration,
            });

            Ok(window_id)
        }

        fn add_to_settlement_window(&mut self, trade_id: u64) -> Result<(), Error> {
            let mut trade = self.trades.get(trade_id).ok_or(Error::TradeNotFound)?;
            
            let mut active_window = self.get_active_window()?;
            let mut window = self.settlement_windows.get(active_window).ok_or(Error::SettlementWindowNotFound)?;
            
            trade.status = TradeStatus::PendingSettlement;
            trade.settlement_window_id = Some(window.window_id);
            self.trades.insert(trade_id, &trade);
            
            window.trade_ids.push(trade_id);
            self.settlement_windows.insert(window.window_id, &window);
            
            Ok(())
        }

        fn get_active_window(&mut self) -> Result<u64, Error> {
            for window_id in 1..=self.window_counter {
                if let Some(mut window) = self.settlement_windows.get(window_id) {
                    let current_timestamp = self.env().block_timestamp();
                    if window.is_active && !window.is_processed && current_timestamp < window.end_time {
                        return Ok(window_id);
                    } else if window.is_active && !window.is_processed && current_timestamp >= window.end_time {
                        window.is_active = false;
                        self.settlement_windows.insert(window_id, &window);
                    }
                }
            }
            
            self.create_settlement_window()
        }

        #[ink(message)]
        pub fn calculate_netting(&mut self, window_id: u64) -> Result<Vec<NettingEntry>, Error> {
            if self.paused {
                return Err(Error::ContractPaused);
            }

            let window = self.settlement_windows.get(window_id).ok_or(Error::SettlementWindowNotFound)?;
            if window.is_processed {
                return Err(Error::SettlementWindowAlreadyProcessed);
            }

            let mut net_balances: ink::prelude::BTreeMap<(AccountId, u128), i128> = ink::prelude::BTreeMap::new();
            let mut mut_payments: ink::prelude::BTreeMap<AccountId, i128> = ink::prelude::BTreeMap::new();

            for &trade_id in &window.trade_ids {
                let trade = self.trades.get(trade_id).ok_or(Error::TradeNotFound)?;
                if trade.status != TradeStatus::PendingSettlement {
                    continue;
                }

                let buyer_key = (trade.buyer, trade.asset_id);
                let seller_key = (trade.seller, trade.asset_id);
                
                *net_balances.entry(buyer_key).or_insert(0) += trade.amount as i128;
                *net_balances.entry(seller_key).or_insert(0) -= trade.amount as i128;

                let total_payment = (trade.amount as u128).saturating_mul(trade.price);
                *mut_payments.entry(trade.buyer).or_insert(0) -= total_payment as i128;
                *mut_payments.entry(trade.seller).or_insert(0) += total_payment as i128;
            }

            let mut netting_entries = Vec::new();
            for ((account, asset_id), net_amount) in net_balances {
                let net_payment = mut_payments.get(&account).copied().unwrap_or(0);
                netting_entries.push(NettingEntry {
                    account,
                    asset_id,
                    net_amount,
                    net_payment,
                });
            }

            self.env().emit_event(NettingCompleted {
                window_id,
                netting_entries: netting_entries.clone(),
            });

            Ok(netting_entries)
        }

        #[ink(message)]
        pub fn process_settlement(&mut self, window_id: u64) -> Result<(), Error> {
            if self.paused {
                return Err(Error::ContractPaused);
            }

            let mut window = self.settlement_windows.get(window_id).ok_or(Error::SettlementWindowNotFound)?;
            if window.is_processed {
                return Err(Error::SettlementWindowAlreadyProcessed);
            }

            let current_timestamp = self.env().block_timestamp();
            if current_timestamp < window.end_time {
                return Err(Error::WindowStillOpen);
            }

            if window.trade_ids.is_empty() {
                return Err(Error::NoTradesToSettle);
            }

            let netting_entries = self.calculate_netting(window_id)?;

            let mut settled_count = 0;
            let mut failed_count = 0;
            let mut total_volume = 0;

            for &trade_id in &window.trade_ids {
                match self.execute_dvp(trade_id) {
                    Ok(_) => {
                        settled_count += 1;
                        let trade = self.trades.get(trade_id).ok_or(Error::TradeNotFound)?;
                        total_volume = total_volume.saturating_add(trade.amount.saturating_mul(trade.price));
                    }
                    Err(e) => {
                        failed_count += 1;
                        self.env().emit_event(SettlementFailed {
                            trade_id,
                            reason: e,
                        });
                    }
                }
            }

            window.is_processed = true;
            self.settlement_windows.insert(window_id, &window);

            self.generate_settlement_report(
                window_id,
                window.trade_ids.len() as u32,
                settled_count,
                failed_count,
                total_volume,
                netting_entries,
            )?;

            Ok(())
        }

        fn execute_dvp(&mut self, trade_id: u64) -> Result<(), Error> {
            let mut trade = self.trades.get(trade_id).ok_or(Error::TradeNotFound)?;
            
            if !self.verify_payment(&trade.buyer, trade.amount.saturating_mul(trade.price)) {
                trade.status = TradeStatus::Failed;
                self.trades.insert(trade_id, &trade);
                self.record_settlement_failure(trade_id, &trade.buyer, Error::PaymentVerificationFailed);
                return Err(Error::PaymentVerificationFailed);
            }

            if !self.verify_assets(&trade.seller, trade.asset_id, trade.amount) {
                trade.status = TradeStatus::Failed;
                self.trades.insert(trade_id, &trade);
                self.record_settlement_failure(trade_id, &trade.seller, Error::InsufficientAssets);
                return Err(Error::InsufficientAssets);
            }

            match self.transfer_payment(&trade.buyer, &trade.seller, trade.amount.saturating_mul(trade.price)) {
                Ok(_) => {}
                Err(_) => {
                    trade.status = TradeStatus::Failed;
                    self.trades.insert(trade_id, &trade);
                    self.record_settlement_failure(trade_id, &trade.buyer, Error::TransferFailed);
                    return Err(Error::TransferFailed);
                }
            }

            match self.transfer_assets(&trade.seller, &trade.buyer, trade.asset_id, trade.amount) {
                Ok(_) => {}
                Err(_) => {
                    trade.status = TradeStatus::Failed;
                    self.trades.insert(trade_id, &trade);
                    self.record_settlement_failure(trade_id, &trade.seller, Error::TransferFailed);
                    return Err(Error::TransferFailed);
                }
            }

            trade.status = TradeStatus::Settled;
            self.trades.insert(trade_id, &trade);

            self.record_successful_settlement(trade_id, &trade)?;

            self.env().emit_event(TradeSettled {
                trade_id,
                buyer: trade.buyer,
                seller: trade.seller,
                amount: trade.amount,
                total_payment: trade.amount.saturating_mul(trade.price),
            });

            Ok(())
        }

        fn verify_payment(&self, account: &AccountId, amount: u128) -> bool {
            let balance = self.env().balance_of(account);
            balance >= amount
        }

        fn verify_assets(&self, _account: &AccountId, _asset_id: u128, _amount: u128) -> bool {
            true
        }

        fn transfer_payment(&mut self, from: &AccountId, to: &AccountId, amount: u128) -> Result<(), Error> {
            if self.env().transfer(*to, amount).is_err() {
                return Err(Error::TransferFailed);
            }
            Ok(())
        }

        fn transfer_assets(&mut self, _from: &AccountId, _to: &AccountId, _asset_id: u128, _amount: u128) -> Result<(), Error> {
            Ok(())
        }

        fn record_successful_settlement(&mut self, trade_id: u64, trade: &Trade) -> Result<(), Error> {
            self.record_counter += 1;
            let record_id = self.record_counter;
            let current_timestamp = self.env().block_timestamp();

            let record = SettlementRecord {
                record_id,
                trade_id,
                account: trade.buyer,
                asset_transferred: trade.amount,
                payment_processed: trade.amount.saturating_mul(trade.price),
                timestamp: current_timestamp,
                status: SettlementStatus::Completed,
                fail_reason: None,
            };

            self.records.insert(record_id, &record);
            Ok(())
        }

        fn record_settlement_failure(&mut self, trade_id: u64, account: &AccountId, error: Error) {
            self.record_counter += 1;
            let record_id = self.record_counter;
            let current_timestamp = self.env().block_timestamp();

            let mut encoded_error = Vec::new();
            error.encode_to(&mut encoded_error);

            let record = SettlementRecord {
                record_id,
                trade_id,
                account: *account,
                asset_transferred: 0,
                payment_processed: 0,
                timestamp: current_timestamp,
                status: SettlementStatus::Failed,
                fail_reason: Some(encoded_error),
            };

            self.records.insert(record_id, &record);
        }

        fn generate_settlement_report(
            &mut self,
            window_id: u64,
            total_trades: u32,
            settled_trades: u32,
            failed_trades: u32,
            total_volume: u128,
            netting_entries: Vec<NettingEntry>,
        ) -> Result<u64, Error> {
            self.report_counter += 1;
            let report_id = self.report_counter;
            let current_timestamp = self.env().block_timestamp();

            let report = SettlementReport {
                report_id,
                window_id,
                timestamp: current_timestamp,
                total_trades,
                settled_trades,
                failed_trades,
                total_volume,
                netting_entries,
            };

            self.reports.insert(report_id, &report);

            self.env().emit_event(SettlementReportGenerated {
                report_id,
                window_id,
                total_trades,
                settled_trades,
                total_volume,
            });

            Ok(report_id)
        }

        #[ink(message)]
        pub fn get_trade(&self, trade_id: u64) -> Option<Trade> {
            self.trades.get(trade_id)
        }

        #[ink(message)]
        pub fn get_settlement_window(&self, window_id: u64) -> Option<SettlementWindow> {
            self.settlement_windows.get(window_id)
        }

        #[ink(message)]
        pub fn get_settlement_report(&self, report_id: u64) -> Option<SettlementReport> {
            self.reports.get(report_id)
        }

        #[ink(message)]
        pub fn get_account_trades(&self, account: AccountId) -> Vec<u64> {
            self.account_trades.get(account).unwrap_or_default()
        }

        #[ink(message)]
        pub fn get_settlement_record(&self, record_id: u64) -> Option<SettlementRecord> {
            self.records.get(record_id)
        }

        #[ink(message)]
        pub fn pause(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::Unauthorized);
            }
            self.paused = true;
            self.env().emit_event(ContractPaused { account: caller });
            Ok(())
        }

        #[ink(message)]
        pub fn unpause(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::Unauthorized);
            }
            self.paused = false;
            self.env().emit_event(ContractUnpaused { account: caller });
            Ok(())
        }

        #[ink(message)]
        pub fn is_paused(&self) -> bool {
            self.paused
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn retry_failed_trade(&mut self, trade_id: u64) -> Result<(), Error> {
            if self.paused {
                return Err(Error::ContractPaused);
            }

            let mut trade = self.trades.get(trade_id).ok_or(Error::TradeNotFound)?;
            if trade.status != TradeStatus::Failed {
                return Err(Error::TradeAlreadySettled);
            }

            trade.status = TradeStatus::PendingSettlement;
            self.add_to_settlement_window(trade_id)?;
            self.trades.insert(trade_id, &trade);

            Ok(())
        }

        #[ink(message)]
        pub fn cancel_trade(&mut self, trade_id: u64) -> Result<(), Error> {
            let caller = self.env().caller();
            let mut trade = self.trades.get(trade_id).ok_or(Error::TradeNotFound)?;

            if trade.buyer != caller && trade.seller != caller && caller != self.owner {
                return Err(Error::Unauthorized);
            }

            if trade.status != TradeStatus::Submitted {
                return Err(Error::TradeAlreadyMatched);
            }

            trade.status = TradeStatus::Cancelled;
            self.trades.insert(trade_id, &trade);

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test::DefaultAccounts;

        fn setup_contract() -> TradeSettlement {
            let accounts = default_accounts();
            TradeSettlement::new(SettlementType::Immediate)
        }

        fn default_accounts() -> DefaultAccounts<ink::env::AccountId> {
            ink::env::test::default_accounts()
        }

        #[ink::test]
        fn test_constructor_works() {
            let contract = setup_contract();
            assert_eq!(contract.is_paused(), false);
            assert!(matches!(contract.settlement_type, SettlementType::Immediate));
        }

        #[ink::test]
        fn test_submit_trade_works() {
            let mut contract = setup_contract();
            let accounts = default_accounts();
            
            let result = contract.submit_trade(accounts.bob, 1, 100, 10);
            assert!(result.is_ok());
            let trade_id = result.unwrap();
            assert_eq!(trade_id, 1);

            let trade = contract.get_trade(trade_id).unwrap();
            assert_eq!(trade.status, TradeStatus::Submitted);
            assert_eq!(trade.amount, 100);
            assert_eq!(trade.price, 10);
        }

        #[ink::test]
        fn test_match_trades_works() {
            let mut contract = setup_contract();
            let accounts = default_accounts();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            let trade1 = contract.submit_trade(accounts.bob, 1, 100, 10).unwrap();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            let trade2 = contract.submit_trade(accounts.alice, 1, 100, 10).unwrap();

            let result = contract.match_trades(trade1, trade2);
            assert!(result.is_ok());

            let trade = contract.get_trade(trade1).unwrap();
            assert_eq!(trade.status, TradeStatus::PendingSettlement);
        }

        #[ink::test]
        fn test_pause_unpause_works() {
            let mut contract = setup_contract();
            let accounts = default_accounts();

            assert!(!contract.is_paused());
            
            let result = contract.pause();
            assert!(result.is_ok());
            assert!(contract.is_paused());

            let result = contract.unpause();
            assert!(result.is_ok());
            assert!(!contract.is_paused());
        }

        #[ink::test]
        fn test_cancel_trade_works() {
            let mut contract = setup_contract();
            let accounts = default_accounts();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            let trade_id = contract.submit_trade(accounts.bob, 1, 100, 10).unwrap();

            let result = contract.cancel_trade(trade_id);
            assert!(result.is_ok());

            let trade = contract.get_trade(trade_id).unwrap();
            assert_eq!(trade.status, TradeStatus::Cancelled);
        }

        #[ink::test]
        fn test_create_settlement_window() {
            let mut contract = setup_contract();
            let window_id = contract.create_settlement_window().unwrap();
            assert_eq!(window_id, 1);

            let window = contract.get_settlement_window(window_id).unwrap();
            assert!(window.is_active);
            assert!(!window.is_processed);
        }
    }
}