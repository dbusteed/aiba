use clipboard::{ClipboardProvider, ClipboardContext};
use dirs;
use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::process;
use term;

#[derive(Debug)]
struct Cred {
    name: String,
    location: String,
    username: String,
    password: String,
}

impl Cred {
    fn new(values: &Vec<&str>) -> Cred {
        Cred {
            name: values[0].to_owned(),
            location: values[1].to_owned(),
            username: values[2].to_owned(),
            password: values[3].to_owned(),
        }
    }
}

fn main() {

    let mut t = term::stdout().unwrap();
    let mut clip: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut creds: BTreeMap<usize, Cred> = BTreeMap::new();

    clear_screen();
    t.reset().unwrap();

    let home_dir = dirs::home_dir().unwrap();
    let aiba_path = home_dir.join(".aiba");

    let mut input = String::from("");
    let input_parts: Vec<&str>;
    let cmd: char;
    let mut idx: usize;

    if !aiba_path.exists() {
        t.fg(term::color::YELLOW).unwrap();

        writeln!(t, "  {} not found, creating now...", aiba_path.display()).unwrap();
        let file = File::create(&aiba_path).unwrap();
        
        writeln!(&file, "name,location,username,password").unwrap();

        writeln!(t, "  Edit the contents of this file to add username/passwords").unwrap();
        quit();
    }

    let aiba_file = fs::read_to_string(&aiba_path)
        .expect("Error reading file");

    for (i, line) in aiba_file.lines().enumerate() {
        creds.insert(i, Cred::new(&line.split(",").collect()));
    }

    t.fg(term::color::CYAN).unwrap();

    writeln!(t, "  passwords").unwrap();
    writeln!(t, "  ---------\n").unwrap();

    for (idx, cred) in creds.iter() {
        writeln!(t, "  {}. {}", idx+1, cred.name).unwrap();
    }

    t.reset().unwrap();

    println!("\n  Enter command, get (h)elp, or (q)uit");
    print!("  >>>  ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input_parts = input.split(" ").collect();

    if input_parts.len() == 1 {
        if input_parts[0].trim() == "h" {
            writeln!(t, "\n  Copy passwords and other information to your clipboard").unwrap();
            writeln!(t, "  by entering a \"command\" followed by an item number").unwrap();
            writeln!(t, "\n  For example, to copy the username for the 4th entry,").unwrap();
            writeln!(t, "  you would enter \"u 4\" (notice the space!). The default").unwrap();
            writeln!(t, "  command is \"p\" (password), so to copy the password for").unwrap();
            writeln!(t, "  the 4th entry, you would just enter \"4\"").unwrap();
            writeln!(t, "\n  Available commands:").unwrap();
            writeln!(t, "    l -- copy location").unwrap();
            writeln!(t, "    u -- copy username").unwrap();
            writeln!(t, "    p -- copy password (this is default, and can be left out)").unwrap();
            writeln!(t, "    q -- quit").unwrap();
            writeln!(t, "\n  The credential file is located here:").unwrap();
            writeln!(t, "    {}", aiba_path.display()).unwrap();
            writeln!(t, "\n  It's formatted like a CSV file, with the following layout:").unwrap();
            writeln!(t, "    name,location,username,password").unwrap();
            writeln!(t, "\n  An example would look like this:").unwrap();
            writeln!(t, "    GitHub,https://github.com/login,bob,bobs_secure_password").unwrap();
            quit();
        } else if input_parts[0].trim() == "q" {
            quit();
        }
        cmd = 'p';
        idx = input_parts[0].trim().parse::<usize>().unwrap();
    } else if input_parts.len() == 2 {
        cmd = input_parts[0].chars().next().unwrap();
        idx = input_parts[1].trim().parse::<usize>().unwrap();
    } else {
        eprintln!("Invalid input");
        process::exit(1);
    }

    idx -= 1;

    println!();

    match cmd {
        'l' => {
            if creds[&idx].location == "" {
                writeln!(t, "  {} location is empty, nothing copied to clipboard", creds[&idx].name).unwrap();
            } else {
                clip.set_contents(creds[&idx].location.to_owned()).unwrap();
                writeln!(t, "  {} location copied to clipboard!", creds[&idx].name).unwrap();
            }
        },
        'u' => {
            if creds[&idx].username == "" {
                writeln!(t, "  {} username is empty, nothing copied to clipboard", creds[&idx].name).unwrap();
            } else {
                clip.set_contents(creds[&idx].username.to_owned()).unwrap();
                writeln!(t, "  {} username copied to clipboard!", creds[&idx].name).unwrap();
            }
        },
        'p' => {
            if creds[&idx].password == "" {
                writeln!(t, "  {} password is empty, nothing copied to clipboard", creds[&idx].name).unwrap();
            } else {
                clip.set_contents(creds[&idx].password.to_owned()).unwrap();
                writeln!(t, "  {} password copied to clipboard!", creds[&idx].name).unwrap();
            }
        },
        _ => writeln!(t, "  Command not recognized, checkout the help option.").unwrap()
    }

    t.reset().unwrap();

}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc=27 as char);
}

fn quit() {
    process::exit(0);
}
