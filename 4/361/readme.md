The Thue-Morse sequence $\{T_n\}$ is a binary sequence satisfying:
$T_0 = 0$
$T_{2n} = T_n$
$T_{2n + 1} = 1 - T_n$

The first several terms of $\{T_n\}$ are given as follows:
$01101001{\color{red}10010}1101001011001101001\cdots$


We define $\{A_n\}$ as the sorted sequence of integers such that the binary expression of each element appears as a subsequence in $\{T_n\}$.
For example, the decimal number $18$ is expressed as $10010$ in binary. $10010$ appears in $\{T_n\}$ ($T_8$ to $T_{12}$), so $18$ is an element of $\{A_n\}$.
The decimal number $14$ is expressed as $1110$ in binary. $1110$ never appears in $\{T_n\}$, so $14$ is not an element of $\{A_n\}$.


The first several terms of $\{A_n\}$ are given as follows:

$n$
$0$
$1$
$2$
$3$
$4$
$5$
$6$
$7$
$8$
$9$
$10$
$11$
$12$
$\cdots$

$A_n$
$0$
$1$
$2$
$3$
$4$
$5$
$6$
$9$
$10$
$11$
$12$
$13$
$18$
$\cdots$


We can also verify that $A_{100} = 3251$ and $A_{1000} = 80852364498$.


Find the last $9$ digits of $\sum \limits_{k = 1}^{18} A_{10^k}$.