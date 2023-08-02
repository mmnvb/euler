For an $n$-tuple of integers $t = (a_1, \dots, a_n)$, let $(x_1, \dots, x_n)$ be the solutions of the polynomial equation $x^n + a_1 x^{n-1} + a_2 x^{n-2} + \cdots + a_{n-1}x + a_n = 0$.


Consider the following two conditions:
$x_1, \dots, x_n$ are all real.
If $x_1, \dots, x_n$ are sorted, $\lfloor x_i\rfloor = i$ for $1 \leq i \leq n$. ($\lfloor \cdot \rfloor$: floor function.)

In the case of $n = 4$, there are $12$ $n$-tuples of integers which satisfy both conditions.
We define $S(t)$ as the sum of the absolute values of the integers in $t$.
For $n = 4$ we can verify that $\sum S(t) = 2087$ for all $n$-tuples $t$ which satisfy both conditions.


Find $\sum S(t)$ for $n = 7$.