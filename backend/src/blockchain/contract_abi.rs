use ethers::prelude::*;

// WeatherInsurance contract ABI
abigen!(
    WeatherInsurance,
    r#"[
        function buyPolicy(uint256 duration, uint256 payout, int256 threshold, string memory eventType, string memory h3HexId) external payable
        function trigger(uint256 policyId, int256 observed) external
        function policies(uint256) external view returns (address user, uint256 payout, uint256 startTime, uint256 endTime, bool paid, int256 threshold, string memory eventType, string memory h3HexId)
        function policyCount() external view returns (uint256)
        function owner() external view returns (address)
        event PolicyCreated(uint256 indexed policyId, address indexed user, uint256 payout, uint256 startTime, uint256 endTime)
        event PolicyTriggered(uint256 indexed policyId, uint256 payout, bool triggered)
    ]"#,
);

// Struct to represent a policy as it exists on the blockchain
#[derive(Debug, Clone)]
pub struct BlockchainPolicy {
    pub user: Address,
    pub payout: U256,
    pub start_time: U256,
    pub end_time: U256,
    pub paid: bool,
    pub threshold: I256,
    pub event_type: String,
    pub h3_hex_id: String,
}

// Struct to represent a buy policy transaction details
#[derive(Debug, Clone)]
pub struct BuyPolicyTransaction {
    pub duration: U256,
    pub payout: U256,
    pub threshold: I256,
    pub event_type: String,
    pub h3_hex_id: String,
    pub premium_paid: U256,
    pub buyer: Address,
}
