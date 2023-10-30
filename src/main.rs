use std::io;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "PascalCase")]
struct ToDoTask{
    condition:bool,
    objective:String,
    term:String
}

fn main() {
    //to do list
    let mut to_do_list: Vec<ToDoTask> = Vec::new();
    let file = Path::new("task/task.json");
    
    //メインループ
    loop{
        //実行したいコマンドを選択
        println!(">実行したいタスクを選択してください.\n1. 閲覧\n2. 追加\n3. 編集\n4. 終了");
        let command_task_input = standard_input_u32();
        let command_task = match command_task_input {
            1 => "「閲覧」が選択されました．".to_string(),
            2 => "「追加」が選択されました．".to_string(),
            3 => "「編集」が選択されました．".to_string(),
            4 => "「終了」が選択されました．".to_string(),
            _ => "正しい値を入力してください.".to_string()
        };
        println!(">{}", command_task);

        //実行
        if command_task_input == 1 {//閲覧
            // to do listをロードする
            let input_fn = fs::read_to_string(file).expect("JSON Read Failed.");
            to_do_list = serde_json::from_str(&input_fn).unwrap();

            //表示
            display_task(&to_do_list);

        }else if command_task_input == 2 {//追加
            // to do listをロードする
            let input_fn = fs::read_to_string(file).expect("JSON Read Failed.");
            to_do_list = serde_json::from_str(&input_fn).unwrap();

            //表示
            display_task(&to_do_list);

            //タスクの作成
            let todo = make_task();

            //配列へ保存
            let add_index = to_do_list.len(); 
            to_do_list.push(todo);

            //jsonに保存
            output_json(file, &to_do_list);

        }else if command_task_input == 3 {//編集
            // to do listをロードする
            let input_fn = fs::read_to_string(file).expect("JSON Read Failed.");
            to_do_list = serde_json::from_str(&input_fn).unwrap();

            //表示
            display_task(&to_do_list);

            //タスク選択
            println!(">編集したいタスクの番号を入力してください．");
            let command_task_number_u32 = standard_input_u32();
            let task_index = command_task_number_u32 as usize;

            //実行コマンド選択
            println!(">編集内容を選択してください.\n1. 達成状況\n2. 内容\n3. 期限\n4. 削除\n");
            let command_task_number = standard_input_u32();

            if command_task_number == 1 {
                let old_condition = to_do_list[task_index].condition;
                if old_condition == false {
                    to_do_list[task_index].condition = true;
                }else if old_condition ==true {
                    to_do_list[task_index].condition = false;
                }else {
                    break;
                }
            }else if command_task_number == 2 {
                println!(">新しい内容を入力してください．");
                let new_task_objective = standard_input_string();
                to_do_list[task_index].objective = new_task_objective;
                println!(">{}番のタスクの内容を変更しました．", task_index);
            }else if command_task_number == 3 {
                println!(">新しい期限を入力してください．");
                let new_task_term = standard_input_string();
                to_do_list[task_index].term = new_task_term;
                println!(">{}番のタスクの期限を変更しました．", task_index);
            }else if command_task_number == 4 {
                to_do_list.remove(task_index);
                println!(">{}番のタスクを削除しました．", task_index);
            }else {
                break;
            }

            //jsonに保存
            output_json(file, &to_do_list);

        }else if command_task_input == 4 {//終了
            break;
        }        
    }
}

    //入力内容を保存
    fn standard_input_string() -> String {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => buffer.trim().parse().expect("Failed to parse."),
            Err(e) => panic!("Failed to read line: {}", e)
        }
    }
    fn standard_input_u32() -> u32 {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => buffer.trim().parse().expect("Failed to parse."),
            Err(e) => panic!("Failed to read line: {}", e)
        }
    }
    //タスクを作成
    fn make_task() -> ToDoTask{
        println!(">タスク内容を入力してください．");
        let task_objective = standard_input_string();
        println!(">タスクの達成期限を入力してください．");
        let task_term = standard_input_string();
        let task = ToDoTask{
            condition:  false,
            objective:  task_objective,
            term:   task_term
        };
        task
    }
    //タスクを保存(task/task.json)
    fn output_json(output_fn: &Path, to_do_list:&Vec<ToDoTask>) -> std::io::Result<()> {
        // シリアライズ
        let serialized: String = serde_json::to_string(&to_do_list).unwrap();
    
        // ファイル出力
        let mut file = File::create(output_fn)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
    //タスクを表示
    fn display_task(to_do_list:&Vec<ToDoTask>){
        let mut task_num = 0;
        for data in to_do_list {
            let cond = if data.condition {"達成"}else{"未達成"};
            println!("{}========{}========{}", task_num, task_num,  task_num);
            println!("達成状況: {}\n内容: {}\n期限: {}", cond, data.objective, data.term);
            println!("{}========{}========{}", task_num, task_num,  task_num);
            task_num += 1;
        }
    }