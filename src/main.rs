extern crate rand;
extern crate serde;
extern crate serde_json;

use rand::seq::SliceRandom;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::io;

const URL: &str = "http://example.com/";

#[derive(Debug)]
enum State {
    Start,
    Question,
    Check,
}

#[derive(Deserialize, Debug, Clone)]
struct ApiGet {
    q_id: u32,
    q_answer: String,
    q_question: String,
    q_category_id: u32,
    c_name: String,
}

#[derive(Debug)]
struct Cate {
    category: String,
    count: u32,
}

fn question_extraction(
    q_vec: &Vec<ApiGet>,
    cate_select: &str,
) -> Result<Vec<ApiGet>, Box<std::error::Error>> {

    let mut new_vec: Vec<ApiGet> = Vec::new();
    let cs: Vec<&str> = cate_select.trim().split_whitespace().collect();
    for v1 in q_vec {
        for v2 in &cs {
            let num: u32 = v2.parse()?;
            if num == 0 {
                new_vec.push(v1.clone());
            } else if v1.q_category_id == num {
                new_vec.push(v1.clone());
            }
        }
    }
    Ok(new_vec)
}

fn totalization(q_vec: &Vec<ApiGet>) -> BTreeMap<u32, Cate> {
    let mut map: BTreeMap<u32, Cate> = BTreeMap::new();
    for v in q_vec {
        if let Some(x) = map.get_mut(&v.q_category_id) {
            x.count += 1;
        } else {
            let obj = Cate {
                category: v.c_name.clone(),
                count: 1,
            };
            map.insert(v.q_category_id, obj);
        }
    }
    map
}

fn vev_shuffle(q_vec: &mut Vec<ApiGet>) {
    let mut rng = rand::thread_rng();
    q_vec.shuffle(&mut rng);
}

fn make_vec(text: &str) -> Vec<ApiGet> {
    serde_json::from_str(text).unwrap()
}

fn get_json(url: &str) -> Result<String, Box<std::error::Error>> {
    let res = reqwest::get(url)?.text()?;
    Ok(res)
}

fn fn_start(
    map: &BTreeMap<u32, Cate>,
    q_vec: &Vec<ApiGet>,
) -> Result<Vec<ApiGet>, Box<std::error::Error>> {
    println!(
        "カテゴリを選択

番号を入力
複数の場合、番号スペース番号スペース…と入力
全選択の場合 0"
    );

    println!("-----・・・・・----------・・・・・----------・・・・・-----");

    for (k, v) in map {
        println!("{} - {} ({})", k, v.category, v.count);
    }

    println!("-----・・・・・----------・・・・・----------・・・・・-----");

    let mut category_selection = String::new();

    io::stdin()
        .read_line(&mut category_selection)
        .expect("Failed to read line");

    println!("選択したカテゴリ: {}", category_selection);

    let question_ok = question_extraction(&q_vec, &category_selection)?;
    Ok(question_ok)
}

fn main() {
    //初期設定
    let mut state = State::Start;
    let json_arr = get_json(URL).unwrap();
    let mut q_vec = make_vec(&json_arr);
    let map = totalization(&q_vec);
    let mut question: Vec<ApiGet> = Vec::new();
    let mut index = 0;
    let mut correct = 0;
    let mut right = false;
    let mut right_ans = String::new();

    //初期画面
    println!(
        "
＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/
プログラム練習問題
＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/＿/"
    );

    //問題ループ
    loop {
        match state {
            State::Start => {
                vev_shuffle(&mut q_vec);
                let a = fn_start(&map, &q_vec);
                match a {
                    Ok(n) => {
                        if n.len() > 0 {
                            question = n;
                            state = State::Question;
                        } else {
                            println!("エラー：問題がないです");
                        }
                    }
                    Err(err) => println!("エラー：{}", err),
                };
            }
            State::Question => {
                println!("************************************************************");

                let q = question.get(index);
                index += 1;

                if let Some(x) = q {
                    right_ans = x.q_answer.clone();
                    println!("{} の問題", x.c_name);
                    println!();
                    println!("{}", x.q_question);
                    println!();
                    println!("↓答えを入力");
                    let mut answer = String::new();
                    io::stdin()
                        .read_line(&mut answer)
                        .expect("Failed to read line");

                    if answer.trim() == right_ans {
                        right = true;
                        correct += 1;
                    } else {
                        right = false;
                    }
                    state = State::Check;
                }

            }
            State::Check => {
                if right {
                    println!(">>> 正解です！");
                    println!();
                } else {
                    println!(">>> 間違いです！");
                    println!();
                    println!("正解は");
                    println!();
                    println!("{:?}", right_ans);
                    println!();
                }

                println!(
                    "{}問中{}問正解／正解率{:.2}",
                    index,
                    correct,
                    correct as f64 / index as f64
                );
                println!();

                println!("次の問題：Enter");
                println!("始めから行う：start");
                println!("終了する：exit");
                let mut next = String::new();
                io::stdin()
                    .read_line(&mut next)
                    .expect("Failed to read line");

                if next.trim() == "start" {
                    index = 0;
                    correct = 0;
                    state = State::Start;
                } else if next.trim() == "exit" {
                    break;
                } else {
                    state = State::Question;
                }
            }
        };
    }
    println!("終了しました");
}

