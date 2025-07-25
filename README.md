# VeriPact: Transparent DAO-Based Donation Platform

**VeriPact** is a transparent and decentralized donation platform built on the Internet Computer Protocol (ICP). This platform allows donors to contribute funds and collectively decide how those funds are distributed through a Decentralized Autonomous Organization (DAO) structure.

## 🎯 Name and Philosophy

**VeriPact** is a combination of:
- **"Veritas"** (Latin): Truth and honesty
- **"Pact"**: A binding agreement or contract

This name communicates the two main values of the application: **verifiable truth** and **binding agreements** between donors and recipients, creating a trustworthy donation ecosystem.

## ✨ Key Features

- 🔒 **Fund Security**: Donations are stored securely in blockchain canisters
- 🗳️ **Decentralized Governance**: Donors have voting rights based on their contribution amounts
- 📊 **Absolute Transparency**: All transactions, voting, and fund disbursements are permanently recorded and auditable
- ⚡ **Efficient Operations**: Leveraging Internet Computer architecture for fast transactions with minimal costs
- 🌐 **Modern Interface**: Elegant and user-friendly frontend with minimalist design

## 🏗️ Platform Architecture

### 🦀 Backend (Rust Canister)

The backend is implemented as a Rust-based canister on the Internet Computer, handling all core functionalities:

- **📦 State Management**: Stores and manages donor information, proposals, and charity projects
- **💰 Donation Processing**: Handles donation reception and accounting
- **🏛️ Governance System**: Manages proposal creation, voting processes, and execution
- **🔍 Query Functions**: Provides data retrieval endpoints for the frontend

### ⚛️ Frontend (React)

The frontend provides a user-friendly interface with a modern and minimalist design:

- **🎨 Modern Design**: Professional color palette with clean typography
- **📱 Responsive Layout**: Optimized for all devices (desktop, tablet, mobile)
- **🗂️ Tab Navigation**: Dashboard, Donate, Proposals, and Create Proposal
- **✨ Interactive Elements**: Smooth animations and real-time feedback

## 🚀 Demo and Access

- **🌐 Frontend**: http://127.0.0.1:4943/?canisterId=u6s2n-gx777-77774-qaaba-cai
- **🔧 Backend Candid**: http://127.0.0.1:4943/?canisterId=u6s2n-gx777-77774-qaaba-cai&id=uxrrr-q7777-77774-qaaaq-cai

## 📋 Technical Documentation

### 🏗️ Data Model

#### 👤 Donor
Represents a contributor to the donation platform.
```rust
pub struct Donor {
    pub id: Principal,                      // Unique Internet Identity of the donor
    pub total_donations: u64,               // Total donation amount (in e8s)
    pub voting_power: u64,                  // Voting power based on donations
    pub donation_history: Vec<DonationRecord>,  // History of all donations
}
```

#### 📝 FundingProposal
Represents a funding request from the collective treasury.
```rust
pub struct FundingProposal {
    pub id: u64,                        // Unique identifier
    pub creator: Principal,             // Principal who created the proposal
    pub title: String,                  // Short title
    pub description: String,            // Detailed description
    pub recipient: Principal,           // Fund recipient
    pub amount: u64,                    // Requested amount (in e8s)
    pub status: ProposalStatus,         // Current status
    pub votes: Vec<Vote>,               // Voting records
    pub yes_votes: u64,                 // Total voting power for approval
    pub no_votes: u64,                  // Total voting power for rejection
    pub created_at: u64,                // Creation timestamp
    pub expires_at: u64,                // Expiration timestamp
    pub executed_at: Option<u64>,       // Execution timestamp (if executed)
}
```

#### 🏥 CharityProject
Represents a charity project eligible to receive funds.
```rust
pub struct CharityProject {
    pub id: u64,                    // Unique identifier
    pub owner: Principal,           // Principal who created the project
    pub name: String,               // Project name
    pub description: String,        // Project description
    pub total_received: u64,        // Total amount received (in e8s)
    pub proposals: Vec<u64>,        // Related proposals
    pub created_at: u64,            // Creation timestamp
}
```

### 🔌 API Reference

#### 💰 Donation Management

```rust
donate() -> Result<String, String>
```
Processes a donation from the caller. In a production environment, this will interact with the ICP ledger.

#### 📋 Proposal Management

```rust
create_proposal(
    title: String,
    description: String,
    recipient: Principal,
    amount: u64,
    duration_seconds: u64
) -> Result<u64, String>
```
Creates a new funding proposal. Returns the proposal ID if successful.

```rust
vote_on_proposal(proposal_id: u64, approve: bool) -> Result<String, String>
```
Votes on a proposal. Voting power is based on the donor's total donations.

```rust
execute_proposal(proposal_id: u64) -> Result<String, String>
```
Executes an approved proposal, transferring funds to the recipient.

#### 🏥 Charity Project Management

```rust
register_charity(name: String, description: String) -> Result<u64, String>
```
Registers a new charity project eligible to receive funds.

#### 🔍 Query Functions

```rust
get_donor(id: Principal) -> Option<Donor>
```
Retrieves information about a specific donor.

```rust
list_active_proposals() -> Vec<FundingProposal>
```
Lists all active proposals (not expired, not executed).

```rust
get_treasury_balance() -> u64
```
Returns the current treasury balance.

```rust
get_governance_settings() -> GovernanceSettings
```
Returns the current governance settings.

### 🏛️ Governance System

The platform has a governance system configurable through the following parameters:

- **⏰ Minimum Proposal Duration**: Minimum time a proposal must remain active for voting
- **📊 Quorum Percentage**: Minimum percentage of total voting power required for a valid vote
- **✅ Approval Threshold**: Minimum percentage of "yes" votes required for a proposal to pass

Governance settings can be updated using:

```rust
update_governance_settings(
    min_duration: Option<u64>,
    quorum: Option<u8>,
    threshold: Option<u8>
) -> Result<String, String>
```

## 🚀 Development

### 📋 Prerequisites

- [DFX SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/) version 0.18.0 or newer
- Rust and Cargo
- Node.js and npm

### ⚙️ Development Setup

1. Clone this repository
2. Install dependencies:
   ```bash
   npm install
   ```
3. Install Rust target for WebAssembly:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
4. Start the local Internet Computer replica:
   ```bash
   dfx start --clean
   ```
5. Deploy canisters:
   ```bash
   dfx deploy
   ```

### 🧪 Backend Testing

```bash
# Test donation
dfx canister call VeriPact_backend donate

# Test proposal creation
dfx canister call VeriPact_backend create_proposal '("Help Local School", "Funding for computers", principal "rrkah-fqaaa-aaaaa-aaaaq-cai", 50000000, 86400)'

# Test voting
dfx canister call VeriPact_backend vote_on_proposal '(0, true)'

# Check treasury balance
dfx canister call VeriPact_backend get_treasury_balance
```

## 🔗 Frontend Integration Guide

Frontend developers can interact with the VeriPact backend using the following methods:

### 📦 Using Agent-js

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

### ⚛️ React Hooks (using dfx-generated declarations)

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

## 🔐 Security Considerations

- The backend implements proper authentication to ensure only legitimate users can interact with sensitive functions
- The governance system includes quorum requirements to prevent attacks through low participation
- Proposal execution is protected to ensure only approved proposals can be processed
- All transactions are permanently recorded on the blockchain for a complete audit trail

## 🚀 Future Enhancements

- 🔗 Integration with the ICP ledger for real token transfers
- 🔏 Multi-signature requirements for large withdrawals
- ⭐ Reputation system for charity projects
- 📅 Support for recurring donations
- 📈 Enhanced analytics and reporting features
- 🌙 Dark mode for the frontend
- 📱 Native mobile application
- 🌍 Multi-language support (including English)

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

---

## 🎉 About VeriPact

VeriPact is the perfect representation of how blockchain technology can be used to create transparency and trust in the world of charity and donations. By combining **"Veritas"** (truth) and **"Pact"** (agreement), this platform bridges the gap between donors and recipients through a transparent and democratic governance system.

**VeriPact - Where Truth Meets Trust in Charitable Giving** 🤝✨
