Let $b(n)$ be the largest power of 2 that divides $n$. For example $b(24) = 8$.

Define the recursive function:
\begin{align*}
\begin{split}
A(0) &= 1\\
A(2n) &= 3A(n) + 5A\big(2n - b(n)\big)  \qquad n \gt 0\\
A(2n+1) &= A(n)
\end{split}
\end{align*}
and let $H(t,r) = A\big((2^t+1)^r\big)$.

You are given $H(3,2) = A(81) = 636056$.

Find $H(10^{14}+31,62)$. Give your answer modulo $1\,000\,062\,031$.