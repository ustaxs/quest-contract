#[cfg(test)]
mod tests {
    use super::*;
    use ink::env::test::{DefaultAccounts, set_caller, get_emitted_events};
    use ink::primitives::AccountId;
    use crate::settlement::{TradeSettlement, SettlementType, TradeStatus, Error, Trade};

    fn setup() -> (TradeSettlement, DefaultAccounts<ink::env::DefaultEnvironment>) {
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        let contract = TradeSettlement::new(SettlementType::Immediate);
        (contract, accounts)
    }

    #[ink::test]
    fn constructor_initializes_correctly() {
        let (contract, _) = setup();
        assert!(!contract.is_paused());
        assert!(matches!(contract.settlement_type, SettlementType::Immediate));
        assert_eq!(contract.trade_counter, 0);
        assert_eq!(contract.window_counter, 0);
    }

    #[ink::test]
    fn submit_trade_succeeds_with_valid_parameters() {
        let (mut contract, accounts) = setup();
        set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        
        let result = contract.submit_trade(accounts.bob, 1, 100, 10);
        assert!(result.is_ok());
        let trade_id = result.unwrap();
        assert_eq!(trade_id, 1);

        let trade = contract.get_trade(trade_id).unwrap();
        assert_eq!(trade.buyer, accounts.alice);
        assert_eq!(trade.seller, accounts.bob);
        assert_eq!(trade.asset_id, 1);
        assert_eq!(trade.amount, 100);
        assert_eq!(trade.price, 10);
        assert_eq!(trade.status, TradeStatus::Submitted);
    }

    #[ink::test]
    fn submit_trade_fails_with_zero_amount() {
        let (mut contract, accounts) = setup();
        set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        
        let result = contract.submit_trade(accounts.bob, 1, 0, 10);
        assert!(matches!(result, Err(Error::InvalidAmount)));
    }

    #[ink::test]
    fn submit_trade_fails_with_zero_price() {
        let (mut contract, accounts) = setup();
        set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        
        let result = contract.submit_trade(accounts.bob, 1, 100, 0);
        assert!(matches!(result, Err(Error::InvalidPrice)));
    }

    #[ink::test]
    fn submit_trade_fails_with_same_account() {
        let (mut contract, accounts) = setup();
        set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        
        let result = contract.submit_trade(accounts.alice, 1, 100, 10);
        assert!(matches!(result, Err(Error::SameAccountTrade)));
    }

    #[ink::test]
    fn match_trades_succeeds_with_matching_trades() {
        let (mut contract, accounts) = setup();
        
        set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        let trade1 = contract.submit_trade(accounts.bob, 1, 100, 10).unwrap();

        set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
        let trade2 = contract.submit_trade(accounts.alice, 1, 100, 10).unwrap();

        let result = contract.match_trades(trade1, trade2);
        assert!(result.is_ok());

        let trade = contract.get_trade(trade1).unwrap();
        assert_eq!(trade.status, TradeStatus::PendingSettlement);
        
        let matches = contract.matched_trades.get(trade1).unwrap();
        assert!(matches.contains(&trade2));
    }

    #[ink::test]
    fn match_trades_fails_with_mismatched_assets() {
        let (mut contract, accounts) = setup();
        
        set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        let trade1 = contract.submit_trade(accounts.bob, 1, 100, 10).unwrap();

        set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
        let trade2 = contract.submit_trade(accounts.alice, 2, 100, 10).unwrap();

        let result = contract.match_trades(trade1, trade2);
        assert!(matches!(result, Err(Error::TradeMismatch)));
    }

    #[ink::test]
    fn match_trades_fails_with_mismatched_amounts() {
        let (mut contract, accounts) = setup();
        
        set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        let trade1 = contract.submit_trade(accounts.bob, 1, 100, 10).unwrap();

        set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
        let trade2 = contract.submit_trade(accounts.alice, 1, 200, 10).unwrap();

        let result = contract.match_trades(trade1, trade2);
        assert!(matches!(result, Err(Error::TradeMismatch)));
    }

    #[ink::test]
    fn create_settlement_window_creates_valid_window() {
        let (mut contract, _) = setup();
        let window_id = contract.create_settlement_window().unwrap();
        assert_eq!(window_id, 1);

        let window = contract.get_settlement_window(window_id).unwrap();
        assert!(window.is_active);
        assert!(!window.is_processed);
        assert!(window.end_time > window.start_time);
    }

    #[ink::test]
    fn calculate_netting_computes_correct_net_positions() {
        let (mut contract, accounts) = setup();
        
        let window_id = contract.create_settlement_window().unwrap();
        
        set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        let trade1 = contract.submit_trade(accounts.bob, 1, 100, 10).unwrap();
        
        set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
        let trade2 = contract.submit_trade(accounts.charlie, 1, 50, 10).unwrap();
        
        let mut window = contract.settlement_windows.get(window_id).unwrap();
        window.trade_ids.push(trade1);
        window.trade_ids.push(trade2);
        contract.settlement_windows.insert(window_id, &window);

        let mut trade1_mut = contract.trades.get(trade1).unwrap();
        trade1_mut.status = TradeStatus::PendingSettlement;
        contract.trades.insert(trade1, &trade1_mut);
        
        let mut trade2_mut = contract.trades.get(trade2).unwrap();
        trade2_mut.status = TradeStatus::PendingSettlement;
        contract.trades.insert(trade2, &trade2_mut);

        let result = contract.calculate_netting(window_id);
        assert!(result.is_ok());
        let netting = result.unwrap();
        assert!(!netting.is_empty());
    }

    #[ink::test]
    fn pause_and_unpause_works_for_owner() {
        let (mut contract, accounts) = setup();
        set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        assert_eq!(contract.get_owner(), accounts.alice);
        
        let result = contract.pause();
        assert!(result.is_ok());
        assert!(contract.is_paused());
        
        let result = contract.unpause();
        assert!(result.is_ok());
        assert!(!contract.is_paused());
    }

    #[ink::test]
    fn pause_fails_for_non_owner() {
        let (mut contract, accounts) = setup();
        set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
        
        let result = contract.pause();
        assert!(matches!(result, Err(Error::Unauthorized)));
    }

    #[ink::test]
    fn cancel_trade_succeeds_for_trade_participant() {
        let (mut contract, accounts) = setup();
        set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        
        let trade_id = contract.submit_trade(accounts.bob, 1, 100, 10).unwrap();
        
        let result = contract.cancel_trade(trade_id);
        assert!(result.is_ok());
        
        let trade = contract.get_trade(trade_id).unwrap();
        assert_eq!(trade.status, TradeStatus::Cancelled);
    }

    #[ink::test]
    fn cancel_trade_fails_for_already_matched_trade() {
        let (mut contract, accounts) = setup();
        
        set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        let trade1 = contract.submit_trade(accounts.bob, 1, 100, 10).unwrap();
        
        set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
        let trade2 = contract.submit_trade(accounts.alice, 1, 100, 10).unwrap();
        
        contract.match_trades(trade1, trade2).unwrap();
        
        set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        let result = contract.cancel_trade(trade1);
        assert!(matches!(result, Err(Error::TradeAlreadyMatched)));
    }

    #[ink::test]
    fn retry_failed_trade_returns_to_pending() {
        let (mut contract, accounts) = setup();
        set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        
        let trade_id = contract.submit_trade(accounts.bob, 1, 100, 10).unwrap();
        
        let mut trade = contract.trades.get(trade_id).unwrap();
        trade.status = TradeStatus::Failed;
        contract.trades.insert(trade_id, &trade);
        
        let result = contract.retry_failed_trade(trade_id);
        assert!(result.is_ok());
        
        let updated_trade = contract.get_trade(trade_id).unwrap();
        assert_eq!(updated_trade.status, TradeStatus::PendingSettlement);
    }

    #[ink::test]
    fn get_account_trades_returns_correct_trades() {
        let (mut contract, accounts) = setup();
        set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        
        let trade1 = contract.submit_trade(accounts.bob, 1, 100, 10).unwrap();
        let trade2 = contract.submit_trade(accounts.charlie, 2, 200, 20).unwrap();
        
        let alice_trades = contract.get_account_trades(accounts.alice);
        assert_eq!(alice_trades.len(), 2);
        assert!(alice_trades.contains(&trade1));
        assert!(alice_trades.contains(&trade2));
    }

    #[ink::test]
    fn trades_emits_correct_events() {
        let (mut contract, accounts) = setup();
        set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        
        let _trade_id = contract.submit_trade(accounts.bob, 1, 100, 10).unwrap();
        
        let events = get_emitted_events::<ink::env::DefaultEnvironment>();
        assert!(!events.is_empty());
    }

    #[ink::test]
    fn t1_settlement_type_creates_correct_duration() {
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        let mut contract = TradeSettlement::new(SettlementType::TPlus1);
        
        let window_id = contract.create_settlement_window().unwrap();
        let window = contract.get_settlement_window(window_id).unwrap();
        
        assert_eq!(window.end_time - window.start_time, 86400);
    }

    #[ink::test]
    fn immediate_settlement_type_creates_correct_duration() {
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        let mut contract = TradeSettlement::new(SettlementType::Immediate);
        
        let window_id = contract.create_settlement_window().unwrap();
        let window = contract.get_settlement_window(window_id).unwrap();
        
        assert_eq!(window.end_time - window.start_time, 3600);
    }

    #[ink::test]
    fn get_active_window_returns_current_window() {
        let (mut contract, _) = setup();
        contract.create_settlement_window().unwrap();
        
        let window_id = contract.get_active_window().unwrap();
        assert_eq!(window_id, 1);
    }

    #[ink::test]
    fn add_to_settlement_window_assigns_trade_to_window() {
        let (mut contract, accounts) = setup();
        
        let window_id = contract.create_settlement_window().unwrap();
        
        set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        let trade_id = contract.submit_trade(accounts.bob, 1, 100, 10).unwrap();
        
        contract.add_to_settlement_window(trade_id).unwrap();
        
        let window = contract.get_settlement_window(window_id).unwrap();
        assert!(window.trade_ids.contains(&trade_id));
        
        let trade = contract.get_trade(trade_id).unwrap();
        assert_eq!(trade.settlement_window_id, Some(window_id));
        assert_eq!(trade.status, TradeStatus::PendingSettlement);
    }

    #[ink::test]
    fn settlement_processing_requires_window_end_time_passed() {
        let (mut contract, accounts) = setup();
        
        let window_id = contract.create_settlement_window().unwrap();
        
        set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        let trade1 = contract.submit_trade(accounts.bob, 1, 100, 10).unwrap();
        contract.add_to_settlement_window(trade1).unwrap();
        
        let result = contract.process_settlement(window_id);
        assert!(matches!(result, Err(Error::WindowStillOpen)));
    }

    #[ink::test]
    fn non_existent_trade_returns_error() {
        let (mut contract, _) = setup();
        let result = contract.get_trade(999);
        assert!(result.is_none());
    }

    #[ink::test]
    fn non_existent_window_returns_error() {
        let (contract, _) = setup();
        let result = contract.get_settlement_window(999);
        assert!(result.is_none());
    }
}