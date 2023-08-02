use std::io::{self, Write};
use std::fs;
use rand::Rng;

struct Student<'a> {
    name: &'a str,
    school: char,
    height: i32,
}

fn centered(msg: &str, end: bool) {
    let sps: usize = if 48 - msg.len() > 0 {(48 - msg.len()) / 2} else {0};
    println!("{}{}", vec![" "; sps].concat(), msg);
    if end {println!("{}", ["-"; 48].concat());}
}

fn start_game(students: &Vec<Student>) {
    let answer: usize = rand::thread_rng().gen_range(0..students.len());
    let answer: &Student = &students[answer];
    let answer_school: &str = match answer.school {
        'a' => "Abydos",
        't' => "Trinity",
        'g' => "Gehenna",
        'm' => "Millenium",
        'v' => "Valkyrie",
        'h' => "Hyakkiyako",
        'u' => "Arius",
        'r' => "Red Winter",
        'j' => "Shanhaijing",
        's' => "SRT",
        _ => "Undefined",
    };
    centered(&["Guess the ~", answer_school, "~ student"].concat()[..], true);
    loop {
        match next_turn(students, answer) {
            Ok(true) => {break;},
            Err(_) => {println!("> [ ( > _ <) ] I can't read this...");},
            Ok(false) => {},
        }
    }
}

fn next_turn(students: &Vec<Student>, answer: &Student) -> Result<bool, io::Error> {
    let mut guess: String = String::new();
    print!("> [Your guess] ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut guess)?;
    guess = String::from(guess.trim());
    for student in students {
        if student.name == guess.to_ascii_lowercase() {
            if student.name == answer.name {
                println!("> [ ( > w <) ] That's correct!");
                return Ok(true);
            } else if student.school != answer.school {
                println!("> [ ( ^ - ^) ] She's from a different school");
            }  else if student.height < answer.height {
                println!("> [ ( ^ - ^) ] Too short! ({} cm)", student.height);
            } else if student.height > answer.height {
                println!("> [ ( ^ - ^) ] Too tall! ({} cm)", student.height);
            } else {
                println!("> [ ( O - O) ] Same height, but different name! ({} cm)", student.height);
            }
            return Ok(false);
        }
    }
    println!("> [ ( > _ <) ] That's not a valid student name");
    return Ok(false);
}

fn main() {
    let database: String = fs::read_to_string("students.dat").expect("Couldn't access database");
    let database: Vec<&str> = database.split('\n').collect();
    let mut students: Vec<Student> = Vec::new();
    for entry in database {
        let data: Vec<&str> = entry.trim().split(' ').collect();
        students.push(Student {
            name: data[0],
            school: data[1].chars().nth(0).expect("Couldn't read database"),
            height: data[2].parse().expect("Couldn't read database"),
        });
    }
    println!("{}", ["-"; 48].concat());
    centered("BT-2 : Guess the Blue Archive Girl", false);
    centered("Made by Filefqa ( O w O)", true);
    start_game(&students);
}
