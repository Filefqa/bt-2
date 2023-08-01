use std::io::{self, Write};
use std::fs;
use rand::Rng;

struct Student<'a> {
    name: &'a str,
    school: char,
    height: i32,
}

fn start_game(students: &Vec<Student>) {
    let answer: usize = rand::thread_rng().gen_range(0..students.len());
    let answer: &Student = &students[answer];
    let answer_school: &str = match answer.school {
        'a' => "Abydos",
        't' => "Trinity",
        'g' => "Gehenna",
        'm' => "Millenium",
        _ => "Undefined",
    };
    println!("      Guess the ~~{}~~ student", answer_school);
    println!("-----------------------------------------");
    loop {
        if let Ok(true) = next_turn(students, answer)  {
            break;
        }
    }
}

fn next_turn(students: &Vec<Student>, answer: &Student) -> Result<bool, io::Error> {
    let mut guess: String = String::new();
    print!("Your guess: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut guess)?;
    guess = String::from(guess.trim());
    for student in students {
        if student.name == guess.to_ascii_lowercase() {
            if student.name == answer.name {
                println!("( > w <) That's correct!");
                return Ok(true);
            } else if student.school != answer.school {
                println!("( ^ - ^) That's a student from a different school");
            }  else if student.height < answer.height {
                println!("( ^ - ^) Too short! ({} cm)", student.height);
            } else if student.height > answer.height {
                println!("( ^ - ^) Too tall! ({} cm)", student.height);
            } else {
                println!("( 0 o 0) Same height, different name! ({} cm)", student.height);
            }
            return Ok(false);
        }
    }
    println!("( > _ <) That's not a valid student name");
    return Ok(false);
}

fn main() {
    let database: String = fs::read_to_string("students.dat").expect("Couldn't access database");
    let database: Vec<&str> = database.split('\n').collect();
    let mut students: Vec<Student> = Vec::new();
    for w in 0..database.len() {
        let student_data: Vec<&str> = database[w].trim().split(' ').collect();
        students.push(Student {
            name: student_data[0],
            school: student_data[1].chars().nth(0).expect("Couldn't read database"),
            height: student_data[2].parse().expect("Couldn't read database"),
        });
    }
    println!("------------------------------------------");
    println!("   Guess the Blue Archive girl! [v0.2.0]");
    println!("         Made by Filefqa ( O w O)");
    println!("-----------------------------------------");
    start_game(&students);
}
