Looking at the table below, it is easy to verify that the maximum possible sum of adjacent numbers in any direction (horizontal, vertical, diagonal or anti-diagonal) is $16$ ($= 8 + 7 + 1$).

$-2$$5$$3$$2$$9$$-6$$5$$1$$3$$2$$7$$3$$-1$$8$$-4$$8$
Now, let us repeat the search, but on a much larger scale:
First, generate four million pseudo-random numbers using a specific form of what is known as a "Lagged Fibonacci Generator":
For $1 \le k \le 55$, $s_k = [100003 - 200003 k + 300007 k^3] \pmod{1000000} - 500000$.
For $56 \le k \le 4000000$, $s_k = [s_{k-24} + s_{k - 55} + 1000000] \pmod{1000000} - 500000$.
Thus, $s_{10} = -393027$ and $s_{100} = 86613$.
The terms of $s$ are then arranged in a $2000 \times 2000$ table, using the first $2000$ numbers to fill the first row (sequentially), the next $2000$ numbers to fill the second row, and so on.
Finally, find the greatest sum of (any number of) adjacent entries in any direction (horizontal, vertical, diagonal or anti-diagonal).