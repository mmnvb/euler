Define $f_k(n) = \sum_{i=0}^n f_k(\lfloor\frac i k \rfloor)$ where $f_k(0) = 1$ and $\lfloor x \rfloor$ denotes the floor function.
For example, $f_5(10) = 18$, $f_7(100) = 1003$, and $f_2(10^3) = 264830889564$.
Find $(\sum_{k=2}^{10} f_k(10^{14})) \bmod (10^9+7)$.