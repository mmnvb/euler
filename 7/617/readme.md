For two integers $n,e \gt 1$, we define an $(n,e)$-MPS (Mirror Power Sequence) to be an infinite sequence of integers $(a_i)_{i\ge 0}$ such that for all $i\ge 0$, $a_{i+1} = \min(a_i^e,n-a_i^e)$ and $a_i \gt 1$. Examples of such sequences are the two $(18,2)$-MPS sequences made of alternating $2$ and $4$.
Note that even though such a sequence is uniquely determined by $n,e$ and $a_0$, for most values such a sequence does not exist. For example, no $(n,e)$-MPS exists for $n \lt 6$.
Define $C(n)$ to be the number of $(n,e)$-MPS for some $e$, and $\displaystyle D(N) = \sum_{n=2}^N C(n)$.
You are given that $D(10) = 2$, $D(100) = 21$, $D(1000) = 69$, $D(10^6) = 1303$ and $D(10^{12}) = 1014800$.
Find $D(10^{18})$.