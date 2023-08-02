A $10$-substring of a number is a substring of its digits that sum to $10$. For example, the $10$-substrings of the number $3523014$ are:
3523014
3523014
3523014
3523014
A number is called $10$-substring-friendly if every one of its digits belongs to a $10$-substring. For example, $3523014$ is $10$-substring-friendly, but $28546$ is not.
Let $T(n)$ be the number of $10$-substring-friendly numbers from $1$ to $10^n$ (inclusive).
For example $T(2) = 9$ and $T(5) = 3492$.
Find $T(10^{18}) \bmod 1\,000\,000\,007$.