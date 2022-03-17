// 選択ソートと挿入ソートのアルゴリズムを Rust で実装してください。

// 仕様
// -1000 以上 1000 以下のランダムな整数を100回発生させ、その整数を昇順に並び替える。

use rand::Rng;

fn main() {
    println!("Select Sort!");

    const MIN: i32 = -1000;
    const MAX: i32 = 1000;
    const NUM: usize = 100;
    let rand_vec = gen_rand_vec(MIN, MAX, NUM);

    let sel_sort_arr: Vec<i32> = select_sort(&rand_vec);
    println!("selection sort: {:?}", sel_sort_arr);

    let ins_sort_arr: Vec<i32> = insert_sort(&rand_vec);
    println!("insert sort: {:?}", ins_sort_arr);
}

/**
 * gen_rand_vec
 * 
 * : ランダムな整数を発生させ、ベクタ配列として取得します。
 * : 引数で整数の範囲、回数を指定します。
 * 
 * * `min` - 範囲の下限
 * * `max` - 範囲の上限
 * * `num` - 回数
 */
fn gen_rand_vec (min: i32, max: i32, num: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..(num + 1)).map(|_| rng.gen_range(min, max)).collect()
}

fn select_sort (arr: &Vec<i32>) -> Vec<i32> {
    let mut a: Vec<i32> = arr.clone();
    for i in 0..a.len() {
        let min = a[i + 1..].iter().enumerate()
            .fold(i, |acc, (j, val)| if *val < a[acc] { j + i + 1 } else { acc });
        if min != i { a.swap(i, min) };
    }
    a
}

fn insert_sort (arr: &Vec<i32>) -> Vec<i32> {
    arr.iter()
        .fold(vec![], |mut acc, &x| {
            if acc.len() == 0 { 
                acc.push(x); 
                return acc;
            }

            let exists = acc.iter().any(|&y| x <= y);
            if exists {
                let j = acc.iter().position(|&y| x <= y).unwrap();
                acc.insert(j, x);
            }

            let &y = acc.last().unwrap();
            if x > y {
                acc.push(x);
            }

            acc
        }
    )
}