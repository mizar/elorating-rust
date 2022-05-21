# 特殊関数 開発メモ

# 二重階乗

- 二重階乗 (double fractorial) / 半階乗 (semifactorial)
- [ja.wikipedia: 二重階乗](https://ja.wikipedia.org/wiki/%E4%BA%8C%E9%87%8D%E9%9A%8E%E4%B9%97)
- [en.wikipedia: Double factorial](https://en.wikipedia.org/wiki/Double_factorial)

$$
\begin{aligned}
n!!&=\prod_{k=0}^{\left\lceil\frac{n}{2}\right\rceil-1}(n-2k)=n(n-2)(n-4)\cdots\\
n!!&=\prod_{k=1}^\frac{n}{2}(2k)=n(n-2)(n-4)\cdots 4\cdot 2\qquad(n\text{ is even})\\
\\
0!!&=1\\
2!!&=2\\
4!!&=8\\
6!!&=48\\
8!!&=384\\
10!!&=3840\\
12!!&=46080\\
14!!&=645120\\
\\
n!!&=\prod_{k=1}^\frac{n+1}{2}(2k-1)=n(n-2)(n-4)\cdots 3\cdot 1\qquad(n\text{ is odd})\\
\\
1!!&=1\\
3!!&=3\\
5!!&=15\\
7!!&=105\\
9!!&=945\\
11!!&=10395\\
13!!&=135135\\
15!!&=2027025\\
\\
(2k)!!&=2^kk!\qquad(k\geq 0)\\
(2k-1)!!&=\frac{(2k)!}{2^kk!}=\frac{(2k)!}{(2k)!!}=\frac{(2k-1)!}{(2k-2)!!}\qquad(k\geq 1)\\
\end{aligned}
$$

# ガンマ関数

$$
\begin{aligned}
\Gamma(z)&=\int_0^\infty t^{z-1} \thinspace e^{-t} \thinspace \mathrm{d}t\qquad(\Re{z} > 0)\\
\Gamma(z+1)&=z \thinspace \Gamma(z)\\
\Gamma(n+1)&=n!\\
\Gamma\left(\frac{n}{2}\right)&=\frac{(n-2)!!}{2^\frac{n-1}{2}}\sqrt\pi\\
\Gamma\left(\frac{1}{2}+n\right)&=\frac{(2n-1)!!}{2^n}\sqrt\pi \thinspace = \thinspace \frac{(2n)!}{4^nn!}\sqrt\pi\qquad(n\ge 1)\\
\Gamma\left(\frac{1}{2}-n\right)&=\frac{(-2)^n}{(2n-1)!!}\sqrt\pi \thinspace = \thinspace \frac{(-4)^nn!}{(2n)!}\sqrt\pi\qquad(n\ge 1)\\
\Gamma(z) \thinspace \Gamma(1-z)&=-z \thinspace \Gamma(z) \thinspace \Gamma(-z)=\frac{\pi}{\sin\pi{z}}\\
\end{aligned}
$$

$$
\begin{aligned}
\Gamma\left(\frac12\right)&=\sqrt{\pi} \thinspace \approx \thinspace 1.772 \thinspace 453 \thinspace 850 \thinspace 905 \thinspace 516 \thinspace 0273\\
\Gamma(1)&=1\\
\Gamma\left(\frac32\right)&=\frac12\sqrt{\pi} \thinspace \approx \thinspace 0.886 \thinspace 226 \thinspace 925 \thinspace 452 \thinspace 758 \thinspace 0137\\
\Gamma(2)&=1\\
\Gamma\left(\frac52\right)&=\frac34\sqrt{\pi} \thinspace \approx \thinspace 1.329 \thinspace 340 \thinspace 388 \thinspace 179 \thinspace 137 \thinspace 0205\\
\Gamma(3)&=2\\
\Gamma\left(\frac72\right)&=\frac{15}8\sqrt{\pi} \thinspace \approx \thinspace 3.323 \thinspace 350 \thinspace 970 \thinspace 447 \thinspace 842 \thinspace 5512\\
\Gamma(4)&=6\\
\Gamma\left(\frac92\right)&=\frac{105}{16}\sqrt{\pi} \thinspace \approx \thinspace 11.631 \thinspace 728 \thinspace 396 \thinspace 567 \thinspace 448 \thinspace 9291\\
\Gamma(5)&=24\\
\end{aligned}
$$

- https://en.wikipedia.org/wiki/Particular_values_of_the_gamma_function
- https://www.boost.org/doc/libs/1_78_0/boost/math/special_functions/gamma.hpp
- https://www.boost.org/doc/libs/1_78_0/libs/math/doc/html/math_toolkit/sf_gamma/tgamma.html
- https://www.boost.org/doc/libs/1_78_0/libs/math/doc/html/math_toolkit/sf_gamma/lgamma.html

## Lanczos近似

- Lanczos近似 : [en.wikipedia: Lanczos approximation](https://en.wikipedia.org/wiki/Lanczos_approximation)
- パラメータ算出 : [https://mrob.com/pub/ries/lanczos-gamma.html](https://mrob.com/pub/ries/lanczos-gamma.html)
- https://www.boost.org/doc/libs/1_78_0/boost/math/special_functions/lanczos.hpp
- https://www.boost.org/doc/libs/1_78_0/libs/math/doc/html/math_toolkit/lanczos.html

## 大浦による実ガンマ関数近似

- [大浦拓哉, ガンマ関数および誤差関数の初等関数近似とその最適化](http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf-j.html) [(ja)](http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf-j.html) [(en)](http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf.html)

$$
\begin{aligned}
\Gamma(x)&\approx\exp((x-0.5)\ln(x+V)-x)\\
&\quad\cdot((\cdots(A_n/(x+B_n)+\cdots A_1)/(x+B_1)+A_0)/x+A_r)\qquad(0 < x <\infty)\\
\end{aligned}
$$

# 誤差関数

- [ja.wikipedia: 誤差関数](https://ja.wikipedia.org/wiki/%E8%AA%A4%E5%B7%AE%E9%96%A2%E6%95%B0)
- [en.wikipedia: Error function](https://en.wikipedia.org/wiki/Error_function)
- 誤差関数 (error function)
    - https://www.boost.org/doc/libs/1_78_0/boost/math/special_functions/erf.hpp
    - https://www.boost.org/doc/libs/1_78_0/libs/math/doc/html/math_toolkit/sf_erf/error_function.html
    - https://www.boost.org/doc/libs/1_78_0/libs/math/doc/html/math_toolkit/sf_erf/error_inv.html

$$
\operatorname{erf}(x)=\frac{2}{\sqrt{\pi}}\int_0^x e^{-t^2}\mathrm{d}t
$$

- 相補誤差関数 (complementary error function)

$$
\operatorname{erfc}(x)=1-\operatorname{erf}(x) =\frac{2}{\sqrt{\pi}}\int_x^\infty e^{-t^2}\mathrm{d}t=e^{-x^2}\operatorname{erfcx}(x)
$$

- スケーリング相補誤差関数 (scaled complementary error function)

$$
\operatorname{erfcx}(x)=e^{x^2}\operatorname{erfc}(x) =\frac{2}{\sqrt{\pi}} e^{x^2}\int_x^\infty e^{-t^2}\mathrm{d}t
$$

## 大浦による実誤差関数近似

- [大浦拓哉, ガンマ関数および誤差関数の初等関数近似とその最適化](http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf-j.html) [(ja)](http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf-j.html) [(en)](http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf.html)

$$
\operatorname{erf}(x)\approx\sum_{k=0}^{N-1}A_k\cdot x^{2k+1}\qquad(|x|\le 0.125)
$$


## 大浦による実相補誤差関数近似

- [大浦拓哉, ガンマ関数および誤差関数の初等関数近似とその最適化](http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf-j.html) [(ja)](http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf-j.html) [(en)](http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf.html)

$$
\begin{aligned}
\operatorname{erfc}(x)&\approx E(x)+x\exp(-x^2)\sum_{k=0}^{N-1}\frac{A_k}{x^2+B_k}\qquad(-\infty < x <\infty)\\
E(x)&=\begin{cases}2/(1+\exp(\alpha x))&(x <\beta)\\
0&(x\ge\beta)\end{cases}\\
\end{aligned}
$$


# ベータ関数

- [ja.wikipedia: ベータ関数](https://ja.wikipedia.org/wiki/%E3%83%99%E3%83%BC%E3%82%BF%E9%96%A2%E6%95%B0)
- [en.wikipedia: Beta function](https://en.wikipedia.org/wiki/Beta_function)
- https://www.boost.org/doc/libs/1_78_0/boost/math/special_functions/beta.hpp
- https://www.boost.org/doc/libs/1_78_0/libs/math/doc/html/math_toolkit/sf_beta/beta_function.html
- https://www.boost.org/doc/libs/1_78_0/libs/math/doc/html/math_toolkit/sf_beta/ibeta_function.html
- https://www.boost.org/doc/libs/1_78_0/libs/math/doc/html/math_toolkit/sf_beta/ibeta_inv_function.html

$$
\begin{aligned}
\operatorname{B}(x,y)&=\int_0^1 t^{x-1} \thinspace (1-t)^{y-1} \thinspace \mathrm{d}t\\
\operatorname{B}(x,y)&=\frac{\Gamma(x)\Gamma(y)}{\Gamma(x+y)}\\
\operatorname{B}(x,y)&=\operatorname{B}(y,x)\\
x \thinspace \operatorname{B}(x,y+1)&=y \thinspace \operatorname{B}(x+1,y)\\
\operatorname{B}(x,y)&=\operatorname{B}(x+1,y)+\operatorname{B}(x,y+1)\\
\operatorname{B}(x+1,y)&=\operatorname{B}(x,y)\cdot\frac{x}{x+y}\\
\operatorname{B}(x,y+1)&=\operatorname{B}(x,y)\cdot\frac{y}{x+y}\\
\operatorname{B}(x,y)\cdot\operatorname{B}(x+y,1-y)&=\frac{\pi}{x\sin(\pi y)}\\
\operatorname{B}(x,1-x)&=\frac{\pi}{\sin(\pi x)}\\
\operatorname{B}(1,x)&=\frac{1}{x}\\
\operatorname{B}(1/2,1/2)&=\pi\\
\end{aligned}
$$

# 不完全ベータ関数

- 不完全ベータ関数 (incomplete beta function)
- [ja.wikipedia: 不完全ベータ関数](https://ja.wikipedia.org/wiki/%E4%B8%8D%E5%AE%8C%E5%85%A8%E3%83%99%E3%83%BC%E3%82%BF%E9%96%A2%E6%95%B0)
- [en.wikipedia: Beta function #Incomplete beta function](https://en.wikipedia.org/wiki/Beta_function#Incomplete_beta_function)

$$
\begin{aligned}
\operatorname{B}_x(a,b)&=\int_{0}^{x} t^{a-1} \thinspace (1-t)^{b-1} \thinspace \mathrm{d}t\qquad(0\le\Re{z}\le 1)\\
\operatorname{B}_x(a,b)&=\operatorname{B}_x(a,b)+\operatorname{B}_{1-x}(b,a)\\
\end{aligned}
$$

## 正則ベータ関数

- 正則ベータ関数 (regularized beta function) / 正則化不完全ベータ関数 (regularized incomplete beta function)
- [Incomplete Beta Functions-Implementation](https://www.boost.org/doc/libs/1_68_0/libs/math/doc/html/math_toolkit/sf_beta/ibeta_function.html#math_toolkit.sf_beta.ibeta_function.implementation)
- [Significant Digit Computation of the Incomplete Beta Function Ratios. AR DiDonato, AH Morris Jr-1988-dtic.mil](http://www.dtic.mil/dtic/tr/fulltext/u2/a210118.pdf)
- [Armido R. Didonato and Alfred H. Morris, Jr.. 1992. Algorithm 708: Significant digit computation of the incomplete beta function ratios. ACM Trans. Math. Softw. 18, 3(September 1992), 360-373.](https://doi.org/10.1145/131766.131776)
- wolframalpha:
  - [`wolfram language symbol BetaRegularized`](https://www.wolframalpha.com/input/?i=wolfram+language+symbol+BetaRegularized&lang=ja)
  - [`BetaRegularized[x,a,b]`](https://www.wolframalpha.com/input/?i=BetaRegularized%5Bx%2Ca%2Cb%5D&lang=ja)

$$
\begin{aligned}
{\operatorname{I}}_{x}(a,b)&=\frac{\operatorname{B}_{x}(a,b)}{\operatorname{B}(a,b)}\qquad(0\le\Re{x}\le1)\\
\frac{\partial}{\partial x}\operatorname{I}_{x}(a,b)&=\frac{x^{a-1}(1-x)^{b-1}}{\operatorname{B}(a,b)}\\
{\operatorname{I}}_{x}(a,b)&=1-{\operatorname{I}}_{1-x}(b,a)\\
\end{aligned}
$$

## ニュートン法適用時の変形

不完全ベータ関数にニュートン法を適用する際、a,b,x の値が極端な場合の計算安定度を上げるため、入出力に以下のような関数を適用して順関数・逆関数の計算を行う。

$$
\begin{aligned}
\tanh^{-1}\left(-1+2\cdot {\operatorname{I}}_{\frac{1}{1+e^{-x}}}(a,b)\right)&=\frac{1}{2}\ln\frac{{\operatorname{I}}_{\frac{1}{1+e^{-x}}}(a,b)}{\operatorname{I}_{\frac{1}{1+e^x}}(b,a)}\\
\frac{\partial}{\partial x}\tanh^{-1}\left(-1+2\cdot {\operatorname{I}}_{\frac{1}{1+e^{-x}}}(a,b)\right)&=\frac{\left(\frac{1}{1+e^{-x}}\right)^a\left(\frac{1}{1+e^x}\right)^b}{2\operatorname{B}(a,b){\operatorname{I}}_{\frac{1}{1+e^{-x}}}(a,b)\left(1-{\operatorname{I}}_{\frac{1}{1+e^{-x}}}(a,b)\right)}\\
\end{aligned}
$$

- 参考 wolframalpha:
  - [`ATanh[-1+2BetaRegularized[1/(1+e^(-x)),a,b]]`](https://www.wolframalpha.com/input/?i=ATanh%5B-1%2B2BetaRegularized%5B1%2F%281%2Be%5E%28-x%29%29%2Ca%2Cb%5D%5D&lang=ja)
  - 左辺式のプロット結果、 x の絶対値が大きいと計算・プロットが難しい。
    - [`plot ATanh[-1+2BetaRegularized[1/(1+e^(-x)),120,80]],-0.8<x<1.7,-20<y<20`](https://www.wolframalpha.com/input/?i=plot+ATanh%5B-1%2B2BetaRegularized%5B1%2F%281%2Be%5E%28-x%29%29%2C120%2C80%5D%5D%2C-0.8%3Cx%3C1.7%2C-20%3Cy%3C20&lang=ja)
  - 右辺式のプロット結果、 x の絶対値が大きくても計算・プロットできる。
    - [`plot Log[BetaRegularized[1/(1+e^(-x)),120,80]/BetaRegularized[1/(1+e^x),80,120]]/2,-0.8<x<1.7,-20<y<20`](https://www.wolframalpha.com/input/?i=plot+Log%5BBetaRegularized%5B1%2F%281%2Be%5E%28-x%29%29%2C120%2C80%5D%2FBetaRegularized%5B1%2F%281%2Be%5Ex%29%2C80%2C120%5D%5D%2F2%2C-0.8%3Cx%3C1.7%2C-20%3Cy%3C20&lang=ja)
    - [`plot Log[BetaRegularized[1/(1+e^(-x)),120,80]/BetaRegularized[1/(1+e^x),80,120]]/2,-3<x<3,-120<y<60`](https://www.wolframalpha.com/input/?i=plot+Log%5BBetaRegularized%5B1%2F%281%2Be%5E%28-x%29%29%2C120%2C80%5D%2FBetaRegularized%5B1%2F%281%2Be%5Ex%29%2C80%2C120%5D%5D%2F2%2C-3%3Cx%3C3%2C-120%3Cy%3C60&lang=ja)


## 不完全ベータ関数の連分数展開

$$
\begin{aligned}
{\operatorname{I}}_{x}(a,b)&=\frac{x^a(1-x)^b}{a\operatorname{B}(a,b)} \thinspace \cfrac{1}{1+\cfrac{{d}_{1}}{1+\cfrac{{d}_{2}}{1+\cfrac{{d}_{3}}{1+\cfrac{{d}_{4}}{1+ \thinspace \cdots}}}}}\qquad(0 \lt x \lt 1)\\
{d}_{1}&=-\frac{a+b}{a+1}x\\
{d}_{2n}&=\frac{n(b-n)}{(a+2n-1)(a+2n)}x\\
{d}_{2n+1}&=-\frac{(a+n)(a+b+n)}{(a+2n)(a+2n+1)}x\\
\end{aligned}
$$

## 連分数展開の計算

連分数の計算を行列の積として計算し、オーバーフロー・アンダーフローを防ぐため定期的に分子・分母に適当な値を乗算する。

$$
\begin{aligned}
&\quad\begin{pmatrix}P_n&Q_n\\
R_n&S_n\end{pmatrix}\\
&=\begin{pmatrix}0&a_1\\
1&b_1\end{pmatrix}\begin{pmatrix}0&a_2\\
1&b_2\end{pmatrix}\begin{pmatrix}0&a_3\\
1&b_3\end{pmatrix}\begin{pmatrix}
0&a_4\\
1&b_4\end{pmatrix}\cdots\begin{pmatrix}0&a_{n-1}\\
1&b_{n-1}\end{pmatrix}\begin{pmatrix}
0&a_n\\
1&b_n\end{pmatrix}\\
&=\begin{pmatrix}a_1&a_1b_2\\
b_1&a_2+b_1b_2\end{pmatrix}\begin{pmatrix}a_3&a_3b_4\\
b_3&a_4+b_3b_4\end{pmatrix}\cdots\begin{pmatrix}a_{2m-1}&a_{2m-1}b_{2m}\\
b_{2m-1}&a_{2m}+b_{2m-1}b_{2m}\end{pmatrix}\cdots\begin{pmatrix}0&a_{n-1}\\
1&b_{n-1}\end{pmatrix}\begin{pmatrix}0&a_n\\
1&b_n\end{pmatrix}\\
\end{aligned}
$$

$$
\begin{aligned}
F_n(x)&=b_0+\frac{P_n \thinspace x+Q_n}{R_n \thinspace x+S_n}\\
&=b_0+\cfrac{a_1}{b_1+\cfrac{a_2}{b_2+\cfrac{a_3}{b_3+ \thinspace \cdots \thinspace \cfrac{a_n}{b_n+x}}}}\\
&\cong b_0+\frac{Q_n}{S_n}\qquad\left(\because\lim_{n\to\infty}\frac{P_n}{R_n}=\lim_{n\to\infty}\frac{Q_n}{S_n}\right)\\
\end{aligned}
$$
