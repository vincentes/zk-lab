# zk-vault-protocols

## SHA-256 Hash Circuit

### Statement
Create a zero-knowledge circuit that computes the SHA-256 hash of four private field elements and returns the resulting hash as two field elements, without revealing the input values.

### Circuit Explanation
The code implements a circuit in ZoKrates that produces a SHA-256 hash from private inputs:

```zokrates
import "hashes/sha256/512bitPacked" as sha256packed;

def main (private field a, private field b, private field c, private field d) -> field[2] {
    field[2] h = sha256packed([a,b,c,d]);
    return h;
}
```

Let's break down how this works:

1. The `import` statement imports the optimized SHA-256 implementation for ZoKrates
2. The `main` function takes four private field elements as input
3. These inputs are packed and hashed using `sha256packed`
4. Unlike the previous circuit, this one returns the hash value instead of asserting equality
5. The return type `field[2]` indicates we return two field elements representing the hash

### Prover
* Provides the secret input values (a, b, c, d)
* Gets back the hash of these values
* Can generate a proof that they know inputs producing this hash
* The input values remain private while the hash becomes public

### Verifier
* Receives the hash output (two field elements)
* Can verify the proof of computation
* Learns the hash value but not the input values
* Can verify the prover correctly computed the hash

### Use Cases
* Commitment schemes (commit to values without revealing them)
* Hash-based signatures
* Creating verifiable commitments to secret data
* Building blocks for larger zero-knowledge protocols
* Document timestamping without revealing content

### Execution
```bash
# Compile the circuit
zokrates compile -i hash.zok

# Generate proving/verification keys
zokrates setup

# Create witness and get hash output
zokrates compute-witness -a <a> <b> <c> <d>

# Generate the proof
zokrates generate-proof

# Verify the proof
zokrates verify
```

### Security Considerations
* Input values are kept private
* The hash output is public
* The proof verifies correct computation
* SHA-256 properties (collision resistance, etc.) apply
* Field arithmetic must be able to handle the hash output values
* No assertions mean this circuit only proves correct computation

### Integration
This circuit can be integrated into larger systems:
1. Generate Solidity verifier contract using `zokrates export-verifier`
2. Deploy the verifier
3. Use the hash output as a commitment
4. Verify proofs of knowledge of the preimage
5. Chain with other circuits that use the hash output

The key difference from the previous circuit is that this one is a building block - it produces a hash output rather than verifying against a known hash. This makes it more flexible for use in larger protocols where you need to create and later verify commitments to secret values.