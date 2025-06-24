# Weather Boyz - Development Plan

Hi @housemech,

I’ve just created [this PR](https://github.com/sunny-lads-group/weather-boyz/pull/19) and have outlined the next steps for development. Here’s what I’m proposing:

## 1. Database & Policy Definitions

First, we need to add a table to the database to manage the policies we offer. This will allow us to define criteria such as a specific amount of rain within 24 hours or a temperature exceeding a certain threshold. We can start with a single policy type for initial implementation.

## 2. Frontend Integration - Policy Display

Next, we need to display these available policies on the step I previously showed [here](https://discord.com/channels/1378085040960831488/1378085040960831491/1385325761703186523). 

## 3. Smart Contract Interaction

When a user selects a policy, we’ll use the ethJS library on the client-side to request a signature from MetaMask. This signature will then be used to add the policy to our deployed smart contract. 

**Policy Data Structure:**

Each user-policy will be stored as an item in a data structure within the smart contract, looking something like this:

```solidity
// This is not final
    struct Policy {
        address user;
        string weatherXMLocation;
        uint payout;
        uint startTime;
        uint endTime;
        bool paid;
        int256 threshold;
        string eventType;
    }
```

## 4. Oracle Implementation

The oracle will be a crucial component. I envision a simple Rust program with access to the private keys that own the smart contract. It will:

*   Loop every 15 minutes.
*   Inspect the policies stored in the contract.
*   Retrieve data from WeatherXM.
*   Trigger payouts by calling a function in the contract when policy conditions are met.

## Database Usage

Currently, the primary purpose of the database is to track the types of policies available on the platform.  All other data can be stored directly within the smart contract. We can expand database usage for other features later.

## Development Order & Parallelization

The oracle project is relatively independent and can be developed in parallel with the backend, frontend, and database work. This is a great opportunity for @_extro to contribute.

## Additional Tasks

*   **"My Policies" Page:** Develop a page to display a user's currently active policies. We can leverage blockchain data for this feature.
