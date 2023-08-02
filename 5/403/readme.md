For integers $a$ and $b$, we define $D(a, b)$ as the domain enclosed by the parabola $y = x^2$ and the line $y = a\cdot x + b$:$D(a, b) = \{(x, y) \mid x^2 \leq y \leq a\cdot x + b \}$.


$L(a, b)$ is defined as the number of lattice points contained in $D(a, b)$.
For example, $L(1, 2) = 8$ and $L(2, -1) = 1$.


We also define $S(N)$ as the sum of $L(a, b)$ for all the pairs $(a, b)$ such that the area of $D(a, b)$ is a rational number and $|a|,|b| \leq N$.
We can verify that $S(5) = 344$ and $S(100) = 26709528$.


Find $S(10^{12})$. Give your answer mod $10^8$.