Let $g(a, n, b, m)$ be the smallest non-negative solution $x$ to the system:
$x = a \bmod n$
$x = b \bmod m$
if such a solution exists, otherwise $0$.


E.g. $g(2,4,4,6)=10$, but $g(3,4,4,6)=0$.


Let $\phi(n)$ be Euler's totient function.


Let $f(n,m)=g(\phi(n),n,\phi(m),m)$


Find $\sum f(n,m)$ for $1000000 \le n \lt m \lt 1005000$.