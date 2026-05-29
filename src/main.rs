//　ハッシュマップとベクタを使用した、会社の部署に従業員の名前を追加するテキストインターフェース
use std::collections::HashMap;
use std::io;

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::from([
        (
            "営業部".to_string(),
            vec!["佐藤健".to_string(), "田中美咲".to_string()],
        ),
        (
            "開発部".to_string(),
            vec!["山田悠斗".to_string(), "高橋玲奈".to_string()],
        ),
        (
            "人事部".to_string(),
            vec!["渡辺結衣".to_string(), "中村陽菜".to_string()],
        ),
    ]);

    loop {
        println!("1. 従業員を追加");
        println!("2. 部署ごとに従業員を表示");
        println!("3. 全従業員を表示");
        println!("4. 終了");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "1" => add_employee(&mut departments),
            "2" => display_departments(&departments),
            "3" => display_all_employees(&departments),
            "4" => break,
            _ => println!("無効な選択です。もう一度試してください。"),
        }
    }
}

// 部署に従業員の名前を追加する関数
fn add_employee(departments: &mut HashMap<String, Vec<String>>) {
    let mut department = String::new();
    let mut employee_name = String::new();

    println!("部署名を入力してください:");
    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read line");
    let department = department.trim().to_string();

    println!("従業員の名前を入力してください:");
    io::stdin()
        .read_line(&mut employee_name)
        .expect("Failed to read line");
    let employee_name = employee_name.trim().to_string();

    departments
        .entry(department)
        .or_insert(Vec::new())
        .push(employee_name);
}

// 部署ごとに従業員を表示する関数
fn display_departments(departments: &HashMap<String, Vec<String>>) {
    for (department, employees) in departments {
        println!("部署: {}", department);
        for employee in employees {
            println!(" - {}", employee);
        }
    }
}

//  全従業員を表示する関数
fn display_all_employees(departments: &HashMap<String, Vec<String>>) {
    println!("全従業員:");
    for employees in departments.values() {
        for employee in employees {
            println!(" - {}", employee);
        }
    }
}
