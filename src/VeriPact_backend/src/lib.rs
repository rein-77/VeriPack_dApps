use candid::{CandidType, Principal};
use ic_cdk::{api, caller, storage, trap};
use std::collections::{BTreeMap, HashMap};
use std::cell::RefCell;
use serde::{Deserialize, Serialize};

/// VeriPact DAO-Based Donation Platform
/// This is a decentralized platform for charitable donations with complete transparency
/// and collective governance by donors.

// ======== Type Definitions ========

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum ProposalStatus {
    Active,
    Approved,
    Rejected,
    Executed,
    Cancelled,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Donor {
    pub id: Principal,
    pub total_donations: u64,
    pub voting_power: u64,
    pub donation_history: Vec<DonationRecord>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct DonationRecord {
    pub timestamp: u64,
    pub amount: u64,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Vote {
    pub donor: Principal,
    pub voting_power: u64,
    pub approved: bool,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct FundingProposal {
    pub id: u64,
    pub creator: Principal,
    pub title: String,
    pub description: String,
    pub recipient: Principal,
    pub amount: u64,
    pub status: ProposalStatus,
    pub votes: Vec<Vote>,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub created_at: u64,
    pub expires_at: u64,
    pub executed_at: Option<u64>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct CharityProject {
    pub id: u64,
    pub owner: Principal,
    pub name: String,
    pub description: String,
    pub total_received: u64,
    pub proposals: Vec<u64>,
    pub created_at: u64,
}

// ======== State Management ========

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(CandidType, Deserialize, Serialize, Clone, Default)]
struct State {
    donors: HashMap<Principal, Donor>,
    proposals: BTreeMap<u64, FundingProposal>,
    charity_projects: BTreeMap<u64, CharityProject>,
    total_donations: u64,
    next_proposal_id: u64,
    next_project_id: u64,
    governance_settings: GovernanceSettings,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
struct GovernanceSettings {
    min_proposal_duration: u64,  // in seconds
    quorum_percentage: u8,       // percentage of total voting power required
    approval_threshold: u8,      // percentage of yes votes required for approval
}

impl Default for GovernanceSettings {
    fn default() -> Self {
        Self {
            min_proposal_duration: 86400 * 3,  // 3 days
            quorum_percentage: 25,            // 25% participation required
            approval_threshold: 51,           // >50% yes votes required
        }
    }
}

// ======== Helper Functions ========

fn get_timestamp() -> u64 {
    api::time()
}

fn assert_caller_not_anonymous() {
    if caller() == Principal::anonymous() {
        trap("Anonymous callers are not allowed");
    }
}

// ======== Donation Management ========

#[ic_cdk::update]
fn donate() -> Result<String, String> {
    assert_caller_not_anonymous();
    
    let donor = caller();
    // In a real implementation, this would interact with the ICP ledger
    // Here we'll simulate accepting tokens by simply tracking donations
    // The actual amount would be determined from the ICP transfer
    let amount: u64 = 100_000_000; // 1 ICP in e8s (simulation)
    
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let donor_entry = state.donors.entry(donor).or_insert(Donor {
            id: donor,
            total_donations: 0,
            voting_power: 0,
            donation_history: Vec::new(),
        });
        
        donor_entry.total_donations += amount;
        donor_entry.voting_power += amount;
        donor_entry.donation_history.push(DonationRecord {
            timestamp: get_timestamp(),
            amount,
        });
        
        state.total_donations += amount;
    });
    
    Ok(format!("Successfully donated {} e8s (1 ICP)", amount))
}

// ======== Proposal Management ========

#[ic_cdk::update]
fn create_proposal(
    title: String,
    description: String,
    recipient: Principal,
    amount: u64,
    duration_seconds: u64,
) -> Result<u64, String> {
    assert_caller_not_anonymous();
    
    let creator = caller();
    let now = get_timestamp();
    
    // Validate inputs
    if title.is_empty() || description.is_empty() {
        return Err("Title and description cannot be empty".to_string());
    }
    
    if amount == 0 {
        return Err("Amount must be greater than zero".to_string());
    }
    
    let mut duration = duration_seconds;
    
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        
        // Check if creator is a donor
        if !state.donors.contains_key(&creator) {
            return Err("Only donors can create proposals".to_string());
        }
        
        // Ensure minimum duration
        if duration < state.governance_settings.min_proposal_duration {
            duration = state.governance_settings.min_proposal_duration;
        }
        
        // Ensure enough funds in treasury
        if amount > state.total_donations {
            return Err("Requested amount exceeds available funds".to_string());
        }
        
        let proposal_id = state.next_proposal_id;
        state.next_proposal_id += 1;
        
        let proposal = FundingProposal {
            id: proposal_id,
            creator,
            title,
            description,
            recipient,
            amount,
            status: ProposalStatus::Active,
            votes: Vec::new(),
            yes_votes: 0,
            no_votes: 0,
            created_at: now,
            expires_at: now + duration,
            executed_at: None,
        };
        
        state.proposals.insert(proposal_id, proposal);
        
        Ok(proposal_id)
    })
}

#[ic_cdk::update]
fn vote_on_proposal(proposal_id: u64, approve: bool) -> Result<String, String> {
    assert_caller_not_anonymous();
    
    let donor = caller();
    let now = get_timestamp();
    
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        
        // Check if donor exists
        let donor_data = match state.donors.get(&donor) {
            Some(d) => d,
            None => return Err("Only donors can vote".to_string()),
        };
        
        let voting_power = donor_data.voting_power;
        
        // Get governance settings before mutable operations
        let quorum_percentage = state.governance_settings.quorum_percentage;
        let approval_threshold = state.governance_settings.approval_threshold;
        
        // Calculate total voting power before mutable operations
        let total_voting_power: u64 = state.donors.values().map(|d| d.voting_power).sum();
        
        // Check if proposal exists
        let proposal = match state.proposals.get_mut(&proposal_id) {
            Some(p) => p,
            None => return Err("Proposal not found".to_string()),
        };
        
        // Validate proposal status
        if proposal.status != ProposalStatus::Active {
            return Err("Proposal is not active".to_string());
        }
        
        // Check if proposal has expired
        if now > proposal.expires_at {
            proposal.status = ProposalStatus::Rejected;
            return Err("Proposal has expired".to_string());
        }
        
        // Check if donor has already voted
        if proposal.votes.iter().any(|v| v.donor == donor) {
            return Err("Already voted on this proposal".to_string());
        }
        
        // Record vote
        proposal.votes.push(Vote {
            donor,
            voting_power,
            approved: approve,
            timestamp: now,
        });
        
        // Update vote counts
        if approve {
            proposal.yes_votes += voting_power;
        } else {
            proposal.no_votes += voting_power;
        }
        
        // Check if proposal can be decided now
        let quorum_threshold = (total_voting_power * quorum_percentage as u64) / 100;
        let total_votes = proposal.yes_votes + proposal.no_votes;
        
        if total_votes >= quorum_threshold {
            let approval_percentage = (proposal.yes_votes * 100) / total_votes;
            
            if approval_percentage >= approval_threshold as u64 {
                proposal.status = ProposalStatus::Approved;
            } else {
                proposal.status = ProposalStatus::Rejected;
            }
        }
        
        Ok(format!("Vote recorded successfully"))
    })
}

#[ic_cdk::update]
fn execute_proposal(proposal_id: u64) -> Result<String, String> {
    assert_caller_not_anonymous();
    
    let now = get_timestamp();
    
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        
        // Get treasury balance before mutable operations
        let treasury_balance = state.total_donations;
        
        // Find proposal
        let proposal = match state.proposals.get_mut(&proposal_id) {
            Some(p) => p,
            None => return Err("Proposal not found".to_string()),
        };
        
        // Validate proposal status
        if proposal.status != ProposalStatus::Approved {
            return Err("Only approved proposals can be executed".to_string());
        }
        
        // Verify funds available
        if proposal.amount > treasury_balance {
            proposal.status = ProposalStatus::Rejected;
            return Err("Insufficient funds in treasury".to_string());
        }
        
        // In a real implementation, this would transfer ICP to the recipient
        // through the ledger canister
        
        // Update proposal status
        proposal.status = ProposalStatus::Executed;
        proposal.executed_at = Some(now);
        
        // Update total donations (treasury)
        state.total_donations -= proposal.amount;
        
        Ok(format!("Proposal executed successfully"))
    })
}

// ======== Charity Project Management ========

#[ic_cdk::update]
fn register_charity(name: String, description: String) -> Result<u64, String> {
    assert_caller_not_anonymous();
    
    let owner = caller();
    let now = get_timestamp();
    
    // Validate inputs
    if name.is_empty() || description.is_empty() {
        return Err("Name and description cannot be empty".to_string());
    }
    
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        
        let project_id = state.next_project_id;
        state.next_project_id += 1;
        
        let project = CharityProject {
            id: project_id,
            owner,
            name,
            description,
            total_received: 0,
            proposals: Vec::new(),
            created_at: now,
        };
        
        state.charity_projects.insert(project_id, project);
        
        Ok(project_id)
    })
}

// ======== Query Functions ========

#[ic_cdk::query]
fn get_donor(id: Principal) -> Option<Donor> {
    STATE.with(|state| {
        let state = state.borrow();
        state.donors.get(&id).cloned()
    })
}

#[ic_cdk::query]
fn get_proposal(id: u64) -> Option<FundingProposal> {
    STATE.with(|state| {
        let state = state.borrow();
        state.proposals.get(&id).cloned()
    })
}

#[ic_cdk::query]
fn list_active_proposals() -> Vec<FundingProposal> {
    STATE.with(|state| {
        let state = state.borrow();
        state.proposals
            .values()
            .filter(|p| p.status == ProposalStatus::Active)
            .cloned()
            .collect()
    })
}

#[ic_cdk::query]
fn list_all_proposals() -> Vec<FundingProposal> {
    STATE.with(|state| {
        let state = state.borrow();
        state.proposals
            .values()
            .cloned()
            .collect()
    })
}

#[ic_cdk::query]
fn get_charity_project(id: u64) -> Option<CharityProject> {
    STATE.with(|state| {
        let state = state.borrow();
        state.charity_projects.get(&id).cloned()
    })
}

#[ic_cdk::query]
fn list_charity_projects() -> Vec<CharityProject> {
    STATE.with(|state| {
        let state = state.borrow();
        state.charity_projects
            .values()
            .cloned()
            .collect()
    })
}

#[ic_cdk::query]
fn get_treasury_balance() -> u64 {
    STATE.with(|state| {
        let state = state.borrow();
        state.total_donations
    })
}

#[ic_cdk::query]
fn get_governance_settings() -> GovernanceSettings {
    STATE.with(|state| {
        let state = state.borrow();
        state.governance_settings.clone()
    })
}

// ======== Admin Functions ========

#[ic_cdk::update]
fn update_governance_settings(
    min_duration: Option<u64>,
    quorum: Option<u8>,
    threshold: Option<u8>,
) -> Result<String, String> {
    assert_caller_not_anonymous();
    
    // In a real implementation, this would check if caller is authorized
    // through a proper role-based system or DAO voting
    
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        
        if let Some(duration) = min_duration {
            state.governance_settings.min_proposal_duration = duration;
        }
        
        if let Some(quorum_pct) = quorum {
            if quorum_pct > 100 {
                return Err("Quorum percentage cannot exceed 100".to_string());
            }
            state.governance_settings.quorum_percentage = quorum_pct;
        }
        
        if let Some(approval_pct) = threshold {
            if approval_pct > 100 {
                return Err("Approval threshold cannot exceed 100".to_string());
            }
            state.governance_settings.approval_threshold = approval_pct;
        }
        
        Ok("Governance settings updated successfully".to_string())
    })
}

// ======== System Management ========

#[ic_cdk::init]
fn init() {
    STATE.with(|state| {
        *state.borrow_mut() = State::default();
    });
    
    ic_cdk::println!("VeriPact DAO-Based Donation Platform initialized");
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| {
        let state_data = state.borrow();
        storage::stable_save((&*state_data,)).expect("Failed to save state");
    });
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    STATE.with(|state| {
        let state_data = storage::stable_restore::<(State,)>().expect("Failed to restore state");
        *state.borrow_mut() = state_data.0;
    });
    
    ic_cdk::println!("VeriPact DAO-Based Donation Platform upgraded");
}
