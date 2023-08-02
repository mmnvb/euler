We define a simber to be a positive integer in which any odd digit, if present, occurs an odd number of times, and any even digit, if present, occurs an even number of times.
For example, $141221242$ is a $9$-digit simber because it has three $1$'s, four $2$'s and two $4$'s. 
Let $Q(n)$ be the count of all simbers with at most $n$ digits.
You are given $Q(7) = 287975$ and $Q(100) \bmod 1\,000\,000\,123 = 123864868$.
Find $(\sum_{1 \le u \le 39} Q(2^u)) \bmod 1\,000\,000\,123$.