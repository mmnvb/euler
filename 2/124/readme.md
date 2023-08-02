The radical of $n$, $\operatorname{rad}(n)$, is the product of the distinct prime factors of $n$. For example, $504 = 2^3 \times 3^2 \times 7$, so $\operatorname{rad}(504) = 2 \times 3 \times 7 = 42$.
If we calculate $\operatorname{rad}(n)$ for $1 \le n \le 10$, then sort them on $\operatorname{rad}(n)$, and sorting on $n$ if the radical values are equal, we get:


Unsorted
 
Sorted


n
rad(n)
 
n
rad(n)
k


11
 
111


22
 
222


33
 
423


42
 
824


55
 
335


66
 
936


77
 
557


82
 
668


93
 
779


1010
 
101010


Let $E(k)$ be the $k$th element in the sorted $n$ column; for example, $E(4) = 8$ and $E(6) = 9$.
If $\operatorname{rad}(n)$ is sorted for $1 \le n \le 100000$, find $E(10000)$.