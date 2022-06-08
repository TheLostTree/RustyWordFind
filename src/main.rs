
use std::{self, fs, collections::HashMap, io::{Error, self}};
use itertools::Itertools;

fn serialize_dictionary(dictionary: &HashMap<String, Vec<String>>)-> String{

    let mut output: String = "".into();

    for key in dictionary.keys().sorted()  {
        output = output + key+ "=>" + &dictionary[key].join(",") + "\n";
    }
    return output;
}

fn deserialize_dictionary(input: String)->Result<HashMap<String, Vec<String>>,Error>{

    let mut dict: HashMap<String, Vec<String>> = HashMap::new();

    for ele in input.split("\n") {
        
        let mut line = ele.split("=>").collect_vec();
        if line.len() < 2 {
            continue;
        }

        let index = line.remove(0).to_string();

        let mut elements: Vec<String> = Vec::new();

        for string in line{
            for word in string.split(","){
                elements.push(word.to_string());
            }
        }

        dict.insert(index, elements);
    }

    if false {
        return Err(Error::new(std::io::ErrorKind::InvalidInput, "Failed to deserialize prexisting file into a dictionary"))
    }
    return Ok(dict);
}

fn reduce_string(string: &str)-> String{

    let mut temp: Vec<char> = Vec::new();

    for c in string.chars(){
        if c.is_whitespace() {
            continue
        };
        if !temp.contains(&c) {
            temp.push(c)
        }
    }
    temp.sort();
    return temp.into_iter().collect();

}


fn generate_processed()->HashMap<String, Vec<String>>{
    let file = fs::read_to_string("src/wordlist.txt").expect("Could not read file");


    let mut dictionary: HashMap<String,Vec<String>> = HashMap::new();



    for line in file.lines() {

        let output = reduce_string(line);

        let vec = dictionary.get_mut(&output);

        match vec {
            None=>{
                dictionary.insert(output, vec![line.to_string()]);
            },
            Some(v)=>{
                v.push(line.to_string());
            }
        }
    }

    fs::write("src/words.txt", serialize_dictionary(&dictionary)).expect("uhoh");

    return dictionary;
}


fn find_subsequence(input: String)->Vec<String>{

    let n = input.len();
    let mut answers:Vec<String> = Vec::new();


    for num in 0..(1<<n){
        let mut string:String = String::new();
        for i in 0..n{
            if num & (1 << i) != 0{
                let a = input.chars().nth(i).unwrap();
                string = string + &a.to_string();
            }
        }

        if string.len() > 0 {
            answers.push(string);
        }
    }
    answers.sort();

    return answers;
}



fn find_all(input: &String, dictionary: &HashMap<String, Vec<String>>)-> Vec<String>{
    let mut output:Vec<String> = Vec::new();



    let reduced_string = reduce_string(&input);

    

    let subsquences = find_subsequence(reduced_string);


    for string in subsquences {
        if string.len() < 3{ continue; } //the nyt doesnt want tiny words in their puzzle

        if let Some(value) = dictionary.get(&string){
            output.append(&mut value.to_owned());
        }

    }



    return output;
}


fn main(){
    let processed_file = fs::read_to_string("src/words.txt");

    let dictionary: HashMap<String, Vec<String>>;

    match processed_file {
        Ok(_) => {
            dictionary = deserialize_dictionary(processed_file.unwrap()).expect("ur shitty lmfao");
            println!("Loaded preexisting list! {} combinations", dictionary.len())
        },
        Err(_) => {
            //generate a proceedFile
            dictionary = generate_processed();
            println!("Generated new list! {} combinations", dictionary.len())
        },
    }

    let mut input = String::new();

    loop{
        println!("Input letters: ");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {

                input = input.trim().to_string();

                if input == "STOP" {
                    println!("Closing!");
                    break;
                }

                let matches = find_all(&input, &dictionary);

                if matches.len() == 0 {
                    println!("No matches found for {}", input);
                }else{
                    println!("{} matches found!", matches.len());
                    println!("{}", matches.join(", "))
                }

                input = String::new();
            }
            Err(error) => {println!("error: {}", error); break;},
        }
    }
}

