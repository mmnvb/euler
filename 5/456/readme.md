Define:$x_n = (1248^n \bmod 32323) - 16161$$y_n = (8421^n \bmod 30103) - 15051$
$P_n = \{(x_1, y_1), (x_2, y_2), \dots, (x_n, y_n)\}$

For example, $P_8 = \{(-14913, -6630),$$(-10161, 5625),$$(5226, 11896),$$(8340, -10778),$$(15852, -5203),$$(-15165, 11295),$$(-1427, -14495),$$(12407, 1060)\}$.
Let $C(n)$ be the number of triangles whose vertices are in $P_n$ which contain the origin in the interior.

Examples:
$C(8) = 20$
$C(600) = 8950634$
$C(40\,000) = 2666610948988$

Find $C(2\,000\,000)$.