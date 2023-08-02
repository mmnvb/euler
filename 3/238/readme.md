Create a sequence of numbers using the "Blum Blum Shub" pseudo-random number generator:

\begin{align}
s_0 &= 14025256\\
s_{n + 1} &= s_n^2 \bmod 20300713
\end{align}

Concatenate these numbers $s_0s_1s_2\cdots$ to create a string $w$ of infinite length.
Then, $w = {\color{blue}14025256741014958470038053646\cdots}$
For a positive integer $k$, if no substring of $w$ exists with a sum of digits equal to $k$, $p(k)$ is defined to be zero. If at least one substring of $w$ exists with a sum of digits equal to $k$, we define $p(k) = z$, where $z$ is the starting position of the earliest such substring.
For instance:
The substrings $\color{blue}1, 14, 1402, \dots$
with respective sums of digits equal to $1, 5, 7, \dots$
start at position $\mathbf 1$, hence $p(1) = p(5) = p(7) = \cdots = \mathbf 1$.
The substrings $\color{blue}4, 402, 4025, \dots$
with respective sums of digits equal to $4, 6, 11, \dots$
start at position $\mathbf 2$, hence $p(4) = p(6) = p(11) = \cdots = \mathbf 2$.
The substrings $\color{blue}02, 0252, \dots$
with respective sums of digits equal to $2, 9, \dots$
start at position $\mathbf 3$, hence $p(2) = p(9) = \cdots = \mathbf 3$.
Note that substring $\color{blue}025$ starting at position $\mathbf 3$, has a sum of digits equal to $7$, but there was an earlier substring (starting at position $\mathbf 1$) with a sum of digits equal to $7$, so $p(7) = 1$, not $3$.
We can verify that, for $0 \lt k \le 10^3$, $\sum p(k) = 4742$.
Find $\sum p(k)$, for $0 \lt k \le 2 \times 10^{15}$.