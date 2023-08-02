Let $\{a_1, a_2, \dots, a_n\}$ be an integer sequence of length $n$ such that:
$a_1 = 6$
for all $1 \le i \lt n$: $\phi(a_i) \lt \phi(a_{i + 1}) \lt a_i \lt a_{i + 1}$.1
Let $S(N)$ be the number of such sequences with $a_n \le N$.
For example, $S(10) = 4$: $\{6\}$, $\{6, 8\}$, $\{6, 8, 9\}$ and $\{6, 10\}$.
We can verify that $S(100) = 482073668$ and $S(10\,000) \bmod 10^8 = 73808307$.
Find $S(20\,000\,000) \bmod 10^8$.
1 $\phi$ denotes Euler's totient function.