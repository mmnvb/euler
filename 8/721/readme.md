Given is the function $f(a,n)=\lfloor (\lceil \sqrt a \rceil + \sqrt a)^n \rfloor$.
$\lfloor \cdot \rfloor$ denotes the floor function and $\lceil \cdot \rceil$ denotes the ceiling function.
$f(5,2)=27$ and $f(5,5)=3935$.


$G(n) = \displaystyle \sum_{a=1}^n f(a, a^2).$
$G(1000) \bmod 999\,999\,937=163861845. $
Find $G(5\,000\,000).$ Give your answer modulo $999\,999\,937$.