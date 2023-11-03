use std::io;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use serde::{Serialize, Deserialize};
use colored::Colorize;

#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "PascalCase")]
struct ToDoTask{
    condition:bool,
    objective:String,
    term:String
}

fn main() {
    run();
}

fn run(){
    //to do list
    let mut to_do_list: Vec<ToDoTask> = Vec::new();
    let file = Path::new("task/task.json");
    
    //メインループ
    loop{
        //実行したいコマンドを選択
        println!("\n{}",">実行したいタスクを選択してください.".truecolor(255, 165, 0));
        println!("{} 閲覧  {} 追加   {} 編集   {} 終了\n","[1]".blue(),"[2]".blue(),"[3]".blue(), "[4]".blue());
        
        let command_task_input = standard_input_u32();
        let command_task = match command_task_input {
            1 => "「閲覧」が選択されました．".to_string(),
            2 => "「追加」が選択されました．".to_string(),
            3 => "「編集」が選択されました．".to_string(),
            4 => "「終了」が選択されました．".to_string(),
            _ => "正しい値を入力してください.".to_string()
        };
        let command_task2 = ">".to_string();
        let command_task3 = command_task2 + &command_task;
        println!("{}", command_task3.purple());

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
            println!("{}",">編集したいタスクの番号を入力してください．".purple());
            let command_task_number_u32 = standard_input_u32();
            let task_index = command_task_number_u32 as usize;

            //実行コマンド選択
            println!("\n{}",">編集内容を選択してください.".truecolor(255, 165, 0));
            println!("{} 達成状況  {} 内容   {} 期限   {} 削除\n","[1]".blue(),"[2]".blue(),"[3]".blue(), "[4]".blue());
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
                println!("{}",">新しい内容を入力してください．".purple());
                let new_task_objective = standard_input_string();
                to_do_list[task_index].objective = new_task_objective;
                println!("{}{}{}",">".purple(), task_index.to_string().purple(), "番のタスクの内容を変更しました．".purple());
            }else if command_task_number == 3 {
                println!("{}",">新しい期限を入力してください．".purple());
                let new_task_term = standard_input_string();
                to_do_list[task_index].term = new_task_term;
                println!("{}{}{}",">".purple(), task_index.to_string().purple(), "番のタスクの期限を変更しました．".purple());
            }else if command_task_number == 4 {
                to_do_list.remove(task_index);
                println!("{}{}{}",">".purple(), task_index.to_string().purple(), "番のタスクの内容を削除しました．".purple());
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
        println!("{}",">タスク内容を入力してください．".purple());
        let task_objective = standard_input_string();
        println!("{}",">タスクの達成期限を入力してください．".purple());
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
        println!("{}{  }{}{}","No.".blue(), "  状況  ".red(), " 日付 ".bright_yellow(), "   内容".bright_white());
        println!("{}", "--- ------ ------  ----------------------------------".truecolor(0, 255, 8));
        for data in to_do_list {
            let cond = if data.condition {"達成　".red()}else{"未達成".dimmed()};
            print!("{}:  ", task_num.to_string().blue());
            print!("{}  ", cond);
            print!("{}  ", data.term.bright_yellow());
            println!("{}",data.objective);
            task_num += 1;
        }
        println!(" ");
    }