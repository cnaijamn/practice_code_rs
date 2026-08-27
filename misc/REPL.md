Rust REPL
=========

* REPL : Read-Eval-Print Loop
* `evcxr`
  * Rust向けのREPLコマンド
  * Rustコードを1行ずつ実行できる
  * 変数や関数を保持したまま試せる
  * Cargoのクレートをその場で追加できる
  * Jupyterカーネルとしても利用可能

`evcxr`インストール
-------------------

    $ cargo install evcxr_repl

起動
----

    $ evcxr
    Welcome to evcxr. For help, type :help
    >>

例
--

    >> 7 * 3
    21

    >> let x = 10;
    >> x * 2
    20

    >> :type x
    x: i32

    >> :dep regex = "1"
    >> use regex::Regex;
    >> Regex::new(r"\d+").unwrap().is_match("abc123")
    true

    >> :dep rand = "0.10"
    >> let n: u8 = rand::random_range(0..=255);
    >> n
    104

    >> #[derive(Debug)]
       struct Aaa { name: String }
    >> let a = Aaa { name: "あいうえお".to_string() };
    >> a
    Aaa { name: "あいうえお" }

    >> use std::f64::consts::PI;
    >> PI
    3.141592653589793

    >> (PI / 2.0).sin()
    1.0

    >> PI.cos()
    -1.0

    >> let mut vec: Vec<String> = Vec::new();
    >> vec.push(String::from("こんにちは"));
    >> vec.push(String::from("世界"));
    >> vec
    ["こんにちは", "世界"]
    >> vec.join("、")
    "こんにちは、世界"

度数法からラジアンへ変換

    >> use std::f64::consts::PI;
    >> for deg in [0.0, 30.0, 45.0, 60.0, 90.0] {
        let rad = deg * PI / 180.0;
        println!("{:>3}°  sin={:.3}  cos={:.3}", deg, rad.sin(), rad.cos());
    }
      0°  sin=0.000  cos=1.000
     30°  sin=0.500  cos=0.866
     45°  sin=0.707  cos=0.707
     60°  sin=0.866  cos=0.500
     90°  sin=1.000  cos=0.000

REPL時にサンプルや学習コードに`Debug``Clone``PartialEq`を付けると非常に便利。  

    >> #[derive(Debug, Clone, PartialEq)]
       struct Aaa { name: String }

    >> let a = Aaa { name: "あいうえお".to_string() };
    >> let b: Aaa = a.clone();
    >> b
    Aaa { name: "あいうえお" }

    >> a == b
    true
    >> a != b
    false

クレート(Crate)を使う
---------------------

例. `uuid`

    $ cargo new hoge
    $ cd hoge
    $ cargo add uuid --features "v4 serde"
    $ evcxr
    >> :dep uuid = { version = "1", features = ["v4", "serde"] }
    >> use uuid::Uuid;
    >> let id = Uuid::new_v4();
    >> id
    c080c2dd-0c5b-4943-852c-da4669b50aea


**_TODO_**
----------
