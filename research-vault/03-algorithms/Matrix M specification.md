# Matrix M specification

## Row order

For each \(a=1,2,3\), concatenate the following tuples \((a,j,i,k)\) lexicographically within each family:

1. \(4\le j<i<k\le8\): 10 tuples.
2. \(4\le k<j<i\le8\): 10 tuples.
3. \(4=i=j<k\le8\): 4 tuples.
4. \(4=i<j=k\le8\): 4 tuples.
5. \(5=i=j<k\le6\): 1 tuple.
6. \(5=i<j=k\le6\): 1 tuple.

Total: \(3(10+10+4+4+1+1)=90\).

## Column order

Use triples \((r,s,t)\), with \(s\le t\), in two lexicographic blocks:

1. \(1\le r,s\le3<t\le8\): \(3\cdot3\cdot5=45\).
2. \(4\le r\le8,\ 4\le s\le t\le8\), excluding \(r=s=t\): \(5\cdot15-5=70\).

Total: \(45+70=115\).

The centered column coordinate printed by Lee is
\[
z_{r,st}=p_{r,st}
-\frac{\delta_{r,t}}2p_{s,ss}
-\frac{\delta_{r,s}}2p_{t,tt}.
\]

Any implementation must emit these manifests before numerical linear algebra. Ordering differences are harmless only if serialized explicitly.
