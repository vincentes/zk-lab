# zk-vault-protocols

## Range Proof

### Statement

Given public field elements `min_age` and `max_age`, prove that you know a private field element `age` such that `min_age ≤ age ≤ max_age`. In other words, prove that a secret value lies within an inclusive range without revealing the value itself.

### Circuit

The core code is in `range.zok`. This is our circuit's definition using the Zokrates language.

```zokrates
def main(private field age, field min_age, field max_age) {
    assert(age >= min_age && age <= max_age);
    return;
}
```

### Prover
* Provides private `age` value and public range bounds `min_age` and `max_age`
* Prover shares `min_age`, `max_age`, result, and proof with verifier
* The actual `age` value remains private

### Verifier
Verifier can check that prover's secret value lies within the specified range, without learning the actual value.

### Use Cases
* Age verification (prove someone is over 18 without revealing exact age)
* Salary range verification (prove income falls within a bracket)
* Balance checks (prove account contains sufficient funds without revealing amount)

### Execution
```bash
# Compile the circuit
zokrates compile -i range.zok

# Generate proving and verification keys
zokrates setup

# Create witness from inputs
# Example: age=25, min_age=21, max_age=150
zokrates compute-witness -a 25 21 150

# Generate the proof
zokrates generate-proof

# Verify the proof
zokrates verify
```

### Security Considerations
* The range bounds (`min_age` and `max_age`) are public inputs
* The value being proven (`age`) must be kept private
* The proof reveals no information about the actual value beyond its inclusion in the range
* The circuit uses standard field arithmetic for comparisons

### Integration
This protocol can be integrated with smart contracts by:
1. Generating a Solidity verifier contract
2. Deploying the verifier contract
3. Calling the verify function with the generated proof

Use `zokrates export-verifier` to generate the Solidity verifier contract.