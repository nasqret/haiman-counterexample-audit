# Singular artifacts

`verify_minor.sing` independently reconstructs the specialized sparse matrix
from Lee's six row families and the centered-coordinate substitution. It checks
translation cancellation over the integers, the shape and 1410-entry support,
and the selected determinant over `GF(1000003)`.

Run from the repository root:

```bash
Singular -q artifacts/singular/verify_minor.sing
```

The final output line is machine-readable JSON. The script imports no matrix
entries from SageMath.
