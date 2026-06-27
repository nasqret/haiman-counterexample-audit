# Magma artifacts

`verify_minor.m` is designed for `/opt/magma/current/magma` on
`lts-faculty.wmi.amu.edu.pl`. It reconstructs the matrix from index formulas,
checks exact translation cancellation and support size, and verifies the same
finite-field minor certificate as the other implementations.

Run from a synchronized repository checkout:

```bash
/opt/magma/current/magma -b artifacts/magma/verify_minor.m
```

It imports no SageMath matrix entries; only the canonical assignment and pivot
certificate values are duplicated.
