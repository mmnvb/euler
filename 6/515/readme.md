Let $d(p, n, 0)$ be the multiplicative inverse of $n$ modulo prime $p$, defined as $n \times d(p, n, 0) = 1 \bmod p$.
Let $d(p, n, k) = \sum_{i = 1}^n d(p, i, k - 1)$ for $k \ge 1$.
Let $D(a, b, k) = \sum (d(p, p-1, k) \bmod p)$ for all primes $a \le p \lt a + b$.
You are given:
$D(101,1,10) = 45$
$D(10^3,10^2,10^2) = 8334$
$D(10^6,10^3,10^3) = 38162302$Find $D(10^9,10^5,10^5)$.