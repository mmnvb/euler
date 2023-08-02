In a tournament there are $n$ teams and each team plays each other team twice. A team gets two points for a win, one point for a draw and no points for a loss.

With two teams there are three possible outcomes for the total points. $(4,0)$ where a team wins twice, $(3,1)$ where a team wins and draws, and $(2,2)$ where either there are two draws or a team wins one game and loses the other. Here we do not distinguish the teams and so $(3,1)$ and $(1,3)$ are considered identical.

Let $F(n)$ be the total number of possible final outcomes with $n$ teams, so that $F(2) = 3$.
You are also given $F(7) = 32923$.

Find $F(100)$. Give your answer modulo $10^9+7$.