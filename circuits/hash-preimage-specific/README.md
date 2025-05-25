# zk-vault-protocols

## SHA-256 Preimage Proof

### Statement
Given a known SHA-256 hash value (split into two field elements), prove knowledge of a preimage consisting of four private field elements (a, b, c, d) that hash to this specific value, without revealing the preimage itself.

### Circuit Explanation
The code represents a zero-knowledge proof circuit written in ZoKrates that implements a SHA-256 preimage verification:

```zokrates
import "hashes/sha256/512bitPacked" as sha256packed;

def main (private field a, private field b, private field c, private field d) {
    field[2] h = sha256packed([a,b,c,d]);
    assert(h[0] ==  263561599766550617289250058199814760685);
    assert(h[1] == 65303172752238645975888084098459749904);
    return;
}
```

Let's break down how this works:

1. The `import` statement brings in a packed version of SHA-256 that's optimized for the ZoKrates circuit
2. The `main` function takes four private field elements as input - these form our preimage
3. The `sha256packed` function computes the hash of these four values
4. The result is compared against a known hash value split into two field elements
5. The `assert` statements verify that the computed hash matches the expected value

### Prover
* Knows the secret preimage values (a, b, c, d)
* Generates a zero-knowledge proof showing they know values that hash to the target
* Never reveals the actual preimage values in the process
* Shares only the proof and public inputs/outputs with the verifier

### Verifier
* Can verify the proof without learning the preimage
* Only learns that the prover knows some values that hash to the target
* Cannot determine what those values are
* Can be convinced the prover knows a valid preimage

### Use Cases
* Password verification without storing actual passwords
* Document commitment (prove you know a document without revealing it)
* Secret message verification
* Credential verification without revealing the credentials

### Execution
```bash
# Compile the circuit
zokrates compile -i preimage.zok

# Generate proving/verification keys
zokrates setup

# Create witness using actual preimage values
zokrates compute-witness -a <a> <b> <c> <d>

# Generate the proof
zokrates generate-proof

# Verify the proof
zokrates verify
```

### Security Considerations
* The preimage values must remain private
* The hash value is public and embedded in the circuit
* The proof reveals no information about the preimage beyond its existence
* SHA-256 collision resistance properties apply
* The field size must be large enough to represent the hash values

### Integration
This can be integrated with blockchain systems by:
1. Generating a Solidity verifier using `zokrates export-verifier`
2. Deploying the verifier contract
3. Calling verify with the generated proof
4. Using it as part of larger privacy-preserving protocols

The system allows for secure verification of knowledge of a preimage without revealing the preimage itself, making it useful for privacy-preserving applications where hash preimage verification is needed.