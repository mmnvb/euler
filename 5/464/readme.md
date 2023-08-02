The Möbius function, denoted $\mu(n)$, is defined as:
$\mu(n) = (-1)^{\omega(n)}$ if $n$ is squarefree (where $\omega(n)$ is the number of distinct prime factors of $n$)
$\mu(n) = 0$ if $n$ is not squarefree.

Let $P(a, b)$ be the number of integers $n$ in the interval $[a, b]$ such that $\mu(n) = 1$.
Let $N(a, b)$ be the number of integers $n$ in the interval $[a, b]$ such that $\mu(n) = -1$.
For example, $P(2,10) = 2$ and $N(2,10) = 4$.


Let $C(n)$ be the number of integer pairs $(a, b)$ such that:
 $1\le a \le b \le n$,
 $99 \cdot N(a, b) \le 100 \cdot P(a, b)$, and
 $99 \cdot P(a, b) \le 100 \cdot N(a, b)$.

For example, $C(10) = 13$, $C(500) = 16676$ and $C(10\,000) = 20155319$.


Find $C(20\,000\,000)$.