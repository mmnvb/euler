The Golomb's self-describing sequence $(G(n))$ is the only nondecreasing sequence of natural numbers such that $n$ appears exactly $G(n)$ times in the sequence. The values of $G(n)$ for the first few $n$ are

\[
\begin{matrix}
n & 1 & 2 & 3 & 4 & 5 & 6 & 7 & 8 & 9 & 10 & 11 & 12 & 13 & 14 & 15 & \ldots \\
G(n) & 1 & 2 & 2 & 3 & 3 & 4 & 4 & 4 & 5 & 5 & 5 & 6 & 6 & 6 & 6 & \ldots
\end{matrix}
\]

You are given that $G(10^3) = 86$, $G(10^6) = 6137$.
You are also given that $\sum G(n^3) = 153506976$ for $1 \le n \lt 10^3$.
Find $\sum G(n^3)$ for $1 \le n \lt 10^6$.