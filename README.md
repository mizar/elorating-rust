# 勝率と、それに相当するイロレーティング差の信頼区間計算

対局者Aと対局者Bがとある条件でベルヌーイ試行と見なせる対局を行う。対局者Aが対局で勝つ（1回のベルヌーイ試行に成功する）確率を ![p](https://latex.codecogs.com/svg.latex?p) とし、 ![n](https://latex.codecogs.com/svg.latex?n) 回対局を行う場合、勝った回数 ![X](https://latex.codecogs.com/svg.latex?X) が ![k](https://latex.codecogs.com/svg.latex?k) 以下となる確率を ![F(k;\,n,p)=\Pr\left(X\le k\right)](https://latex.codecogs.com/svg.latex?F%28k%3B%5C%2Cn%2Cp%29%3D%5CPr%5Cleft%28X%5Cle%20k%5Cright%29) とする。

ここで逆に、例えば ![F(k;\,n,p)=\Pr\left(X\le k\right)=0.025](https://latex.codecogs.com/svg.latex?F%28k%3B%5C%2Cn%2Cp%29%3D%5CPr%5Cleft%28X%5Cle%20k%5Cright%29%3D0.025) となる勝率 ![p](https://latex.codecogs.com/svg.latex?p) の値を求める事で、その試行回数 ![n](https://latex.codecogs.com/svg.latex?n) で得られた結果はどれだけ確からしいか、実際の勝率 ![p](https://latex.codecogs.com/svg.latex?p) はどれぐらいの範囲の中にあると見なせそうか、という事を考察する試みです。

## 正則化不完全ベータ関数と二項分布の累積分布関数との関連

- https://en.wikipedia.org/wiki/Beta_function#Incomplete_beta_function

>正則化不完全ベータ関数は、ベータ分布の累積分布関数であり、二項分布の確率変数 ![X](https://latex.codecogs.com/svg.latex?X) の累積分布関数 ![F(k;\,n,p)=\Pr\left(X\le k\right)](https://latex.codecogs.com/svg.latex?F%28k%3B%5C%2Cn%2Cp%29%3D%5CPr%5Cleft%28X%5Cle%20k%5Cright%29) に関連している。ここで、1回のベルヌーイ試行で成功する確率を ![p](https://latex.codecogs.com/svg.latex?p) 、ベルヌーイ試行する回数を ![n](https://latex.codecogs.com/svg.latex?n) とする:
>
>![F(k;\,n,p)=\Pr\left(X\le k\right)=I_{1-p}(n-k,k+1)=1-I_p(k+1,n-k).](https://latex.codecogs.com/svg.latex?F%28k%3B%5C%2Cn%2Cp%29%3D%5CPr%5Cleft%28X%5Cle%20k%5Cright%29%3DI_%7B1-p%7D%28n-k%2Ck+1%29%3D1-I_p%28k+1%2Cn-k%29.)

## ベルヌーイ試行

- https://ja.wikipedia.org/wiki/%E3%83%99%E3%83%AB%E3%83%8C%E3%83%BC%E3%82%A4%E8%A9%A6%E8%A1%8C

> 確率論や統計学において、ベルヌーイ試行（ベルヌーイしこう、英語: Bernoulli trial）または二項試行（にこうしこう、英語: binomial trial）とは、取り得る結果が「成功」「失敗」の2つのみであり、各試行において成功の確率が同じであるランダム試行である。この名前は、17世紀のスイスの数学者であるヤコブ・ベルヌーイにちなんで名付けられた。ベルヌーイは、1713年の著書『推測法』(Ars Conjectandi)でこの試行を分析した。

## イロレーティング

- https://ja.wikipedia.org/wiki/%E3%82%A4%E3%83%AD%E3%83%AC%E3%83%BC%E3%83%86%E3%82%A3%E3%83%B3%E3%82%B0
