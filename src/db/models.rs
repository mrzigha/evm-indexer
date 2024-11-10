use serde::{Serialize, Deserialize};
use mongodb::bson::{Document, DateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventLog {
    pub chain_name: String,
    pub event_name: String,
    pub block_number: u64,
    pub transaction_hash: String,
    pub params: Document,
    pub timestamp: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventType {
    CoordinatorSet,
    DevTaxSent,
    LotteryClaimed,
    LotteryCreated,
    LotteryIncentivized,
    LotteryWinnerDrawn,
    LotteryWinnerRequestSent,
    OwnerChanged,
    OwnershipTransferRequested,
    OwnershipTransferred,
    ReferralTaxSent,
    RequestFulfilled,
    TicketsBought,
}
