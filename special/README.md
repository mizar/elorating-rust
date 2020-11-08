# 特殊関数 開発メモ

# 二重階乗

- 二重階乗 (double fractorial) / 半階乗 (semifactorial)
- [ja.wikipedia: 二重階乗](https://ja.wikipedia.org/wiki/%E4%BA%8C%E9%87%8D%E9%9A%8E%E4%B9%97)
- [en.wikipedia: Double factorial](https://en.wikipedia.org/wiki/Double_factorial)

![n!!=\prod_{k=0}^{\left\lceil\frac{n}{2}\right\rceil-1}(n-2k)=n(n-2)(n-4)\cdots](https://latex.codecogs.com/svg.latex?n%21%21%3D%5Cprod_%7Bk%3D0%7D%5E%7B%5Cleft%5Clceil%5Cfrac%7Bn%7D%7B2%7D%5Cright%5Crceil-1%7D%28n-2k%29%3Dn%28n-2%29%28n-4%29%5Ccdots)

![n!!=\prod_{k=1}^\frac{n}{2}(2k)=n(n-2)(n-4)\cdots 4\cdot 2\qquad(n\text{ is even})](https://latex.codecogs.com/svg.latex?n%21%21%3D%5Cprod_%7Bk%3D1%7D%5E%5Cfrac%7Bn%7D%7B2%7D%282k%29%3Dn%28n-2%29%28n-4%29%5Ccdots%204%5Ccdot%202%5Cqquad%28n%5Ctext%7B%20is%20even%7D%29)

![\begin{aligned}0!!&=1\\2!!&=2\\4!!&=8\\6!!&=48\\8!!&=384\\10!!&=3840\\12!!&=46080\\14!!&=645120\end{aligned}](https://latex.codecogs.com/svg.latex?%5Cbegin%7Baligned%7D0%21%21%26%3D1%5C%5C2%21%21%26%3D2%5C%5C4%21%21%26%3D8%5C%5C6%21%21%26%3D48%5C%5C8%21%21%26%3D384%5C%5C10%21%21%26%3D3840%5C%5C12%21%21%26%3D46080%5C%5C14%21%21%26%3D645120%5Cend%7Baligned%7D)

![n!!=\prod_{k=1}^\frac{n+1}{2}(2k-1)=n(n-2)(n-4)\cdots 3\cdot 1\qquad(n\text{ is odd})](https://latex.codecogs.com/svg.latex?n%21%21%3D%5Cprod_%7Bk%3D1%7D%5E%5Cfrac%7Bn+1%7D%7B2%7D%282k-1%29%3Dn%28n-2%29%28n-4%29%5Ccdots%203%5Ccdot%201%5Cqquad%28n%5Ctext%7B%20is%20odd%7D%29)

![\begin{aligned}1!!&=1\\3!!&=3\\5!!&=15\\7!!&=105\\9!!&=945\\11!!&=10395\\13!!&=135135\\15!!&=2027025\end{aligned}](https://latex.codecogs.com/svg.latex?%5Cbegin%7Baligned%7D1%21%21%26%3D1%5C%5C3%21%21%26%3D3%5C%5C5%21%21%26%3D15%5C%5C7%21%21%26%3D105%5C%5C9%21%21%26%3D945%5C%5C11%21%21%26%3D10395%5C%5C13%21%21%26%3D135135%5C%5C15%21%21%26%3D2027025%5Cend%7Baligned%7D)

# ガンマ関数

![\begin{aligned}\Gamma(z)&=\int_0^\infty t^{z-1}\,e^{-t}\,\mathrm{d}t\qquad(\Re{z} > 0)\\\Gamma(z+1)&=z\,\Gamma(z)\\\Gamma(n+1)&=n!\\\Gamma\left(\frac{1}{2}+n\right)&=\frac{(2n-1)!!}{2^n}\sqrt{\pi}\qquad(n\ge 1)\\\Gamma\left(\frac{1}{2}-n\right)&=\frac{(-2)^n}{(2n-1)!!}\sqrt{\pi}\qquad(n\ge 1)\\\Gamma(z)\,\Gamma(1-z)&=-z\,\Gamma(z)\,\Gamma(-z)=\frac{\pi}{\sin\pi{z}}\\\end{aligned}](https://latex.codecogs.com/svg.latex?%5Cbegin%7Baligned%7D%5CGamma%28z%29%26%3D%5Cint_0%5E%5Cinfty%20t%5E%7Bz-1%7D%5C%2Ce%5E%7B-t%7D%5C%2C%5Cmathrm%7Bd%7Dt%5Cqquad%28%5CRe%7Bz%7D%20%3E%200%29%5C%5C%5CGamma%28z+1%29%26%3Dz%5C%2C%5CGamma%28z%29%5C%5C%5CGamma%28n+1%29%26%3Dn%21%5C%5C%5CGamma%5Cleft%28%5Cfrac%7B1%7D%7B2%7D+n%5Cright%29%26%3D%5Cfrac%7B%282n-1%29%21%21%7D%7B2%5En%7D%5Csqrt%7B%5Cpi%7D%5Cqquad%28n%5Cge%201%29%5C%5C%5CGamma%5Cleft%28%5Cfrac%7B1%7D%7B2%7D-n%5Cright%29%26%3D%5Cfrac%7B%28-2%29%5En%7D%7B%282n-1%29%21%21%7D%5Csqrt%7B%5Cpi%7D%5Cqquad%28n%5Cge%201%29%5C%5C%5CGamma%28z%29%5C%2C%5CGamma%281-z%29%26%3D-z%5C%2C%5CGamma%28z%29%5C%2C%5CGamma%28-z%29%3D%5Cfrac%7B%5Cpi%7D%7B%5Csin%5Cpi%7Bz%7D%7D%5C%5C%5Cend%7Baligned%7D)
- https://www.boost.org/doc/libs/1_72_0/boost/math/special_functions/gamma.hpp
- https://www.boost.org/doc/libs/1_72_0/libs/math/doc/html/math_toolkit/sf_gamma/tgamma.html
- https://www.boost.org/doc/libs/1_72_0/libs/math/doc/html/math_toolkit/sf_gamma/lgamma.html

## Lanczos近似

- Lanczos近似 : [en.wikipedia: Lanczos approximation](https://en.wikipedia.org/wiki/Lanczos_approximation)
- パラメータ算出 : [https://mrob.com/pub/ries/lanczos-gamma.html](https://mrob.com/pub/ries/lanczos-gamma.html)
- https://www.boost.org/doc/libs/1_72_0/boost/math/special_functions/lanczos.hpp
- https://www.boost.org/doc/libs/1_72_0/libs/math/doc/html/math_toolkit/lanczos.html

## 大浦による実ガンマ関数近似

- [大浦拓哉, ガンマ関数および誤差関数の初等関数近似とその最適化](http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf-j.html) [(ja)](http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf-j.html) [(en)](http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf.html)

![\begin{aligned}\Gamma(x)&\approx\exp((x-0.5)\ln(x+V)-x)\\&\quad\cdot((\cdots(A_n/(x+B_n)+\cdots A_1)/(x+B_1)+A_0)/x+A_r)\qquad(0 < x <\infty)\\\end{aligned}](https://latex.codecogs.com/svg.latex?%5Cbegin%7Baligned%7D%5CGamma%28x%29%26%5Capprox%5Cexp%28%28x-0.5%29%5Cln%28x+V%29-x%29%5C%5C%26%5Cquad%5Ccdot%28%28%5Ccdots%28A_n/%28x+B_n%29+%5Ccdots%20A_1%29/%28x+B_1%29+A_0%29/x+A_r%29%5Cqquad%280%20%3C%20x%20%3C%5Cinfty%29%5C%5C%5Cend%7Baligned%7D)

# 誤差関数

- [ja.wikipedia: 誤差関数](https://ja.wikipedia.org/wiki/%E8%AA%A4%E5%B7%AE%E9%96%A2%E6%95%B0)
- [en.wikipedia: Error function](https://en.wikipedia.org/wiki/Error_function)
- 誤差関数 (error function)
- https://www.boost.org/doc/libs/1_72_0/boost/math/special_functions/erf.hpp
- https://www.boost.org/doc/libs/1_72_0/libs/math/doc/html/math_toolkit/sf_erf/error_function.html
- https://www.boost.org/doc/libs/1_72_0/libs/math/doc/html/math_toolkit/sf_erf/error_inv.html

![\operatorname{erf}(x)=\frac{2}{\sqrt{\pi}}\int_0^x e^{-t^2}\mathrm{d}t](https://latex.codecogs.com/svg.latex?%5Coperatorname%7Berf%7D%28x%29%3D%5Cfrac%7B2%7D%7B%5Csqrt%7B%5Cpi%7D%7D%5Cint_0%5Ex%20e%5E%7B-t%5E2%7D%5Cmathrm%7Bd%7Dt)

- 相補誤差関数 (complementary error function)

![\operatorname{erfc}(x)=1-\operatorname{erf}(x) =\frac{2}{\sqrt{\pi}}\int_x^\infty e^{-t^2}\mathrm{d}t=e^{-x^2}\operatorname{erfcx}(x)](https://latex.codecogs.com/svg.latex?%5Coperatorname%7Berfc%7D%28x%29%3D1-%5Coperatorname%7Berf%7D%28x%29%20%3D%5Cfrac%7B2%7D%7B%5Csqrt%7B%5Cpi%7D%7D%5Cint_x%5E%5Cinfty%20e%5E%7B-t%5E2%7D%5Cmathrm%7Bd%7Dt%3De%5E%7B-x%5E2%7D%5Coperatorname%7Berfcx%7D%28x%29)

- スケーリング相補誤差関数 (scaled complementary error function)

![\operatorname{erfcx}(x)=e^{x^2}\operatorname{erfc}(x) =\frac{2}{\sqrt{\pi}} e^{x^2}\int_x^\infty e^{-t^2}\mathrm{d}t](https://latex.codecogs.com/svg.latex?%5Coperatorname%7Berfcx%7D%28x%29%3De%5E%7Bx%5E2%7D%5Coperatorname%7Berfc%7D%28x%29%20%3D%5Cfrac%7B2%7D%7B%5Csqrt%7B%5Cpi%7D%7D%20e%5E%7Bx%5E2%7D%5Cint_x%5E%5Cinfty%20e%5E%7B-t%5E2%7D%5Cmathrm%7Bd%7Dt)

## 大浦による実誤差関数近似

- [大浦拓哉, ガンマ関数および誤差関数の初等関数近似とその最適化](http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf-j.html) [(ja)](http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf-j.html) [(en)](http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf.html)

![\operatorname{erf}(x)\approx\sum_{k=0}^{N-1}A_k\cdot x^{2k+1}\qquad(|x|\le 0.125)](https://latex.codecogs.com/svg.latex?%5Coperatorname%7Berf%7D%28x%29%5Capprox%5Csum_%7Bk%3D0%7D%5E%7BN-1%7DA_k%5Ccdot%20x%5E%7B2k+1%7D%5Cqquad%28%7Cx%7C%5Cle%200.125%29)


## 大浦による実相補誤差関数近似

- [大浦拓哉, ガンマ関数および誤差関数の初等関数近似とその最適化](http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf-j.html) [(ja)](http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf-j.html) [(en)](http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf.html)

![\begin{aligned}\operatorname{erfc}(x)&\approx E(x)+x\exp(-x^2)\sum_{k=0}^{N-1}\frac{A_k}{x^2+B_k}\qquad(-\infty < x <\infty)\\E(x)&=\begin{cases}2/(1+\exp(\alpha x))&(x <\beta)\\0&(x\ge\beta)\\\end{cases}\end{aligned}](https://latex.codecogs.com/svg.latex?%5Cbegin%7Baligned%7D%5Coperatorname%7Berfc%7D%28x%29%26%5Capprox%20E%28x%29+x%5Cexp%28-x%5E2%29%5Csum_%7Bk%3D0%7D%5E%7BN-1%7D%5Cfrac%7BA_k%7D%7Bx%5E2+B_k%7D%5Cqquad%28-%5Cinfty%20%3C%20x%20%3C%5Cinfty%29%5C%5CE%28x%29%26%3D%5Cbegin%7Bcases%7D2/%281+%5Cexp%28%5Calpha%20x%29%29%26%28x%20%3C%5Cbeta%29%5C%5C0%26%28x%5Cge%5Cbeta%29%5C%5C%5Cend%7Bcases%7D%5Cend%7Baligned%7D)


# ベータ関数

- [ja.wikipedia: ベータ関数](https://ja.wikipedia.org/wiki/%E3%83%99%E3%83%BC%E3%82%BF%E9%96%A2%E6%95%B0)
- [en.wikipedia: Beta function](https://en.wikipedia.org/wiki/Beta_function)
- https://www.boost.org/doc/libs/1_72_0/boost/math/special_functions/beta.hpp
- https://www.boost.org/doc/libs/1_72_0/libs/math/doc/html/math_toolkit/sf_beta/beta_function.html
- https://www.boost.org/doc/libs/1_72_0/libs/math/doc/html/math_toolkit/sf_beta/ibeta_function.html
- https://www.boost.org/doc/libs/1_72_0/libs/math/doc/html/math_toolkit/sf_beta/ibeta_inv_function.html

![\begin{aligned}\operatorname{B}(x,y)&=\int_0^1 t^{x-1}\,(1-t)^{y-1}\,\mathrm{d}t\\\operatorname{B}(x,y)&=\frac{\Gamma(x)\Gamma(y)}{\Gamma(x+y)}\\\operatorname{B}(x,y)&=\operatorname{B}(y,x)\\x\,\operatorname{B}(x,y+1)&=y\,\operatorname{B}(x+1,y)\\\operatorname{B}(x,y)&=\operatorname{B}(x+1,y)+\operatorname{B}(x,y+1)\\\operatorname{B}(x+1,y)&=\operatorname{B}(x,y)\cdot\frac{x}{x+y}\\\operatorname{B}(x,y+1)&=\operatorname{B}(x,y)\cdot\frac{y}{x+y}\\\operatorname{B}(x,y)\cdot\operatorname{B}(x+y,1-y)&=\frac{\pi}{x\sin(\pi y)}\\\operatorname{B}(x,1-x)&=\frac{\pi}{\sin(\pi x)}\\\operatorname{B}(1,x)&=\frac{1}{x}\\\operatorname{B}(1/2,1/2)&=\pi\end{aligned}](https://latex.codecogs.com/svg.latex?%5Cbegin%7Baligned%7D%5Coperatorname%7BB%7D%28x%2Cy%29%26%3D%5Cint_0%5E1%20t%5E%7Bx-1%7D%5C%2C%281-t%29%5E%7By-1%7D%5C%2C%5Cmathrm%7Bd%7Dt%5C%5C%5Coperatorname%7BB%7D%28x%2Cy%29%26%3D%5Cfrac%7B%5CGamma%28x%29%5CGamma%28y%29%7D%7B%5CGamma%28x+y%29%7D%5C%5C%5Coperatorname%7BB%7D%28x%2Cy%29%26%3D%5Coperatorname%7BB%7D%28y%2Cx%29%5C%5Cx%5C%2C%5Coperatorname%7BB%7D%28x%2Cy+1%29%26%3Dy%5C%2C%5Coperatorname%7BB%7D%28x+1%2Cy%29%5C%5C%5Coperatorname%7BB%7D%28x%2Cy%29%26%3D%5Coperatorname%7BB%7D%28x+1%2Cy%29+%5Coperatorname%7BB%7D%28x%2Cy+1%29%5C%5C%5Coperatorname%7BB%7D%28x+1%2Cy%29%26%3D%5Coperatorname%7BB%7D%28x%2Cy%29%5Ccdot%5Cfrac%7Bx%7D%7Bx+y%7D%5C%5C%5Coperatorname%7BB%7D%28x%2Cy+1%29%26%3D%5Coperatorname%7BB%7D%28x%2Cy%29%5Ccdot%5Cfrac%7By%7D%7Bx+y%7D%5C%5C%5Coperatorname%7BB%7D%28x%2Cy%29%5Ccdot%5Coperatorname%7BB%7D%28x+y%2C1-y%29%26%3D%5Cfrac%7B%5Cpi%7D%7Bx%5Csin%28%5Cpi%20y%29%7D%5C%5C%5Coperatorname%7BB%7D%28x%2C1-x%29%26%3D%5Cfrac%7B%5Cpi%7D%7B%5Csin%28%5Cpi%20x%29%7D%5C%5C%5Coperatorname%7BB%7D%281%2Cx%29%26%3D%5Cfrac%7B1%7D%7Bx%7D%5C%5C%5Coperatorname%7BB%7D%281/2%2C1/2%29%26%3D%5Cpi%5Cend%7Baligned%7D)

# 不完全ベータ関数

- 不完全ベータ関数 (incomplete beta function)
- [ja.wikipedia: 不完全ベータ関数](https://ja.wikipedia.org/wiki/%E4%B8%8D%E5%AE%8C%E5%85%A8%E3%83%99%E3%83%BC%E3%82%BF%E9%96%A2%E6%95%B0)
- [en.wikipedia: Beta function #Incomplete beta function](https://en.wikipedia.org/wiki/Beta_function#Incomplete_beta_function)

![\begin{aligned}\operatorname{B}_x(a,b)&=\int_{0}^{x} t^{a-1}\,(1-t)^{b-1}\,\mathrm{d}t\qquad(0\le\Re{z}\le 1)\\\operatorname{B}_x(a,b)&=\operatorname{B}_x(a,b)+\operatorname{B}_{1-x}(b,a)\\\end{aligned}](https://latex.codecogs.com/svg.latex?%5Cbegin%7Baligned%7D%5Coperatorname%7BB%7D_x%28a%2Cb%29%26%3D%5Cint_%7B0%7D%5E%7Bx%7D%20t%5E%7Ba-1%7D%5C%2C%281-t%29%5E%7Bb-1%7D%5C%2C%5Cmathrm%7Bd%7Dt%5Cqquad%280%5Cle%5CRe%7Bz%7D%5Cle%201%29%5C%5C%5Coperatorname%7BB%7D_x%28a%2Cb%29%26%3D%5Coperatorname%7BB%7D_x%28a%2Cb%29+%5Coperatorname%7BB%7D_%7B1-x%7D%28b%2Ca%29%5C%5C%5Cend%7Baligned%7D)

## 正則ベータ関数

- 正則ベータ関数 (regularized beta function) / 正則化不完全ベータ関数 (regularized incomplete beta function)
- [Incomplete Beta Functions-Implementation](https://www.boost.org/doc/libs/1_68_0/libs/math/doc/html/math_toolkit/sf_beta/ibeta_function.html#math_toolkit.sf_beta.ibeta_function.implementation)
- [Significant Digit Computation of the Incomplete Beta Function Ratios. AR DiDonato, AH Morris Jr-1988-dtic.mil](http://www.dtic.mil/dtic/tr/fulltext/u2/a210118.pdf)
- [Armido R. Didonato and Alfred H. Morris, Jr.. 1992. Algorithm 708: Significant digit computation of the incomplete beta function ratios. ACM Trans. Math. Softw. 18, 3(September 1992), 360-373.](https://doi.org/10.1145/131766.131776)
- wolframalpha:
  - [`wolfram language symbol BetaRegularized`](https://www.wolframalpha.com/input/?i=wolfram+language+symbol+BetaRegularized&lang=ja)
  - [`BetaRegularized[x,a,b]`](https://www.wolframalpha.com/input/?i=BetaRegularized%5Bx%2Ca%2Cb%5D&lang=ja)

![\operatorname{I}_x(a,b)=\frac{\operatorname{B}_x(a,b)}{\operatorname{B}(a,b)}\qquad(0\le\Re{x}\le1)](https://latex.codecogs.com/svg.latex?%5Coperatorname%7BI%7D_x%28a%2Cb%29%3D%5Cfrac%7B%5Coperatorname%7BB%7D_x%28a%2Cb%29%7D%7B%5Coperatorname%7BB%7D%28a%2Cb%29%7D%5Cqquad%280%5Cle%5CRe%7Bx%7D%5Cle1%29)

![\frac{\partial}{\partial x}\operatorname{I}_x(a,b)=\frac{x^{a-1}(1-x)^{b-1}}{\operatorname{B}(a,b)}](https://latex.codecogs.com/svg.latex?%5Cfrac%7B%5Cpartial%7D%7B%5Cpartial%20x%7D%5Coperatorname%7BI%7D_x%28a%2Cb%29%3D%5Cfrac%7Bx%5E%7Ba-1%7D%281-x%29%5E%7Bb-1%7D%7D%7B%5Coperatorname%7BB%7D%28a%2Cb%29%7D)

![\operatorname{I}_x(a,b)=1-\operatorname{I}_{1-x}(b,a)](https://latex.codecogs.com/svg.latex?%5Coperatorname%7BI%7D_x%28a%2Cb%29%3D1-%5Coperatorname%7BI%7D_%7B1-x%7D%28b%2Ca%29)

## ニュートン法適用時の変形

不完全ベータ関数にニュートン法を適用する際、a,b,x の値が極端な場合の計算安定度を上げるため、入出力に以下のような関数を適用して順関数・逆関数の計算を行う。

![\tanh^{-1}\left(-1+2\cdot \operatorname{I}_\frac{1}{1+e^{-x}}(a,b)\right)=\frac{1}{2}\ln\frac{\operatorname{I}_\frac{1}{1+e^{-x}}(a,b)}{\operatorname{I}_\frac{1}{1+e^x}(b,a)}](https://latex.codecogs.com/svg.latex?%5Ctanh%5E%7B-1%7D%5Cleft%28-1+2%5Ccdot%20%5Coperatorname%7BI%7D_%5Cfrac%7B1%7D%7B1+e%5E%7B-x%7D%7D%28a%2Cb%29%5Cright%29%3D%5Cfrac%7B1%7D%7B2%7D%5Cln%5Cfrac%7B%5Coperatorname%7BI%7D_%5Cfrac%7B1%7D%7B1+e%5E%7B-x%7D%7D%28a%2Cb%29%7D%7B%5Coperatorname%7BI%7D_%5Cfrac%7B1%7D%7B1+e%5Ex%7D%28b%2Ca%29%7D)

![\frac{\partial}{\partial x}\tanh^{-1}\left(-1+2\cdot \operatorname{I}_\frac{1}{1+e^{-x}}(a,b)\right)=\frac{\left(\frac{1}{1+e^{-x}}\right)^a\left(\frac{1}{1+e^x}\right)^b}{2\operatorname{B}(a,b)\operatorname{I}_\frac{1}{1+e^{-x}}(a,b)\operatorname{I}_\frac{1}{1+e^x}(b,a)}](https://latex.codecogs.com/svg.latex?%5Cinline%20%5Cfrac%7B%5Cpartial%7D%7B%5Cpartial%20x%7D%5Ctanh%5E%7B-1%7D%5Cleft%28-1+2%5Ccdot%20%5Coperatorname%7BI%7D_%5Cfrac%7B1%7D%7B1+e%5E%7B-x%7D%7D%28a%2Cb%29%5Cright%29%3D%5Cfrac%7B%5Cleft%28%5Cfrac%7B1%7D%7B1+e%5E%7B-x%7D%7D%5Cright%29%5Ea%5Cleft%28%5Cfrac%7B1%7D%7B1+e%5Ex%7D%5Cright%29%5Eb%7D%7B2%5Coperatorname%7BB%7D%28a%2Cb%29%5Coperatorname%7BI%7D_%5Cfrac%7B1%7D%7B1+e%5E%7B-x%7D%7D%28a%2Cb%29%5Coperatorname%7BI%7D_%5Cfrac%7B1%7D%7B1+e%5Ex%7D%28b%2Ca%29%7D)

- 参考 wolframalpha:
  - [`ATanh[-1+2BetaRegularized[1/(1+e^(-x)),a,b]]`](https://www.wolframalpha.com/input/?i=ATanh%5B-1%2B2BetaRegularized%5B1%2F%281%2Be%5E%28-x%29%29%2Ca%2Cb%5D%5D&lang=ja)
  - 左辺式のプロット結果、 x の絶対値が大きいと計算・プロットが難しい。
    - [`plot ATanh[-1+2BetaRegularized[1/(1+e^(-x)),120,80]],-0.8<x<1.7,-20<y<20`](https://www.wolframalpha.com/input/?i=plot+ATanh%5B-1%2B2BetaRegularized%5B1%2F%281%2Be%5E%28-x%29%29%2C120%2C80%5D%5D%2C-0.8%3Cx%3C1.7%2C-20%3Cy%3C20&lang=ja)
  - 右辺式のプロット結果、 x の絶対値が大きくても計算・プロットできる。
    - [`plot Log[BetaRegularized[1/(1+e^(-x)),120,80]/BetaRegularized[1/(1+e^x),80,120]]/2,-0.8<x<1.7,-20<y<20`](https://www.wolframalpha.com/input/?i=plot+Log%5BBetaRegularized%5B1%2F%281%2Be%5E%28-x%29%29%2C120%2C80%5D%2FBetaRegularized%5B1%2F%281%2Be%5Ex%29%2C80%2C120%5D%5D%2F2%2C-0.8%3Cx%3C1.7%2C-20%3Cy%3C20&lang=ja)
    - [`plot Log[BetaRegularized[1/(1+e^(-x)),120,80]/BetaRegularized[1/(1+e^x),80,120]]/2,-3<x<3,-120<y<60`](https://www.wolframalpha.com/input/?i=plot+Log%5BBetaRegularized%5B1%2F%281%2Be%5E%28-x%29%29%2C120%2C80%5D%2FBetaRegularized%5B1%2F%281%2Be%5Ex%29%2C80%2C120%5D%5D%2F2%2C-3%3Cx%3C3%2C-120%3Cy%3C60&lang=ja)


## 不完全ベータ関数の連分数展開

![\operatorname{I}_x(a,b)=\frac{x^a(1-x)^b}{a\operatorname{B}(a,b)}\,\cfrac{1}{1+\cfrac{d_1}{1+\cfrac{d_2}{1+\cfrac{d_3}{1+\cfrac{d_4}{1+\,\cdots}}}}}\qquad(0<x<1)](https://latex.codecogs.com/svg.latex?%5Coperatorname%7BI%7D_x%28a%2Cb%29%3D%5Cfrac%7Bx%5Ea%281-x%29%5Eb%7D%7Ba%5Coperatorname%7BB%7D%28a%2Cb%29%7D%5C%2C%5Ccfrac%7B1%7D%7B1+%5Ccfrac%7Bd_1%7D%7B1+%5Ccfrac%7Bd_2%7D%7B1+%5Ccfrac%7Bd_3%7D%7B1+%5Ccfrac%7Bd_4%7D%7B1+%5C%2C%5Ccdots%7D%7D%7D%7D%7D%5Cqquad%280%3Cx%3C1%29)

![d_1=-\frac{a+b}{a+1}x](https://latex.codecogs.com/svg.latex?d_1%3D-%5Cfrac%7Ba+b%7D%7Ba+1%7Dx)

![d_{2n}=\frac{n(b-n)}{(a+2n-1)(a+2n)}x](https://latex.codecogs.com/svg.latex?d_%7B2n%7D%3D%5Cfrac%7Bn%28b-n%29%7D%7B%28a+2n-1%29%28a+2n%29%7Dx)

![d_{2n+1}=-\frac{(a+n)(a+b+n)}{(a+2n)(a+2n+1)}x](https://latex.codecogs.com/svg.latex?d_%7B2n+1%7D%3D-%5Cfrac%7B%28a+n%29%28a+b+n%29%7D%7B%28a+2n%29%28a+2n+1%29%7Dx)

## 連分数展開の計算

連分数の計算を行列の積として計算し、オーバーフロー・アンダーフローを防ぐため定期的に分子・分母に適当な値を乗算する。

![\begin{aligned}&\quad\begin{pmatrix}P_n&Q_n\\R_n&S_n\end{pmatrix}\\&=\begin{pmatrix}0&a_1\\1&b_1\end{pmatrix}\begin{pmatrix}0&a_2\\1&b_2\end{pmatrix}\begin{pmatrix}0&a_3\\1&b_3\end{pmatrix}\begin{pmatrix}0&a_4\\1&b_4\end{pmatrix}\cdots\begin{pmatrix}0&a_{n-1}\\1&b_{n-1}\end{pmatrix}\begin{pmatrix}0&a_n\\1&b_n\end{pmatrix}\\&=\begin{pmatrix}a_1&a_1b_2\\b_1&a_2+b_1b_2\end{pmatrix}\begin{pmatrix}a_3&a_3b_4\\b_3&a_4+b_3b_4\end{pmatrix}\cdots\begin{pmatrix}a_{2m-1}&a_{2m-1}b_{2m}\\b_{2m-1}&a_{2m}+b_{2m-1}b_{2m}\end{pmatrix}\cdots\begin{pmatrix}0&a_{n-1}\\1&b_{n-1}\end{pmatrix}\begin{pmatrix}0&a_n\\1&b_n\end{pmatrix}\\\end{aligned}](https://latex.codecogs.com/svg.latex?%5Cbegin%7Baligned%7D%26%5Cquad%5Cbegin%7Bpmatrix%7DP_n%26Q_n%5C%5CR_n%26S_n%5Cend%7Bpmatrix%7D%5C%5C%26%3D%5Cbegin%7Bpmatrix%7D0%26a_1%5C%5C1%26b_1%5Cend%7Bpmatrix%7D%5Cbegin%7Bpmatrix%7D0%26a_2%5C%5C1%26b_2%5Cend%7Bpmatrix%7D%5Cbegin%7Bpmatrix%7D0%26a_3%5C%5C1%26b_3%5Cend%7Bpmatrix%7D%5Cbegin%7Bpmatrix%7D0%26a_4%5C%5C1%26b_4%5Cend%7Bpmatrix%7D%5Ccdots%5Cbegin%7Bpmatrix%7D0%26a_%7Bn-1%7D%5C%5C1%26b_%7Bn-1%7D%5Cend%7Bpmatrix%7D%5Cbegin%7Bpmatrix%7D0%26a_n%5C%5C1%26b_n%5Cend%7Bpmatrix%7D%5C%5C%26%3D%5Cbegin%7Bpmatrix%7Da_1%26a_1b_2%5C%5Cb_1%26a_2+b_1b_2%5Cend%7Bpmatrix%7D%20%5Cbegin%7Bpmatrix%7Da_3%26a_3b_4%5C%5Cb_3%26a_4+b_3b_4%5Cend%7Bpmatrix%7D%5Ccdots%5Cbegin%7Bpmatrix%7Da_%7B2m-1%7D%26a_%7B2m-1%7Db_%7B2m%7D%5C%5Cb_%7B2m-1%7D%26a_%7B2m%7D+b_%7B2m-1%7Db_%7B2m%7D%5Cend%7Bpmatrix%7D%5Ccdots%5Cbegin%7Bpmatrix%7D0%26a_%7Bn-1%7D%5C%5C1%26b_%7Bn-1%7D%5Cend%7Bpmatrix%7D%5Cbegin%7Bpmatrix%7D0%26a_n%5C%5C1%26b_n%5Cend%7Bpmatrix%7D%5C%5C%5Cend%7Baligned%7D)

![\begin{aligned}S_n(x)&=b_0+\frac{P_n\,x+Q_n}{R_n\,x+S_n}\\&=b_0+\cfrac{a_1}{b_1+\cfrac{a_2}{b_2+\cfrac{a_3}{b_3+\,\cdots\,\cfrac{a_n}{b_n+x}}}}\\&\cong b_0+\frac{Q_n}{S_n}\qquad\left(\because\lim_{n\to\infty}\frac{P_n}{R_n}=\frac{Q_n}{S_n}\right)\end{aligned}](https://latex.codecogs.com/svg.latex?%5Cbegin%7Baligned%7DS_n%28x%29%26%3Db_0+%5Cfrac%7BP_n%5C%2Cx+Q_n%7D%7BR_n%5C%2Cx+S_n%7D%5C%5C%26%3Db_0+%5Ccfrac%7Ba_1%7D%7Bb_1+%5Ccfrac%7Ba_2%7D%7Bb_2+%5Ccfrac%7Ba_3%7D%7Bb_3+%5C%2C%5Ccdots%5C%2C%5Ccfrac%7Ba_n%7D%7Bb_n+x%7D%7D%7D%7D%5C%5C%26%5Ccong%20b_0+%5Cfrac%7BQ_n%7D%7BS_n%7D%5Cqquad%5Cleft%28%5Cbecause%5Clim_%7Bn%5Cto%5Cinfty%7D%5Cfrac%7BP_n%7D%7BR_n%7D%3D%5Cfrac%7BQ_n%7D%7BS_n%7D%5Cright%29%5Cend%7Baligned%7D)
