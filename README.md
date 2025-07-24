# VeriPact: DAO-Based Donation Platform on Internet Computer

VeriPact is a transparent and decentralized donation platform built on the Internet Computer Protocol. It allows donors to contribute funds and collectively govern how these funds are distributed through a Decentralized Autonomous Organization (DAO) structure.

## Key Features

- **Secure Fund Management**: Donations are stored in secure canisters on the blockchain.
- **Decentralized Governance**: Donors can vote on funding proposals based on their contribution amount.
- **Complete Transparency**: All transactions, votes, and disbursements are permanently recorded and publicly auditable.
- **Efficient Operations**: Leveraging Internet Computer's architecture for fast transactions with minimal fees.

## Architecture

### Backend (Rust Canister)

The backend is implemented as a Rust-based canister on the Internet Computer, handling all core functionality:

- **State Management**: Stores and manages donor information, proposals, and charity projects.
- **Donation Processing**: Handles the receipt and accounting of donations.
- **Governance System**: Manages the proposal creation, voting, and execution process.
- **Query Functions**: Provides data retrieval endpoints for the frontend.

### Frontend (React)

The frontend provides a user-friendly interface for donors and charity projects to interact with the platform.

## Technical Documentation

### Data Models

#### Donor
Represents a contributor to the platform.
```rust
pub struct Donor {
    pub id: Principal,                      // Unique Internet Identity of the donor
    pub total_donations: u64,               // Total amount donated (in e8s)
    pub voting_power: u64,                  // Voting power based on donations
    pub donation_history: Vec<DonationRecord>,  // Record of all donations
}
```

#### FundingProposal
Represents a request for funds from the collective treasury.
```rust
pub struct FundingProposal {
    pub id: u64,                        // Unique identifier
    pub creator: Principal,             // Principal who created the proposal
    pub title: String,                  // Brief title
    pub description: String,            // Detailed description
    pub recipient: Principal,           // Recipient of the funds
    pub amount: u64,                    // Amount requested (in e8s)
    pub status: ProposalStatus,         // Current status
    pub votes: Vec<Vote>,               // Record of votes
    pub yes_votes: u64,                 // Total voting power for yes
    pub no_votes: u64,                  // Total voting power for no
    pub created_at: u64,                // Creation timestamp
    pub expires_at: u64,                // Expiration timestamp
    pub executed_at: Option<u64>,       // Execution timestamp (if executed)
}
```

#### CharityProject
Represents a charitable project that can receive funds.
```rust
pub struct CharityProject {
    pub id: u64,                    // Unique identifier
    pub owner: Principal,           // Principal who created the project
    pub name: String,               // Project name
    pub description: String,        // Project description
    pub total_received: u64,        // Total amount received (in e8s)
    pub proposals: Vec<u64>,        // Associated proposals
    pub created_at: u64,            // Creation timestamp
}
```

### API Reference

#### Donation Management

```
donate() -> Result<String, String>
```
Processes a donation from the caller. In a production environment, this would interact with the ICP ledger.

#### Proposal Management

```
create_proposal(
    title: String,
    description: String,
    recipient: Principal,
    amount: u64,
    duration_seconds: u64
) -> Result<u64, String>
```
Creates a new funding proposal. Returns the proposal ID if successful.

```
vote_on_proposal(proposal_id: u64, approve: bool) -> Result<String, String>
```
Casts a vote on a proposal. The voting power is based on the donor's total donations.

```
execute_proposal(proposal_id: u64) -> Result<String, String>
```
Executes an approved proposal, transferring funds to the recipient.

#### Charity Project Management

```
register_charity(name: String, description: String) -> Result<u64, String>
```
Registers a new charitable project that can receive funds.

#### Query Functions

```
get_donor(id: Principal) -> Option<Donor>
```
Retrieves information about a specific donor.

```
get_proposal(id: u64) -> Option<FundingProposal>
```
Retrieves information about a specific proposal.

```
list_active_proposals() -> Vec<FundingProposal>
```
Lists all active (non-expired, non-executed) proposals.

```
list_all_proposals() -> Vec<FundingProposal>
```
Lists all proposals regardless of status.

```
get_charity_project(id: u64) -> Option<CharityProject>
```
Retrieves information about a specific charity project.

```
list_charity_projects() -> Vec<CharityProject>
```
Lists all registered charity projects.

```
get_treasury_balance() -> u64
```
Returns the current balance of the treasury.

```
get_governance_settings() -> GovernanceSettings
```
Returns the current governance settings.

### Governance

The platform's governance is configurable through the following parameters:

- **Minimum Proposal Duration**: The minimum time a proposal must remain active for voting.
- **Quorum Percentage**: The minimum percentage of total voting power that must participate for a vote to be valid.
- **Approval Threshold**: The minimum percentage of "yes" votes required for a proposal to pass.

Governance settings can be updated using:

```
update_governance_settings(
    min_duration: Option<u64>,
    quorum: Option<u8>,
    threshold: Option<u8>
) -> Result<String, String>
```

## Development

### Prerequisites

- [DFX SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/) version 0.18.0 or later
- Rust and Cargo
- Node.js and npm

### Setup

1. Clone the repository
2. Install dependencies:
   ```
   npm install
   ```
3. Start the local Internet Computer replica:
   ```
   dfx start --clean
   ```
4. Deploy the canisters:
   ```
   dfx deploy
   ```

## Frontend Integration Guide

Frontend developers can interact with the VeriPact backend using the following methods:

### Agent-js

```javascript
import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "./declarations/VeriPact_backend";

const agent = new HttpAgent();
const veriPactActor = Actor.createActor(idlFactory, {
  agent,
  canisterId: process.env.VERI_PACT_BACKEND_CANISTER_ID,
});

// Example: Get all active proposals
const activeProposals = await veriPactActor.list_active_proposals();
```

### React Hooks (using dfx-generated declarations)

```javascript
import { useCanister } from "@connect2ic/react";

function ProposalsList() {
  const [veriPactBackend] = useCanister("VeriPact_backend");
  const [proposals, setProposals] = useState([]);

  useEffect(() => {
    async function fetchProposals() {
      const activeProposals = await veriPactBackend.list_active_proposals();
      setProposals(activeProposals);
    }
    fetchProposals();
  }, [veriPactBackend]);

  return (
    <div>
      {proposals.map((proposal) => (
        <ProposalCard key={proposal.id} proposal={proposal} />
      ))}
    </div>
  );
}
```

## Security Considerations

- The backend implements proper authentication to ensure only legitimate users can interact with sensitive functions.
- The governance system includes quorum requirements to prevent attacks through low participation.
- Proposal execution is protected to ensure only approved proposals can be processed.

## Future Enhancements

- Integration with the ICP ledger for real token transfers
- Multi-signature requirements for large withdrawals
- Reputation system for charity projects
- Support for recurring donations
- Enhanced analytics and reporting features

## License

This project is licensed under the MIT License - see the LICENSE file for details.

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
