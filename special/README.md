# 不完全ベータ関数 開発メモ

## ニュートン法適用時の変形

<!--
```math
\begin{aligned}
\operatorname{I}_x(a,b)&=\frac{\operatorname{B}_x(a,b)}{\operatorname{B}(a,b)}\\
\frac{\partial}{\partial x}\operatorname{I}_x(a,b)&=\frac{x^{a-1}(1-x)^{b-1}}{\operatorname{B}(a,b)}\\
\tanh^{-1}\left(-1+2\cdot \operatorname{I}_\frac{1}{1+e^{-x}}(a,b)\right)&=\frac{1}{2}\ln\frac{\operatorname{I}_\frac{1}{1+e^{-x}}(a,b)}{\operatorname{I}_\frac{1}{1+e^x}(b,a)}\\
\frac{\partial}{\partial x}\tanh^{-1}\left(-1+2\cdot \operatorname{I}_\frac{1}{1+e^{-x}}(a,b)\right)&=\frac{\left(\frac{1}{1+e^{-x}}\right)^a\left(\frac{1}{1+e^x}\right)^b}{2\operatorname{B}(a,b)\operatorname{I}_\frac{1}{1+e^{-x}}(a,b)\operatorname{I}_\frac{1}{1+e^x}(b,a)}\\
\end{aligned}
```
-->

![\operatorname{I}_x(a,b)=\frac{\operatorname{B}_x(a,b)}{\operatorname{B}(a,b)}](https://latex.codecogs.com/svg.latex?%5Coperatorname%7BI%7D_x%28a%2Cb%29%3D%5Cfrac%7B%5Coperatorname%7BB%7D_x%28a%2Cb%29%7D%7B%5Coperatorname%7BB%7D%28a%2Cb%29%7D)

![\frac{\partial}{\partial x}\operatorname{I}_x(a,b)=\frac{x^{a-1}(1-x)^{b-1}}{\operatorname{B}(a,b)}](https://latex.codecogs.com/svg.latex?%5Cfrac%7B%5Cpartial%7D%7B%5Cpartial%20x%7D%5Coperatorname%7BI%7D_x%28a%2Cb%29%3D%5Cfrac%7Bx%5E%7Ba-1%7D%281-x%29%5E%7Bb-1%7D%7D%7B%5Coperatorname%7BB%7D%28a%2Cb%29%7D)

- 参考 wolframalpha:
  - [`wolfram language symbol BetaRegularized`](https://www.wolframalpha.com/input/?i=wolfram+language+symbol+BetaRegularized&lang=ja)
  - [`BetaRegularized[x,a,b]`](https://www.wolframalpha.com/input/?i=BetaRegularized%5Bx%2Ca%2Cb%5D&lang=ja)

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

<!--
```math
\begin{aligned}
\operatorname{I}_x(a,b)&=\frac{\operatorname{B}_x(a,b)}{\operatorname{B}(a,b)}\qquad(0\le\Re{x}\le 1)\\
\operatorname{I}_x(a,b)&=\frac{x^a(1-x)^b}{a\operatorname{B}(a,b)}\left(\frac{d_0}{1+}\frac{d_1}{1+}\frac{d_2}{1+}\frac{d_3}{1+}\frac{d_4}{1+}\cdots\right)\qquad(0 \lt x \lt 1)\\
d_0&=1\\
d_1&=-\frac{a+b}{a+1}x\\
d_{2n}&=\frac{n(b-n)}{(a+2n-1)(a+2n)}x\\
d_{2n+1}&=-\frac{(a+n)(a+b+n)}{(a+2n)(a+2n+1)}x\\
\end{aligned}
```
-->

![\operatorname{I}_x(a,b)=\frac{\operatorname{B}_x(a,b)}{\operatorname{B}(a,b)}\qquad(0\le\Re{x}\le 1)](https://latex.codecogs.com/svg.latex?%5Coperatorname%7BI%7D_x%28a%2Cb%29%3D%5Cfrac%7B%5Coperatorname%7BB%7D_x%28a%2Cb%29%7D%7B%5Coperatorname%7BB%7D%28a%2Cb%29%7D%5Cqquad%280%5Cle%5CRe%7Bx%7D%5Cle%201%29)

![\operatorname{I}_x(a,b)=\frac{x^a(1-x)^b}{a\operatorname{B}(a,b)}\left(\frac{d_0}{1+}\frac{d_1}{1+}\frac{d_2}{1+}\frac{d_3}{1+}\frac{d_4}{1+}\cdots\right)\qquad(0 \lt x \lt 1)](https://latex.codecogs.com/svg.latex?%5Coperatorname%7BI%7D_x%28a%2Cb%29%3D%5Cfrac%7Bx%5Ea%281-x%29%5Eb%7D%7Ba%5Coperatorname%7BB%7D%28a%2Cb%29%7D%5Cleft%28%5Cfrac%7Bd_0%7D%7B1+%7D%5Cfrac%7Bd_1%7D%7B1+%7D%5Cfrac%7Bd_2%7D%7B1+%7D%5Cfrac%7Bd_3%7D%7B1+%7D%5Cfrac%7Bd_4%7D%7B1+%7D%5Ccdots%5Cright%29%5Cqquad%280%3Cx%3C1%29)

![d_0=1](https://latex.codecogs.com/svg.latex?d_0%3D1)

![d_1=-\frac{a+b}{a+1}x](https://latex.codecogs.com/svg.latex?d_1%3D-%5Cfrac%7Ba+b%7D%7Ba+1%7Dx)

![d_{2n}=\frac{n(b-n)}{(a+2n-1)(a+2n)}x](https://latex.codecogs.com/svg.latex?d_%7B2n%7D%3D%5Cfrac%7Bn%28b-n%29%7D%7B%28a+2n-1%29%28a+2n%29%7Dx)

![d_{2n+1}=-\frac{(a+n)(a+b+n)}{(a+2n)(a+2n+1)}x](https://latex.codecogs.com/svg.latex?d_%7B2n+1%7D%3D-%5Cfrac%7B%28a+n%29%28a+b+n%29%7D%7B%28a+2n%29%28a+2n+1%29%7Dx)

## 連分数展開の計算

連分数の計算を行列の積として計算し、オーバーフロー・アンダーフローを防ぐため定期的に分子・分母に適当な値を乗算する。

<!--
```math
\begin{aligned}&\quad\begin{pmatrix}P_n&Q_n\\R_n&S_n\end{pmatrix}\\&=\begin{pmatrix}0&a_1\\1&b_1\end{pmatrix}\begin{pmatrix}0&a_2\\1&b_2\end{pmatrix}\begin{pmatrix}0&a_3\\1&b_3\end{pmatrix}\begin{pmatrix}0&a_4\\1&b_4\end{pmatrix}\cdots\begin{pmatrix}0&a_{n-1}\\1&b_{n-1}\end{pmatrix}\begin{pmatrix}0&a_n\\1&b_n\end{pmatrix}\\&=\begin{pmatrix}a_1&a_1b_2\\b_1&a_2+b_1b_2\end{pmatrix}
\begin{pmatrix}a_3&a_3b_4\\b_3&a_4+b_3b_4\end{pmatrix}\cdots\begin{pmatrix}a_{2m-1}&a_{2m-1}b_{2m}\\b_{2m-1}&a_{2m}+b_{2m-1}b_{2m}\end{pmatrix}\cdots\begin{pmatrix}0&a_{n-1}\\1&b_{n-1}\end{pmatrix}\begin{pmatrix}0&a_n\\1&b_n\end{pmatrix}\\\end{aligned}
```
-->

![\begin{aligned}&\quad\begin{pmatrix}P_n&Q_n\\R_n&S_n\end{pmatrix}\\&=\begin{pmatrix}0&a_1\\1&b_1\end{pmatrix}\begin{pmatrix}0&a_2\\1&b_2\end{pmatrix}\begin{pmatrix}0&a_3\\1&b_3\end{pmatrix}\begin{pmatrix}0&a_4\\1&b_4\end{pmatrix}\cdots\begin{pmatrix}0&a_{n-1}\\1&b_{n-1}\end{pmatrix}\begin{pmatrix}0&a_n\\1&b_n\end{pmatrix}\\&=\begin{pmatrix}a_1&a_1b_2\\b_1&a_2+b_1b_2\end{pmatrix}
\begin{pmatrix}a_3&a_3b_4\\b_3&a_4+b_3b_4\end{pmatrix}\cdots\begin{pmatrix}a_{2m-1}&a_{2m-1}b_{2m}\\b_{2m-1}&a_{2m}+b_{2m-1}b_{2m}\end{pmatrix}\cdots\begin{pmatrix}0&a_{n-1}\\1&b_{n-1}\end{pmatrix}\begin{pmatrix}0&a_n\\1&b_n\end{pmatrix}\\\end{aligned}
](https://latex.codecogs.com/svg.latex?%5Cbegin%7Baligned%7D%26%5Cquad%5Cbegin%7Bpmatrix%7DP_n%26Q_n%5C%5CR_n%26S_n%5Cend%7Bpmatrix%7D%5C%5C%26%3D%5Cbegin%7Bpmatrix%7D0%26a_1%5C%5C1%26b_1%5Cend%7Bpmatrix%7D%5Cbegin%7Bpmatrix%7D0%26a_2%5C%5C1%26b_2%5Cend%7Bpmatrix%7D%5Cbegin%7Bpmatrix%7D0%26a_3%5C%5C1%26b_3%5Cend%7Bpmatrix%7D%5Cbegin%7Bpmatrix%7D0%26a_4%5C%5C1%26b_4%5Cend%7Bpmatrix%7D%5Ccdots%5Cbegin%7Bpmatrix%7D0%26a_%7Bn-1%7D%5C%5C1%26b_%7Bn-1%7D%5Cend%7Bpmatrix%7D%5Cbegin%7Bpmatrix%7D0%26a_n%5C%5C1%26b_n%5Cend%7Bpmatrix%7D%5C%5C%26%3D%5Cbegin%7Bpmatrix%7Da_1%26a_1b_2%5C%5Cb_1%26a_2+b_1b_2%5Cend%7Bpmatrix%7D%20%5Cbegin%7Bpmatrix%7Da_3%26a_3b_4%5C%5Cb_3%26a_4+b_3b_4%5Cend%7Bpmatrix%7D%5Ccdots%5Cbegin%7Bpmatrix%7Da_%7B2m-1%7D%26a_%7B2m-1%7Db_%7B2m%7D%5C%5Cb_%7B2m-1%7D%26a_%7B2m%7D+b_%7B2m-1%7Db_%7B2m%7D%5Cend%7Bpmatrix%7D%5Ccdots%5Cbegin%7Bpmatrix%7D0%26a_%7Bn-1%7D%5C%5C1%26b_%7Bn-1%7D%5Cend%7Bpmatrix%7D%5Cbegin%7Bpmatrix%7D0%26a_n%5C%5C1%26b_n%5Cend%7Bpmatrix%7D%5C%5C%5Cend%7Baligned%7D)

<!--
```math
\begin{aligned}S_n(x)&=b_0+\frac{P_n\,x+Q_n}{R_n\,x+S_n}\\&=b_0+\cfrac{a_1}{b_1+\cfrac{a_2}{b_2+\cfrac{a_3}{b_3+\,\cdots\,\cfrac{a_n}{b_n+x}}}}\\&\cong b_0+\frac{Q_n}{S_n}\qquad\left(\because\lim_{n\to\infty}\frac{P_n}{R_n}=\frac{Q_n}{S_n}\right)\end{aligned}
```
-->

![\begin{aligned}S_n(x)&=b_0+\frac{P_n\,x+Q_n}{R_n\,x+S_n}\\&=b_0+\cfrac{a_1}{b_1+\cfrac{a_2}{b_2+\cfrac{a_3}{b_3+\,\cdots\,\cfrac{a_n}{b_n+x}}}}\\&\cong b_0+\frac{Q_n}{S_n}\qquad\left(\because\lim_{n\to\infty}\frac{P_n}{R_n}=\frac{Q_n}{S_n}\right)\end{aligned}](https://latex.codecogs.com/svg.latex?%5Cbegin%7Baligned%7DS_n%28x%29%26%3Db_0+%5Cfrac%7BP_n%5C%2Cx+Q_n%7D%7BR_n%5C%2Cx+S_n%7D%5C%5C%26%3Db_0+%5Ccfrac%7Ba_1%7D%7Bb_1+%5Ccfrac%7Ba_2%7D%7Bb_2+%5Ccfrac%7Ba_3%7D%7Bb_3+%5C%2C%5Ccdots%5C%2C%5Ccfrac%7Ba_n%7D%7Bb_n+x%7D%7D%7D%7D%5C%5C%26%5Ccong%20b_0+%5Cfrac%7BQ_n%7D%7BS_n%7D%5Cqquad%5Cleft%28%5Cbecause%5Clim_%7Bn%5Cto%5Cinfty%7D%5Cfrac%7BP_n%7D%7BR_n%7D%3D%5Cfrac%7BQ_n%7D%7BS_n%7D%5Cright%29%5Cend%7Baligned%7D)
