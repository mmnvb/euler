Define
$$\displaystyle g(m,n) = (m\oplus n)+(m\vee n)+(m\wedge n)$$
where $\oplus, \vee, \wedge$ are the bitwise XOR, OR and AND operator respectively.
Also set
$$\displaystyle G(N) = \sum_{n=0}^N\sum_{k=0}^n g(k,n-k)$$
For example, $G(10) = 754$ and $G(10^2) = 583766$.
Find $G(10^{18})$. Give your answer modulo $1\,000\,000\,007$.