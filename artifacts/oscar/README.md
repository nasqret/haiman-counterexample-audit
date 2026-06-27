# Oscar.jl artifacts

`verify_minor.jl` independently reconstructs the specialized sparse matrix
from Lee's formulas, checks exact cancellation of translation terms, and uses
Oscar's finite-field matrices to verify rank and determinant.

Provision the pinned project environment once, then run the verifier:

```bash
julia --project=artifacts/oscar -e 'using Pkg; Pkg.instantiate()'
julia --project=artifacts/oscar artifacts/oscar/verify_minor.jl
```

The program prints Julia, Oscar, GAP, and Singular environment information and
finishes with one machine-readable JSON line.
