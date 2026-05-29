use std::collections::HashMap;
use std::io;

enum Command {
    Add {
        employee: String,
        department: String,
    },
    ListDepartment(String),
    ListAll,
    Help,
    Exit,
}

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    print_help();

    loop {
        println!();
        println!("コマンドを入力してください:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match parse_command(input.trim()) {
            Ok(Command::Add {
                employee,
                department,
            }) => {
                add_employee(&mut departments, department.as_str(), employee.as_str());
                println!("{employee} を {department} に追加しました。");
            }
            Ok(Command::ListDepartment(department)) => {
                println!(
                    "{}",
                    format_department_listing(&departments, department.as_str())
                );
            }
            Ok(Command::ListAll) => {
                println!("{}", format_all_departments_listing(&departments));
            }
            Ok(Command::Help) => print_help(),
            Ok(Command::Exit) => break,
            Err(message) => println!("{message}"),
        }
    }
}

fn parse_command(input: &str) -> Result<Command, String> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err("コマンドが空です。Help と入力して使い方を表示できます。".to_string());
    }

    if trimmed.eq_ignore_ascii_case("help") {
        return Ok(Command::Help);
    }

    if trimmed.eq_ignore_ascii_case("exit") {
        return Ok(Command::Exit);
    }

    if trimmed.eq_ignore_ascii_case("list all") {
        return Ok(Command::ListAll);
    }

    if has_ascii_prefix(trimmed, "list ") {
        let department = trimmed[5..].trim();
        if department.is_empty() {
            return Err(
                "部署一覧は `List Engineering` のように部署名を指定してください。".to_string(),
            );
        }

        return Ok(Command::ListDepartment(department.to_string()));
    }

    if has_ascii_prefix(trimmed, "add ") {
        let rest = trimmed[4..].trim();
        if let Some(separator_index) = find_ascii_case_insensitive(rest, " to ") {
            let employee = rest[..separator_index].trim();
            let department = rest[separator_index + 4..].trim();

            if employee.is_empty() || department.is_empty() {
                return Err(
                    "追加は `Add Sally to Engineering` の形式で入力してください。".to_string(),
                );
            }

            return Ok(Command::Add {
                employee: employee.to_string(),
                department: department.to_string(),
            });
        }

        return Err("追加は `Add Sally to Engineering` の形式で入力してください。".to_string());
    }

    Err("不明なコマンドです。Help と入力して使い方を確認してください。".to_string())
}

fn has_ascii_prefix(input: &str, prefix: &str) -> bool {
    input
        .get(..prefix.len())
        .is_some_and(|part| part.eq_ignore_ascii_case(prefix))
}

fn find_ascii_case_insensitive(haystack: &str, needle: &str) -> Option<usize> {
    haystack.char_indices().find_map(|(start, _)| {
        let end = start + needle.len();
        if haystack.is_char_boundary(end)
            && haystack
                .get(start..end)
                .is_some_and(|part| part.eq_ignore_ascii_case(needle))
        {
            Some(start)
        } else {
            None
        }
    })
}

fn add_employee(departments: &mut HashMap<String, Vec<String>>, department: &str, employee: &str) {
    departments
        .entry(department.to_string())
        .or_default()
        .push(employee.to_string());
}

fn format_department_listing(
    departments: &HashMap<String, Vec<String>>,
    department: &str,
) -> String {
    match departments.get(department) {
        Some(employees) => {
            let mut sorted_employees = employees.clone();
            sorted_employees.sort();

            let employee_lines = sorted_employees
                .into_iter()
                .map(|employee| format!(" - {employee}"))
                .collect::<Vec<_>>()
                .join("\n");

            format!("部署: {department}\n{employee_lines}")
        }
        None => format!("部署 `{department}` はまだ登録されていません。"),
    }
}

fn format_all_departments_listing(departments: &HashMap<String, Vec<String>>) -> String {
    if departments.is_empty() {
        return "登録されている従業員はいません。".to_string();
    }

    let mut sorted_departments = departments.iter().collect::<Vec<_>>();
    sorted_departments.sort_by(|(left, _), (right, _)| left.cmp(right));

    sorted_departments
        .into_iter()
        .map(|(department, employees)| {
            let mut sorted_employees = employees.clone();
            sorted_employees.sort();

            let employee_lines = sorted_employees
                .into_iter()
                .map(|employee| format!(" - {employee}"))
                .collect::<Vec<_>>()
                .join("\n");

            format!("部署: {department}\n{employee_lines}")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn print_help() {
    println!("利用できるコマンド:");
    println!("  Add Sally to Engineering");
    println!("  List Engineering");
    println!("  List All");
    println!("  Help");
    println!("  Exit");
}
