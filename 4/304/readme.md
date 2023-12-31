For any positive integer $n$ the function $\operatorname{next\_prime}(n)$ returns the smallest prime $p$ such that $p \gt n$.


The sequence $a(n)$ is defined by:
$a(1)=\operatorname{next\_prime}(10^{14})$ and $a(n)=\operatorname{next\_prime}(a(n-1))$ for $n \gt 1$.


The Fibonacci sequence $f(n)$ is defined by:
$f(0)=0$, $f(1)=1$ and $f(n)=f(n-1)+f(n-2)$ for $n \gt 1$.


The sequence $b(n)$ is defined as $f(a(n))$.


Find $\sum b(n)$ for $1 \le n \le 100\,000$. 
Give your answer mod $1234567891011$.