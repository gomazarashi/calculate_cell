# i,jから繰り返しセル数Nを計算する関数
def calculate_cell(i: int, j: int) -> int:
    return i**2 + j**2 + i * j


# 繰り返しセル数Nからi,jを計算する関数(i>=j)
def calculate_i_j(n: int) -> list:
    if n <= 0:
        raise ValueError("Nは正の整数でなければなりません。")

    i_j_list = []
    max_i = int(n**0.5)
    for i in range(max_i + 1):
        for j in range(i + 1):
            if calculate_cell(i, j) == n:
                i_j_list.append((i, j))
            elif calculate_cell(i, j) > n:
                break

    return i_j_list


# 与えられた範囲を満たすNを計算する関数
def calculate_N(min_val: int, max_val: int) -> list:
    if min_val <= 0 or max_val <= 0:
        raise ValueError("範囲の値は正の整数でなければなりません。")
    if min_val > max_val:
        raise ValueError("最小値は最大値以下でなければなりません。")

    n_list = []
    for i in range(min_val, max_val + 1):
        if calculate_i_j(i) != []:
            n_list.append(i)
    return n_list


def main():
    while True:
        print(
            "計算モードを選択して下さい。\n1: 繰り返しセル数Nからi,jを計算\n2: i,jから繰り返しセル数Nを計算\n3: 与えられた範囲を満たすNを計算\n0: 終了"
        )
        try:
            mode = int(input())
        except ValueError:
            print("正しい数値を入力してください。\n")
            continue

        try:
            match mode:
                case 1:
                    n = int(input("繰り返しセル数Nを入力して下さい: "))
                    i_j_list = calculate_i_j(n)
                    if len(i_j_list) == 0:
                        print("与えられたNを満たす非負整数の組(i,j)は存在しません。")
                    else:
                        print("与えられたNを満たす非負整数の組(i,j)は以下の通りです。")
                        for i, j in i_j_list:
                            print(f"i = {i}, j = {j}")

                case 2:
                    i = int(input("iを入力して下さい: "))
                    j = int(input("jを入力して下さい: "))
                    if i==0 and j==0:
                        print("i、jが共に0になることはありません")
                        break
                    else:
                        print(calculate_cell(i, j))

                case 3:
                    min_val = int(input("最小値を入力して下さい: "))
                    max_val = int(input("最大値を入力して下さい: "))
                    print(calculate_N(min_val, max_val))

                case 0:
                    print("終了します")
                    break
                case _:
                    print("正しいモードを選択して下さい")
        except ValueError as e:
            print(e)
        print("\n")


if __name__ == "__main__":
    main()
