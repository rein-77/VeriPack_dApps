import { useState, useEffect } from 'react';
import { VeriPact_backend } from 'declarations/VeriPact_backend';
import './App.css';

function App() {
  const [activeTab, setActiveTab] = useState('dashboard');
  const [treasuryBalance, setTreasuryBalance] = useState(0);
  const [proposals, setProposals] = useState([]);
  const [donors, setDonors] = useState([]);
  const [loading, setLoading] = useState(false);
  const [userPrincipal, setUserPrincipal] = useState(null);

  // States for forms
  const [donationAmount, setDonationAmount] = useState('');
  const [proposalForm, setProposalForm] = useState({
    title: '',
    description: '',
    recipient: '',
    amount: '',
    duration: '3'
  });

  useEffect(() => {
    loadDashboardData();
  }, []);

  const loadDashboardData = async () => {
    try {
      setLoading(true);
      const balance = await VeriPact_backend.get_treasury_balance();
      const activeProposals = await VeriPact_backend.list_active_proposals();
      
      setTreasuryBalance(Number(balance));
      setProposals(activeProposals);
    } catch (error) {
      console.error('Error loading data:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleDonate = async () => {
    if (!donationAmount) return;
    
    try {
      setLoading(true);
      const result = await VeriPact_backend.donate();
      if (result.Ok) {
        alert('Donation successful!');
        setDonationAmount('');
        await loadDashboardData();
      } else {
        alert('Donation failed: ' + result.Err);
      }
    } catch (error) {
      console.error('Error donating:', error);
      alert('Error processing donation');
    } finally {
      setLoading(false);
    }
  };

  const handleCreateProposal = async (e) => {
    e.preventDefault();
    
    try {
      setLoading(true);
      const result = await VeriPact_backend.create_proposal(
        proposalForm.title,
        proposalForm.description,
        proposalForm.recipient,
        BigInt(proposalForm.amount * 100000000), // Convert to e8s
        BigInt(proposalForm.duration * 86400) // Convert days to seconds
      );
      
      if (result.Ok) {
        alert('Proposal created successfully!');
        setProposalForm({
          title: '',
          description: '',
          recipient: '',
          amount: '',
          duration: '3'
        });
        await loadDashboardData();
      } else {
        alert('Failed to create proposal: ' + result.Err);
      }
    } catch (error) {
      console.error('Error creating proposal:', error);
      alert('Error creating proposal');
    } finally {
      setLoading(false);
    }
  };

  const handleVote = async (proposalId, approve) => {
    try {
      setLoading(true);
      const result = await VeriPact_backend.vote_on_proposal(proposalId, approve);
      
      if (result.Ok) {
        alert(`Vote ${approve ? 'approved' : 'rejected'} successfully!`);
        await loadDashboardData();
      } else {
        alert('Vote failed: ' + result.Err);
      }
    } catch (error) {
      console.error('Error voting:', error);
      alert('Error processing vote');
    } finally {
      setLoading(false);
    }
  };

  const formatICP = (amount) => {
    return (Number(amount) / 100000000).toFixed(4);
  };

  const formatDate = (timestamp) => {
    return new Date(Number(timestamp) / 1000000).toLocaleDateString();
  };

  return (
    <div className="app">
      {/* Header */}
      <header className="header">
        <div className="container">
          <div className="logo">
            <img src="/logo2.svg" alt="VeriPact" className="logo-icon" />
            <h1>VeriPact</h1>
            <span className="tagline">Transparent Charity DAO</span>
          </div>
          <nav className="nav">
            <button 
              className={activeTab === 'dashboard' ? 'nav-btn active' : 'nav-btn'}
              onClick={() => setActiveTab('dashboard')}
            >
              Dashboard
            </button>
            <button 
              className={activeTab === 'donate' ? 'nav-btn active' : 'nav-btn'}
              onClick={() => setActiveTab('donate')}
            >
              Donate
            </button>
            <button 
              className={activeTab === 'proposals' ? 'nav-btn active' : 'nav-btn'}
              onClick={() => setActiveTab('proposals')}
            >
              Proposals
            </button>
            <button 
              className={activeTab === 'create' ? 'nav-btn active' : 'nav-btn'}
              onClick={() => setActiveTab('create')}
            >
              Create Proposal
            </button>
          </nav>
        </div>
      </header>

      {/* Main Content */}
      <main className="main">
        <div className="container">
          {loading && <div className="loading">Processing...</div>}
          
          {/* Dashboard Tab */}
          {activeTab === 'dashboard' && (
            <div className="dashboard">
              <div className="stats-grid">
                <div className="stat-card">
                  <h3>Treasury Balance</h3>
                  <p className="stat-value">{formatICP(treasuryBalance)} ICP</p>
                </div>
                <div className="stat-card">
                  <h3>Active Proposals</h3>
                  <p className="stat-value">{proposals.length}</p>
                </div>
                <div className="stat-card">
                  <h3>Total Proposals</h3>
                  <p className="stat-value">{proposals.length}</p>
                </div>
              </div>
              
              <div className="recent-activity">
                <h2>Recent Proposals</h2>
                <div className="proposals-list">
                  {proposals.slice(0, 3).map((proposal) => (
                    <div key={proposal.id} className="proposal-card mini">
                      <h4>{proposal.title}</h4>
                      <p>{proposal.description.substring(0, 100)}...</p>
                      <div className="proposal-meta">
                        <span>Amount: {formatICP(proposal.amount)} ICP</span>
                        <span>Status: {Object.keys(proposal.status)[0]}</span>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          )}

          {/* Donate Tab */}
          {activeTab === 'donate' && (
            <div className="donate-section">
              <div className="donate-card">
                <h2>Make a Donation</h2>
                <p>Join our transparent charity DAO and help make a difference</p>
                
                <div className="donation-form">
                  <div className="input-group">
                    <label>Donation Amount (ICP)</label>
                    <input
                      type="number"
                      value={donationAmount}
                      onChange={(e) => setDonationAmount(e.target.value)}
                      placeholder="Enter amount"
                      min="0"
                      step="0.01"
                    />
                  </div>
                  
                  <div className="donation-info">
                    <div className="info-item">
                      <span>🔒</span>
                      <div>
                        <strong>Secure & Transparent</strong>
                        <p>All donations are stored securely on the blockchain</p>
                      </div>
                    </div>
                    <div className="info-item">
                      <span>🗳️</span>
                      <div>
                        <strong>Voting Rights</strong>
                        <p>Your donation gives you voting power in fund allocation</p>
                      </div>
                    </div>
                    <div className="info-item">
                      <span>📊</span>
                      <div>
                        <strong>Full Transparency</strong>
                        <p>Track every transaction and vote on the blockchain</p>
                      </div>
                    </div>
                  </div>
                  
                  <button 
                    className="btn btn-primary"
                    onClick={handleDonate}
                    disabled={!donationAmount || loading}
                  >
                    Donate Now
                  </button>
                </div>
              </div>
            </div>
          )}

          {/* Proposals Tab */}
          {activeTab === 'proposals' && (
            <div className="proposals-section">
              <h2>Active Proposals</h2>
              <div className="proposals-grid">
                {proposals.map((proposal) => (
                  <div key={proposal.id} className="proposal-card">
                    <div className="proposal-header">
                      <h3>{proposal.title}</h3>
                      <span className={`status ${Object.keys(proposal.status)[0].toLowerCase()}`}>
                        {Object.keys(proposal.status)[0]}
                      </span>
                    </div>
                    
                    <p className="proposal-description">{proposal.description}</p>
                    
                    <div className="proposal-details">
                      <div className="detail-item">
                        <strong>Amount:</strong> {formatICP(proposal.amount)} ICP
                      </div>
                      <div className="detail-item">
                        <strong>Created:</strong> {formatDate(proposal.created_at)}
                      </div>
                      <div className="detail-item">
                        <strong>Expires:</strong> {formatDate(proposal.expires_at)}
                      </div>
                    </div>
                    
                    <div className="voting-section">
                      <div className="vote-stats">
                        <div className="vote-bar">
                          <div className="yes-votes" style={{width: `${(Number(proposal.yes_votes) / (Number(proposal.yes_votes) + Number(proposal.no_votes)) || 0) * 100}%`}}></div>
                        </div>
                        <div className="vote-counts">
                          <span>Yes: {formatICP(proposal.yes_votes)}</span>
                          <span>No: {formatICP(proposal.no_votes)}</span>
                        </div>
                      </div>
                      
                      <div className="vote-buttons">
                        <button 
                          className="btn btn-success"
                          onClick={() => handleVote(proposal.id, true)}
                          disabled={loading}
                        >
                          Vote Yes
                        </button>
                        <button 
                          className="btn btn-danger"
                          onClick={() => handleVote(proposal.id, false)}
                          disabled={loading}
                        >
                          Vote No
                        </button>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
              
              {proposals.length === 0 && (
                <div className="empty-state">
                  <h3>No Active Proposals</h3>
                  <p>Be the first to create a proposal!</p>
                </div>
              )}
            </div>
          )}

          {/* Create Proposal Tab */}
          {activeTab === 'create' && (
            <div className="create-section">
              <div className="create-card">
                <h2>Create New Proposal</h2>
                <p>Submit a funding request to the DAO community</p>
                
                <form onSubmit={handleCreateProposal} className="proposal-form">
                  <div className="input-group">
                    <label>Title *</label>
                    <input
                      type="text"
                      value={proposalForm.title}
                      onChange={(e) => setProposalForm({...proposalForm, title: e.target.value})}
                      placeholder="Brief title for your proposal"
                      required
                    />
                  </div>
                  
                  <div className="input-group">
                    <label>Description *</label>
                    <textarea
                      value={proposalForm.description}
                      onChange={(e) => setProposalForm({...proposalForm, description: e.target.value})}
                      placeholder="Detailed description of how funds will be used"
                      rows="4"
                      required
                    />
                  </div>
                  
                  <div className="input-group">
                    <label>Recipient Principal *</label>
                    <input
                      type="text"
                      value={proposalForm.recipient}
                      onChange={(e) => setProposalForm({...proposalForm, recipient: e.target.value})}
                      placeholder="Principal ID of the fund recipient"
                      required
                    />
                  </div>
                  
                  <div className="form-row">
                    <div className="input-group">
                      <label>Amount (ICP) *</label>
                      <input
                        type="number"
                        value={proposalForm.amount}
                        onChange={(e) => setProposalForm({...proposalForm, amount: e.target.value})}
                        placeholder="0.00"
                        min="0"
                        step="0.01"
                        required
                      />
                    </div>
                    
                    <div className="input-group">
                      <label>Duration (Days) *</label>
                      <select
                        value={proposalForm.duration}
                        onChange={(e) => setProposalForm({...proposalForm, duration: e.target.value})}
                      >
                        <option value="1">1 Day</option>
                        <option value="3">3 Days</option>
                        <option value="7">7 Days</option>
                        <option value="14">14 Days</option>
                        <option value="30">30 Days</option>
                      </select>
                    </div>
                  </div>
                  
                  <button 
                    type="submit" 
                    className="btn btn-primary"
                    disabled={loading}
                  >
                    Create Proposal
                  </button>
                </form>
              </div>
            </div>
          )}
        </div>
      </main>

      {/* Footer */}
      <footer className="footer">
        <div className="container">
          <p>&copy; 2025 VeriPact - Transparent Charity DAO on Internet Computer</p>
        </div>
      </footer>
    </div>
  );
}

export default App;
