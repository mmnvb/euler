For any set $A$ of numbers, let $\operatorname{sum}(A)$ be the sum of the elements of $A$.
Consider the set $B = \{1,3,6,8,10,11\}$. There are $20$ subsets of $B$ containing three elements, and their sums are:

\begin{align}
\operatorname{sum}(\{1,3,6\}) &= 10,\\
\operatorname{sum}(\{1,3,8\}) &= 12,\\
\operatorname{sum}(\{1,3,10\}) &= 14,\\
\operatorname{sum}(\{1,3,11\}) &= 15,\\
\operatorname{sum}(\{1,6,8\}) &= 15,\\
\operatorname{sum}(\{1,6,10\}) &= 17,\\
\operatorname{sum}(\{1,6,11\}) &= 18,\\
\operatorname{sum}(\{1,8,10\}) &= 19,\\
\operatorname{sum}(\{1,8,11\}) &= 20,\\
\operatorname{sum}(\{1,10,11\}) &= 22,\\
\operatorname{sum}(\{3,6,8\}) &= 17,\\
\operatorname{sum}(\{3,6,10\}) &= 19,\\
\operatorname{sum}(\{3,6,11\}) &= 20,\\
\operatorname{sum}(\{3,8,10\}) &= 21,\\
\operatorname{sum}(\{3,8,11\}) &= 22,\\
\operatorname{sum}(\{3,10,11\}) &= 24,\\
\operatorname{sum}(\{6,8,10\}) &= 24,\\
\operatorname{sum}(\{6,8,11\}) &= 25,\\
\operatorname{sum}(\{6,10,11\}) &= 27,\\
\operatorname{sum}(\{8,10,11\}) &= 29.
\end{align}

Some of these sums occur more than once, others are unique.
For a set $A$, let $U(A,k)$ be the set of unique sums of $k$-element subsets of $A$, in our example we find $U(B,3) = \{10,12,14,18,21,25,27,29\}$ and $\operatorname{sum}(U(B,3)) = 156$.
Now consider the $100$-element set $S = \{1^2, 2^2, \dots, 100^2\}$.
S has $100891344545564193334812497256$ $50$-element subsets.
Determine the sum of all integers which are the sum of exactly one of the $50$-element subsets of $S$, i.e. find $\operatorname{sum}(U(S,50))$.