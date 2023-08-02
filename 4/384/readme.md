Define the sequence $a(n)$ as the number of adjacent pairs of ones in the binary expansion of $n$ (possibly overlapping).
E.g.: $a(5) = a(101_2) = 0$, $a(6) = a(110_2) = 1$, $a(7) = a(111_2) = 2$.
Define the sequence $b(n) = (-1)^{a(n)}$.
This sequence is called the Rudin-Shapiro sequence.
Also consider the summatory sequence of $b(n)$: $s(n) = \sum \limits_{i = 0}^n b(i)$.
The first couple of values of these sequences are:

$n$
$0$
$1$
$2$
$3$
$4$
$5$
$6$
$7$

$a(n)$
$0$
$0$
$0$
$1$
$0$
$0$
$1$
$2$

$b(n)$
$1$
$1$
$1$
$-1$
$1$
$1$
$-1$
$1$

$s(n)$
$1$
$2$
$3$
$2$
$3$
$4$
$3$
$4$

The sequence $s(n)$ has the remarkable property that all elements are positive and every positive integer $k$ occurs exactly $k$ times.
Define $g(t,c)$, with $1 \le c \le t$, as the index in $s(n)$ for which $t$ occurs for the $c$'th time in $s(n)$.
E.g.: $g(3,3) = 6$, $g(4,2) = 7$ and $g(54321,12345) = 1220847710$.
Let $F(n)$ be the Fibonacci sequence defined by:
$F(0)=F(1)=1$ and
$F(n)=F(n-1)+F(n-2)$ for $n \gt 1$.
Define $GF(t)=g(F(t),F(t-1))$.
Find $\sum GF(t)$ for $2 \le t \le 45$.