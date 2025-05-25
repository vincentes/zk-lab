import './App.css'
import { useState, useEffect } from 'react';
import { sha256 } from '@noble/hashes/sha256';
import { bytesToHex } from '@noble/hashes/utils';

function App() {
  const [email, setEmail] = useState('');
  const [commitments, setCommitments] = useState<string[]>([]);

  const fetchRegistry = async () => {
    try {
      const response = await fetch('http://localhost:3000/registry');
      const data = await response.json();
      setCommitments(data.commitments);
    } catch (error) {
      console.error('Error fetching registry:', error);
    }
  };

  useEffect(() => {
    fetchRegistry();
  }, []);

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const emailBytes = new TextEncoder().encode(email);
    const hash = sha256(emailBytes);
    const hashHex = bytesToHex(hash);
    console.log('Email:', email);
    console.log('SHA-256 Hash:', hashHex);

    try {
      await fetch('http://localhost:3000/registry', {
        method: 'POST',
        body: JSON.stringify({ commitment: hashHex }),
        headers: {
          'Content-Type': 'application/json',
        },
      });
      
      // Fetch updated registry after successful submission
      await fetchRegistry();
      
      // Clear the email input
      setEmail('');
    } catch (error) {
      console.error('Error submitting commitment:', error);
    }
  }

  return (
    <div className="container">
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor="email">Email</label>
          <input
            type="email"
            id="email"
            name="email"
            placeholder="Enter your email"
            required
            value={email}
            onChange={(e) => setEmail(e.target.value)}
          />
        </div>
        <button type="submit">Submit</button>
      </form>

      <div className="commitments-list">
        <h2>Registered Commitments</h2>
        <ul>
          {commitments.map((commitment, index) => (
            <li key={index}>{commitment}</li>
          ))}
        </ul>
      </div>
    </div>
  )
}

export default App
