Let $F_5(n)$ be the number of strings $s$ such that:
$s$ consists only of '0's and '1's,
$s$ has length at most $n$, and
$s$ contains a palindromic substring of length at least $5$.
For example, $F_5(4) = 0$, $F_5(5) = 8$, 
$F_5(6) = 42$ and $F_5(11) = 3844$.
Let $D(L)$ be the number of integers $n$ such that $5 \le n \le L$ and $F_5(n)$ is divisible by $87654321$.
For example, $D(10^7) = 0$ and $D(5 \cdot 10^9) = 51$.
Find $D(10^{18})$.