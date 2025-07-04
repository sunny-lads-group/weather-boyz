// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

contract WeatherInsurance {
    address public owner;

    struct Policy {
        address user;
        uint payout;
        uint startTime;
        uint endTime;
        bool paid;
        int256 threshold;
        string eventType;
        string h3HexId;
    }

    mapping(uint => Policy) public policies;
    uint public policyCount = 0;

    constructor() {
        owner = msg.sender;
    }

    function buyPolicy(uint duration, uint payout, int256 threshold, string memory eventType, string memory h3HexId) public payable {
        require(duration > 0, "Duration must be greater than 0");
        require(payout > 0, "Payout must be greater than 0");
        require(msg.value >= payout / 10, "Premium too low");
        require(bytes(eventType).length > 0, "Event type cannot be empty");
        require(bytes(h3HexId).length > 0, "H3 hex ID cannot be empty");

        policies[policyCount] = Policy(
            msg.sender,
            payout,
            block.timestamp,
            block.timestamp + duration,
            false,
            threshold,
            eventType,
            h3HexId
        );
        policyCount++;
    }

    function trigger(uint policyId, int256 observed) public {
        Policy storage p = policies[policyId];
        require(!p.paid, "Already settled");
        require(msg.sender == owner, "Only owner (oracle) can trigger");

        if (
            keccak256(bytes(p.eventType)) == keccak256("TEMP_BELOW") &&
            observed < p.threshold
        ) {
            payable(p.user).transfer(p.payout);
        }

        p.paid = true;
    }
}