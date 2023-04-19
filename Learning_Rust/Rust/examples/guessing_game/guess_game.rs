use std::io;



fn recursive_guess(word: String){
    let mut buffy = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffy);
    &mut buffy.truncate(buffy.len() - 2);
    if(word == buffy){
        return;
    }
    println!("Guess {:?}",buffy);
    return recursive_guess(word);
}

fn main() {
    println!("Guessing game");

    println!("Your input");

   

 
    recursive_guess("World".to_owned());
    println!("Correct U won");
    

}
