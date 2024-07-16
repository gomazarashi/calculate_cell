use std::io;

// 標準入力を読み込む関数
fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("行の読み込みに失敗しました");
    input
}

fn calculate_cell(i: i32, j: i32) -> i32 {
    i.pow(2) + j.pow(2) + i * j
}

fn calculate_i_j(n: i32) -> Vec<(i32, i32)> {
    let mut i_j_list = Vec::new();
    let max_i = (n as f64).sqrt() as i32;
    for i in 0..=max_i {
        for j in 0..=i {
            if calculate_cell(i, j) == n {
                i_j_list.push((i, j));
            } else if calculate_cell(i, j) > n {
                break;
            }
        }
    }
    i_j_list
}

fn calculate_n(min: i32, max: i32) -> Vec<i32> {
    let mut n_list = Vec::new();
    for i in min..=max {
        if !calculate_i_j(i).is_empty() {
            n_list.push(i);
        }
    }
    n_list
}

fn main() {
    loop {
        println!(
            "計算モードを選択して下さい。\n1: 繰り返しセル数Nからi,jを計算\n2: i,jから繰り返しセル数Nを計算\n3: 与えられた範囲を満たすNを計算\n0: 終了"
        );

        match read_input().trim().parse::<i32>().unwrap() {
            1 => {
                println!("繰り返しセル数Nを入力して下さい:");
                let n = read_input();
                let n: i32 = match n.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                let i_j_list = calculate_i_j(n);
                if i_j_list.is_empty() {
                    println!("与えられたNを満たす非負整数の組(i,j)は存在しません。");
                } else {
                    println!("与えられたNを満たす非負整数の組(i,j)は以下の通りです。");
                    for (i, j) in i_j_list {
                        println!("i = {}, j = {}", i, j);
                    }
                }
                println!("\n");
            }
            2 => {
                println!("iを入力して下さい:");
                let i = read_input();
                let i: i32 = match i.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                println!("jを入力して下さい:");
                let j = String::new();
                let j: i32 = match j.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                println!("{}", calculate_cell(i, j));
                println!("\n");
            }
            3 => {
                println!("最小値を入力して下さい:");
                let min = read_input();
                let min: i32 = match min.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                println!("最大値を入力して下さい:");
                let max =read_input();
                
                let max: i32 = match max.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                let n_list = calculate_n(min, max);
                println!("{:?}", n_list);
                println!("\n");
            }
            0 => {
                println!("終了します");
                break;
            }
            _ => {
                println!("正しいモードを選択して下さい");
                println!("\n");
            }
        }
    }
}
