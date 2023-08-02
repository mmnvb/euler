The following equation represents the continuous topography of a mountainous region, giving the elevationheight above sea level $h$ at any point $(x, y)$:
$$h = \left(5000 - \frac{x^2 + y^2 + xy}{200} + \frac{25(x + y)}2\right) \cdot e^{-\left|\frac{x^2 + y^2}{1000000} - \frac{3(x + y)}{2000} + \frac 7 {10}\right|}.$$

A mosquito intends to fly from $A(200,200)$ to $B(1400,1400)$, without leaving the area given by $0 \le x, y \le 1600$.
Because of the intervening mountains, it first rises straight up to a point $A^\prime$, having elevation $f$. Then, while remaining at the same elevation $f$, it flies around any obstacles until it arrives at a point $B^\prime$ directly above $B$.
First, determine $f_{\mathrm{min}}$ which is the minimum constant elevation allowing such a trip from $A$ to $B$, while remaining in the specified area.
Then, find the length of the shortest path between $A^\prime$ and $B^\prime$, while flying at that constant elevation $f_{\mathrm{min}}$.
Give that length as your answer, rounded to three decimal places.
Note: For convenience, the elevation function shown above is repeated below, in a form suitable for most programming languages:
h=( 5000-0.005*(x*x+y*y+x*y)+12.5*(x+y) ) * exp( -abs(0.000001*(x*x+y*y)-0.0015*(x+y)+0.7) )