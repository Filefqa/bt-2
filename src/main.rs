use std::io::{self, Write};
use std::fs;
use std::cmp::Ordering;
use rand::Rng;

struct Student<'a> {
    name: &'a str,
    school: char,
    height: i32,
}

fn start_game(students: &Vec<Student>) {
    let schools: [(char, &str); 4] = [('a', "Abydos"), ('t', "Trinity"), ('g', "Gehenna"), ('m', "Millenium")];
    let secret_id: usize = rand::thread_rng().gen_range(0..students.len());
    let mut secret_full: &str = "";
    for (short_s, full_s) in schools {
        if students[secret_id].school == short_s {
            secret_full = full_s;
            break;
        }
    } 
    println!("      Guess the ~~{}~~ student", secret_full);
    println!("-----------------------------------------");
    loop {
        match start_turn(students, secret_id) {
            Ok(i) => {if i == 0 {break;}},
            Err(_) => {break;}
        }
    }
}

fn request_guess(guess: &mut String) -> Result<usize, io::Error> {
    print!("Your guess: ");
    io::stdout().flush()?;
    io::stdin().read_line(guess)?;
    Ok(guess.len())
}

fn start_turn(students: &Vec<Student>, secret_id: usize) -> Result<i32, io::Error> {
    let mut guess: String = String::new();
    request_guess(&mut guess)?;
    guess = String::from(guess.trim());
    let mut gstat: i32 = 0;
    let mut guess_id: usize = 0;
    for st in 0..students.len() {
        if students[st].name == guess.to_ascii_lowercase() {
            gstat = if students[secret_id].school == students[st].school {1} else {2};
            guess_id = st;
        }
    }
    if guess_id == secret_id {
        println!("( > w <) That's correct!");
        return Ok(0);
    }
    match gstat {
        0 => {
            println!("( > _ <) That's not a valid student name");
            return Ok(1);
        },
        2 => {
            println!("( ^ - ^) That's a student from a different school");
            return Ok(1);
        },
        _ => {},
    }
    match students[guess_id].height.cmp(&students[secret_id].height) {
        Ordering::Less => {
            println!("( ^ - ^) Too short! ({} cm)", students[guess_id].height);
        },
        Ordering::Greater => {
            println!("( ^ - ^) Too tall! ({} cm)", students[guess_id].height);
        },
        Ordering::Equal => {
            println!("( 0 o 0) Same hight, different name! ({} cm)", students[guess_id].height);
        },
    }
    Ok(1)
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
