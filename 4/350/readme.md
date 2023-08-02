A list of size $n$ is a sequence of $n$ natural numbers. Examples are $(2,4,6)$, $(2,6,4)$, $(10,6,15,6)$, and $(11)$.

The greatest common divisor, or $\gcd$, of a list is the largest natural number that divides all entries of the list. Examples: $\gcd(2,6,4) = 2$, $\gcd(10,6,15,6) = 1$ and $\gcd(11) = 11$.

The least common multiple, or $\operatorname{lcm}$, of a list is the smallest natural number divisible by each entry of the list. Examples: $\operatorname{lcm}(2,6,4) = 12$, $\operatorname{lcm}(10,6,15,6) = 30$ and $\operatorname{lcm}(11) = 11$.

Let $f(G, L, N)$ be the number of lists of size $N$ with $\gcd \ge G$ and $\operatorname{lcm} \le L$. For example:

$f(10, 100, 1) = 91$.
$f(10, 100, 2) = 327$.
$f(10, 100, 3) = 1135$.
$f(10, 100, 1000) \bmod 101^4 = 3286053$.

Find $f(10^6, 10^{12}, 10^{18}) \bmod 101^4$.