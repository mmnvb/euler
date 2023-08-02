Here are the records from a busy telephone system with one million users:

RecNrCallerCalled
$1$$200007$$100053$$2$$600183$$500439$$3$$600863$$701497$$\cdots$$\cdots$$\cdots$
The telephone number of the caller and the called number in record $n$ are $\operatorname{Caller}(n) = S_{2n-1}$ and $\operatorname{Called}(n) = S_{2n}$ where $S_{1,2,3,\dots}$ come from the "Lagged Fibonacci Generator":
For $1 \le k \le 55$, $S_k = [100003 - 200003k + 300007k^3] \pmod{1000000}$.
For $56 \le k$, $S_k = [S_{k-24} + S_{k-55}] \pmod{1000000}$.
If $\operatorname{Caller}(n) = \operatorname{Called}(n)$ then the user is assumed to have misdialled and the call fails; otherwise the call is successful.
From the start of the records, we say that any pair of users $X$ and $Y$ are friends if $X$ calls $Y$ or vice-versa. Similarly, $X$ is a friend of a friend of $Z$ if $X$ is a friend of $Y$ and $Y$ is a friend of $Z$; and so on for longer chains.
The Prime Minister's phone number is $524287$. After how many successful calls, not counting misdials, will $99\%$ of the users (including the PM) be a friend, or a friend of a friend etc., of the Prime Minister?