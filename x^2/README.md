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

