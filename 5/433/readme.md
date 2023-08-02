Let $E(x_0, y_0)$ be the number of steps it takes to determine the greatest common divisor of $x_0$ and $y_0$ with Euclid's algorithm. More formally:$x_1 = y_0$, $y_1 = x_0 \bmod y_0$$x_n = y_{n-1}$, $y_n = x_{n-1} \bmod y_{n-1}$
$E(x_0, y_0)$ is the smallest $n$ such that $y_n = 0$.


We have $E(1,1) = 1$, $E(10,6) = 3$ and $E(6,10) = 4$.


Define $S(N)$ as the sum of $E(x,y)$ for $1 \leq x,y \leq N$.
We have $S(1) = 1$, $S(10) = 221$ and $S(100) = 39826$.


Find $S(5\cdot 10^6)$.