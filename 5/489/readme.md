Let $G(a, b)$ be the smallest non-negative integer $n$ for which $\operatorname{\mathbf{gcd}}$Greatest common divisor$(n^3 + b, (n + a)^3 + b)$ is maximized.
For example, $G(1, 1) = 5$ because $\gcd(n^3 + 1, (n + 1)^3 + 1)$ reaches its maximum value of $7$ for $n = 5$, and is smaller for $0 \le n \lt 5$.
Let $H(m, n) = \sum G(a, b)$ for $1 \le a \le m$, $1 \le b \le n$.
You are given $H(5, 5) = 128878$ and $H(10, 10) = 32936544$.
Find $H(18, 1900)$.