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

= モバイル通信における繰り返しセル数等に関する計算プログラム
