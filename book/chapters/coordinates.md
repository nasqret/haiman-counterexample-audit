# Ideal-projector coordinates

For an ideal projector

$$
P:\mathbb C[x_1,\ldots,x_d]\to\operatorname{span}(1,x_1,\ldots,x_d),
$$

write

$$
P(x_i x_j)=p_{0,ij}+\sum_{m=1}^{d}p_{m,ij}x_m,
\quad p_{m,ij}=p_{m,ji}.
$$


De Boor's identity turns commutation of multiplication into polynomial equations. When $a\notin\{i,j,k\}$, the coefficient required later is

$$
C(a;j,(i,k))
=\sum_{m=1}^{d}
\left(p_{m,ij}p_{a,km}-p_{m,kj}p_{a,im}\right).
$$


After a linear centering transformation, translations split off and the remaining coordinate space has dimension

$$
d\left(\binom{d+1}{2}-1\right).
$$

For $d=8$, this is $280$.

:::{admonition} Reconstruction rule
:class: definition
The implementation will generate $C$ first in the original symmetric $p$-coordinates and then apply the centering substitution. Matrix entries will be extracted from that polynomial identity. No matrix entry will be transcribed by hand from the article.
:::
