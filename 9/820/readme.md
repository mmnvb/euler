Let $d_n(x)$ be the $n$th decimal digit of the fractional part of $x$, or $0$ if the fractional part has fewer than $n$ digits.
For example:

$d_7 \mathopen{}\left( 1 \right)\mathclose{} = d_7 \mathopen{}\left( \frac 1 2 \right)\mathclose{} = d_7 \mathopen{}\left( \frac 1 4 \right)\mathclose{} = d_7 \mathopen{}\left( \frac 1 5 \right)\mathclose{} = 0$
$d_7 \mathopen{}\left( \frac 1 3 \right)\mathclose{} = 3$ since $\frac 1 3 =$ 0.3333333333...
$d_7 \mathopen{}\left( \frac 1 6 \right)\mathclose{} = 6$ since $\frac 1 6 =$ 0.1666666666...
$d_7 \mathopen{}\left( \frac 1 7 \right)\mathclose{} = 1$ since $\frac 1 7 =$ 0.1428571428...

Let $\displaystyle  S(n) = \sum_{k=1}^n d_n \mathopen{}\left( \frac 1 k \right)\mathclose{}$.
You are given:

$S(7) = 0 + 0 + 3 + 0 + 0 + 6 + 1 = 10$
$S(100) = 418$

Find $S(10^7)$.