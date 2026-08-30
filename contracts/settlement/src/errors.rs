use ink::prelude::vec::Vec;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
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
    WindowCreationFailed,
    MatchValidationFailed,
    AssetTransferFailed,
    PaymentTransferFailed,
    InsufficientBalance,
    TradeNotInPendingState,
    InvalidWindowTransition,
    ReportNotFound,
    RecordNotFound,
    DuplicateTrade,
    InvalidTradeTransition,
    NettingAlreadyProcessed,
    SettlementInProgress,
    FailsafeActivated,
    EmergencyStopActive,
    InvalidAccount,
    AssetNotFound,
    InsufficientCollateral,
    SettlementTooLarge,
    CounterpartyRiskLimitExceeded,
    SelfSettlementNotAllowed,
    UnsupportedAsset,
    SettlementWindowMismatch,
    TradeExpired,
    VerificationTimeout,
    SystemOverload,
}

impl Error {
    pub fn to_string(&self) -> Vec<u8> {
        match self {
            Error::ContractPaused => b"Contract is paused".to_vec(),
            Error::Unauthorized => b"Unauthorized access".to_vec(),
            Error::TradeNotFound => b"Trade not found".to_vec(),
            Error::TradeAlreadyMatched => b"Trade already matched".to_vec(),
            Error::TradeAlreadySettled => b"Trade already settled".to_vec(),
            Error::InvalidTrade => b"Invalid trade parameters".to_vec(),
            Error::SettlementWindowNotFound => b"Settlement window not found".to_vec(),
            Error::SettlementWindowNotActive => b"Settlement window not active".to_vec(),
            Error::SettlementWindowAlreadyProcessed => b"Settlement window already processed".to_vec(),
            Error::InsufficientFunds => b"Insufficient funds".to_vec(),
            Error::InsufficientAssets => b"Insufficient assets".to_vec(),
            Error::DVPNotSatisfied => b"DVP conditions not satisfied".to_vec(),
            Error::PaymentVerificationFailed => b"Payment verification failed".to_vec(),
            Error::TransferFailed => b"Transfer failed".to_vec(),
            Error::InvalidSettlementType => b"Invalid settlement type".to_vec(),
            Error::NettingCalculationFailed => b"Netting calculation failed".to_vec(),
            Error::ReportGenerationFailed => b"Report generation failed".to_vec(),
            Error::TradeMismatch => b"Trade details mismatch for matching".to_vec(),
            Error::NoTradesToSettle => b"No trades to settle in window".to_vec(),
            Error::WindowStillOpen => b"Settlement window still open".to_vec(),
            Error::RetryLimitExceeded => b"Retry limit exceeded".to_vec(),
            Error::AccountNotFound => b"Account not found".to_vec(),
            Error::InvalidAmount => b"Invalid amount specified".to_vec(),
            Error::InvalidPrice => b"Invalid price specified".to_vec(),
            Error::SameAccountTrade => b"Cannot trade with same account".to_vec(),
            Error::WindowCreationFailed => b"Failed to create settlement window".to_vec(),
            Error::MatchValidationFailed => b"Trade match validation failed".to_vec(),
            Error::AssetTransferFailed => b"Asset transfer failed".to_vec(),
            Error::PaymentTransferFailed => b"Payment transfer failed".to_vec(),
            Error::InsufficientBalance => b"Insufficient account balance".to_vec(),
            Error::TradeNotInPendingState => b"Trade not in pending state".to_vec(),
            Error::InvalidWindowTransition => b"Invalid window state transition".to_vec(),
            Error::ReportNotFound => b"Settlement report not found".to_vec(),
            Error::RecordNotFound => b"Settlement record not found".to_vec(),
            Error::DuplicateTrade => b"Duplicate trade detected".to_vec(),
            Error::InvalidTradeTransition => b"Invalid trade status transition".to_vec(),
            Error::NettingAlreadyProcessed => b"Netting already processed for window".to_vec(),
            Error::SettlementInProgress => b"Settlement currently in progress".to_vec(),
            Error::FailsafeActivated => b"Failsafe mechanism activated".to_vec(),
            Error::EmergencyStopActive => b"Emergency stop is active".to_vec(),
            Error::InvalidAccount => b"Invalid account address".to_vec(),
            Error::AssetNotFound => b"Asset not found".to_vec(),
            Error::InsufficientCollateral => b"Insufficient collateral".to_vec(),
            Error::SettlementTooLarge => b"Settlement amount exceeds limits".to_vec(),
            Error::CounterpartyRiskLimitExceeded => b"Counterparty risk limit exceeded".to_vec(),
            Error::SelfSettlementNotAllowed => b"Self-settlement not allowed".to_vec(),
            Error::UnsupportedAsset => b"Unsupported asset for settlement".to_vec(),
            Error::SettlementWindowMismatch => b"Settlement window mismatch".to_vec(),
            Error::TradeExpired => b"Trade has expired".to_vec(),
            Error::VerificationTimeout => b"Verification timeout".to_vec(),
            Error::SystemOverload => b"System overload, please retry later".to_vec(),
        }
    }

    pub fn is_retryable(&self) -> bool {
        matches!(self,
            Error::InsufficientFunds |
            Error::InsufficientAssets |
            Error::TransferFailed |
            Error::PaymentTransferFailed |
            Error::AssetTransferFailed |
            Error::InsufficientBalance |
            Error::VerificationTimeout |
            Error::SystemOverload
        )
    }

    pub fn is_critical(&self) -> bool {
        matches!(self,
            Error::FailsafeActivated |
            Error::EmergencyStopActive |
            Error::Unauthorized |
            Error::InvalidSettlementType
        )
    }
}

impl From<&Error> for Vec<u8> {
    fn from(error: &Error) -> Self {
        error.to_string()
    }
}