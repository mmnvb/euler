A game is played with three piles of stones and two players.
On each player's turn, the player may remove one or more stones from the piles. However, if the player takes stones from more than one pile, then the same number of stones must be removed from each of the selected piles.
In other words, the player chooses some $N \gt 0$ and removes:
$N$ stones from any single pile; or
$N$ stones from each of any two piles ($2N$ total); or
$N$ stones from each of the three piles ($3N$ total).
The player taking the last stone(s) wins the game.
A winning configuration is one where the first player can force a win.
For example, $(0,0,13)$, $(0,11,11)$, and $(5,5,5)$ are winning configurations because the first player can immediately remove all stones.
A losing configuration is one where the second player can force a win, no matter what the first player does.
For example, $(0,1,2)$ and $(1,3,3)$ are losing configurations: any legal move leaves a winning configuration for the second player.
Consider all losing configurations $(x_i, y_i, z_i)$ where $x_i \le y_i \le z_i \le 100$.
We can verify that $\sum (x_i + y_i + z_i) = 173895$ for these.
Find $\sum (x_i + y_i + z_i)$ where $(x_i, y_i, z_i)$ ranges over the losing configurations with $x_i \le y_i \le z_i \le 1000$.