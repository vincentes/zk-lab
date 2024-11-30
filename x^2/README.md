# zk-vault-protocols

## x^2

### Statement

Given a field element `b`, prove that you know a field element `a` such that `a^2 = b`. In other words, prove knowledge of the square root of `b`.

### Circuit
The core code is in `root.zok`. This is our circuit's definition in using the Zokrates language.

```zokrates
def main (private field a, field b) {
    assert(a * a == b);
    return;
}
```
### Prover
* Provides `a` and `b`, calculates the result of the circuit. 1 if valid, 0 if not.
* Prover shares `b`, result, and proof with verifier.

### Verifier
Verifier can check that prover knew square root of `b`, without learning the square root.

### Execution
`zokrates compile -i root.zok`

`zokrates setup`

`zokrates compute-witness -a <a> -b <b>`

`zokrates generate-proof`

`zokrates verify`