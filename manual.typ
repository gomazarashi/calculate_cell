#set text(lang: "ja") // 言語を日本語に設定
#set text(font: ("New Computer Modern","Harano Aji Mincho"),size: 10pt,) // フォントを設定
#show figure.where(
  kind: table
): set figure.caption(position: top) // 表におけるキャプションを上部に表示するよう設定

#show heading: set text(font: ("Harano Aji Gothic"),weight:500 )
#set heading( numbering: "1.1.")
#import "@preview/codelst:2.0.1": sourcecode
#show raw: set text(font: ("DejaVu Sans Mono","Harano Aji Gothic"))

#set page(numbering: "1",)

= 概要
このプログラムは、モバイル通信における繰り返しセル数に関する計算を行うプログラムです。

モバイル通信における繰り返しセル数$N$は以下の式で表されます。
$ N=i^2+j^2+i j $
ただし$N$は自然数であり、$i$、$j$は$i>=j$を満たす整数です。
このプログラムは、繰り返しセル数$N$および$i$、$j$に関する以下の計算を行います。

+ 繰り返しセル数$N$が与えられたとき、$i$、$j$を求める。
+ $i$、$j$が与えられたとき、繰り返しセル数$N$を求める。
+ $N$の範囲が与えられたとき、その範囲内において$N$が取りうる値を求める。

