Let $S_n$ be an integer sequence produced with the following pseudo-random number generator:
\begin{align}
S_0 & = 290797 \\
S_{n+1} & = S_n^2 \bmod 50515093
\end{align}


Let $A(i, j)$ be the minimum of the numbers $S_i, S_{i+1}, \dots, S_j$ for $i\le j$.
Let $M(N) = \sum A(i, j)$ for $1 \le i \le j \le N$.
We can verify that $M(10) = 432256955$ and $M(10\,000) = 3264567774119$.

Find $M(2\,000\,000\,000)$.