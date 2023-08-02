The flipping game is a two player game played on an $N$ by $N$ square board.
Each square contains a disk with one side white and one side black.
The game starts with all disks showing their white side.
A turn consists of flipping all disks in a rectangle with the following properties:
the upper right corner of the rectangle contains a white disk
the rectangle width is a perfect square ($1$, $4$, $9$, $16$, ...)
the rectangle height is a triangular numberThe triangular numbers are defined as $\frac 1 2 n(n + 1)$ for positive integer $n$. ($1$, $3$, $6$, $10$, ...)


Players alternate turns. A player wins by turning the grid all black.
Let $W(N)$ be the number of winning movesThe first move of a strategy that ensures a win no matter what the opponent plays. for the first player on an $N$ by $N$ board with all disks white, assuming perfect play.
$W(1) = 1$, $W(2) = 0$, $W(5) = 8$ and $W(10^2) = 31395$.
For $N=5$, the first player's eight winning first moves are:

Find $W(10^6)$.