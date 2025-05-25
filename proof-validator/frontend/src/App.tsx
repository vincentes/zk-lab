import { useState } from 'react'
import './App.css'

interface Proof {
  a: [string, string];
  b: [[string, string], [string, string]];
  c: [string, string];
}

interface ProofData {
  scheme: string;
  curve: string;
  proof: Proof;
  inputs: string[];
}

function App() {
  const [proofData, setProofData] = useState<ProofData>({
    scheme: 'g16',
    curve: 'bn128',
    proof: {
      a: ['', ''],
      b: [['', ''], ['', '']],
      c: ['', '']
    },
    inputs: ['']
  })
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [validationResult, setValidationResult] = useState<boolean | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [isJsonMode, setIsJsonMode] = useState(false)
  const [jsonInput, setJsonInput] = useState('')

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setIsSubmitting(true)
    setError(null)
    
    try {
      const dataToSubmit = isJsonMode ? JSON.parse(jsonInput) : proofData
      const response = await fetch('http://localhost:3000/verify', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(dataToSubmit),
      })
      
      const result = await response.json()
      setValidationResult(result.valid)
      if (!result.valid) {
        setError(result.message)
      }
    } catch (error) {
      if (error instanceof SyntaxError) {
        setError('Invalid JSON format')
      } else {
        setError('Failed to connect to verification server')
      }
      setValidationResult(false)
    } finally {
      setIsSubmitting(false)
    }
  }

  const updateProofField = (field: keyof Proof, index1: number, index2: number | null, value: string) => {
    setProofData(prev => {
      const newProof = { ...prev.proof }
      if (index2 === null) {
        (newProof[field] as string[])[index1] = value
      } else {
        (newProof[field] as string[][])[index1][index2] = value
      }
      return { ...prev, proof: newProof }
    })
  }

  const updateInput = (index: number, value: string) => {
    setProofData(prev => {
      const newInputs = [...prev.inputs]
      newInputs[index] = value
      return { ...prev, inputs: newInputs }
    })
  }

  const toggleInputMode = () => {
    if (isJsonMode) {
      try {
        const parsed = JSON.parse(jsonInput)
        setProofData(parsed)
      } catch {
        // If JSON is invalid, just switch modes without updating
      }
    } else {
      setJsonInput(JSON.stringify(proofData, null, 2))
    }
    setIsJsonMode(!isJsonMode)
  }

  return (
    <div className="container">
      <h1>ZK Proof Validator</h1>
      <div className="format-info">
        <h3>Expected Proof Format:</h3>
        <pre>
{`{
  "scheme": "g16",
  "curve": "bn128",
  "proof": {
    "a": ["0x...", "0x..."],
    "b": [["0x...", "0x..."], ["0x...", "0x..."]],
    "c": ["0x...", "0x..."]
  },
  "inputs": ["0x..."]
}`}
        </pre>
      </div>

      <div className="mode-toggle">
        <button 
          onClick={toggleInputMode}
          className="toggle-button"
        >
          Switch to {isJsonMode ? 'Form' : 'JSON'} Input
        </button>
      </div>

      <form onSubmit={handleSubmit} className="proof-form">
        {isJsonMode ? (
          <div className="form-group">
            <label>Proof JSON</label>
            <textarea
              value={jsonInput}
              onChange={(e) => setJsonInput(e.target.value)}
              placeholder="Paste your proof JSON here..."
              rows={10}
              required
            />
          </div>
        ) : (
          <>
            <div className="form-group">
              <label>Proof A</label>
              <div className="proof-inputs">
                <input
                  type="text"
                  value={proofData.proof.a[0]}
                  onChange={(e) => updateProofField('a', 0, null, e.target.value)}
                  placeholder="0x..."
                  required
                />
                <input
                  type="text"
                  value={proofData.proof.a[1]}
                  onChange={(e) => updateProofField('a', 1, null, e.target.value)}
                  placeholder="0x..."
                  required
                />
              </div>
            </div>

            <div className="form-group">
              <label>Proof B</label>
              <div className="proof-inputs">
                <div className="proof-row">
                  <input
                    type="text"
                    value={proofData.proof.b[0][0]}
                    onChange={(e) => updateProofField('b', 0, 0, e.target.value)}
                    placeholder="0x..."
                    required
                  />
                  <input
                    type="text"
                    value={proofData.proof.b[0][1]}
                    onChange={(e) => updateProofField('b', 0, 1, e.target.value)}
                    placeholder="0x..."
                    required
                  />
                </div>
                <div className="proof-row">
                  <input
                    type="text"
                    value={proofData.proof.b[1][0]}
                    onChange={(e) => updateProofField('b', 1, 0, e.target.value)}
                    placeholder="0x..."
                    required
                  />
                  <input
                    type="text"
                    value={proofData.proof.b[1][1]}
                    onChange={(e) => updateProofField('b', 1, 1, e.target.value)}
                    placeholder="0x..."
                    required
                  />
                </div>
              </div>
            </div>

            <div className="form-group">
              <label>Proof C</label>
              <div className="proof-inputs">
                <input
                  type="text"
                  value={proofData.proof.c[0]}
                  onChange={(e) => updateProofField('c', 0, null, e.target.value)}
                  placeholder="0x..."
                  required
                />
                <input
                  type="text"
                  value={proofData.proof.c[1]}
                  onChange={(e) => updateProofField('c', 1, null, e.target.value)}
                  placeholder="0x..."
                  required
                />
              </div>
            </div>

            <div className="form-group">
              <label>Inputs</label>
              <div className="proof-inputs">
                <input
                  type="text"
                  value={proofData.inputs[0]}
                  onChange={(e) => updateInput(0, e.target.value)}
                  placeholder="0x..."
                  required
                />
              </div>
            </div>
          </>
        )}
        
        <button 
          type="submit" 
          disabled={isSubmitting}
          className="submit-button"
        >
          {isSubmitting ? 'Validating...' : 'Validate Proof'}
        </button>

        {validationResult !== null && (
          <div className={`validation-result ${validationResult ? 'success' : 'error'}`}>
            {validationResult ? '✓ Proof is valid' : '✗ Proof is invalid'}
            {error && <div className="error-message">{error}</div>}
          </div>
        )}
      </form>
    </div>
  )
}

export default App
