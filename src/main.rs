// 宣言型マクロ
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// 手続き型マクロ（カスタムderiveマクロ）
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    // 宣言型マクロ
    let v: Vec<u32> = vec![1, 2, 3];
    println!("{:?}", v);

    // 手続き型マクロ（カスタムderiveマクロ）
    Pancakes::hello_macro();
}
