// import rand module
use rand;

fn get_response() -> String {
    // create a vector of strings
    let responses = vec![
        "It is certain",
        "It is decidedly so",
        "Without a doubt",
        "Yes, definitely",
        "You may rely on it",
        "As I see it, yes",
        "Most likely",
        "Outlook good",
        "Yes",
        "Signs point to yes",
        "Reply hazy try again",
        "Ask again later",
        "Better not tell you now",
        "Cannot predict now",
        "Concentrate and ask again",
        "Don't count on it",
        "My reply is no",
        "My sources say no",
        "Outlook not so good",
        "Very doubtful",
    ];

    // get a random index from the vector
    let index = rand::random::<usize>() % responses.len();

    // return the string at the random index
    responses[index].to_string()
}


fn main() {
    // take command line input as text, save to variable
    println!("Ask a question: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // call the get_response function and print the result
    println!("{}", get_response());
}
