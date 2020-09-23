// pub fn abbreviate(phrase: &str) -> String {
//     // phrase.chars().into_iter().filter(|x| x.is_uppercase()).collect::<String>()
//     // let count_uppercase = phrase.chars().into_iter().fold(0, |mut acc, x| {
//     //     if x.is_uppercase() {
//     //         acc += 1
//     //     }
//     //     acc
//     // });
//     // let count_words = phrase.split(' ').collect::<Vec<&str>>();
//     let check_alphabetic_only = phrase.chars().into_iter().all(|x| x.is_alphabetic());
//
//     let punctuations = phrase.chars().into_iter().filter(|x| x.is_ascii_punctuation()).collect::<String>();
//
//     match check_alphabetic_only {
//         true => {
//             phrase.chars().into_iter().filter(|x| x.is_uppercase()).collect::<String>()
//         },
//         false => {
//
//         }
//     }
// }

fn main() {
    let phrase = "PHP: Hypertext Preprocessor";
    // let punctuations = phrase.chars().into_iter().filter(|x| x.is).collect::<String>();
    // println!("{}", punctuations);
    let test = "HyperText Markup Language";
    println!("{:?}",test.split(|ch: char| !ch.is_alphabetic()).filter(|&x| !x.is_empty()).collect::<Vec<&str>>());
    let t = test.split(|ch: char| !ch.is_alphabetic()).filter(|&x| !x.is_empty()).filter(|&x| x.len() > 1 || x.chars().next().unwrap().is_uppercase()).collect::<Vec<&str>>();
    let res = t.iter().fold(String::new(), |mut acc, &x| {
        if x.chars().all(|x| x.is_uppercase()) {
            acc.push(x.chars().next().unwrap());
        }
        else if (x.chars().filter(|ch| ch.is_uppercase()).collect::<String>()).len() > 1 {

            acc.push_str(x.chars().filter(|x| x.is_uppercase()).collect::<String>().as_str())
        }
        else {
            acc.push(x.chars().next().unwrap().to_ascii_uppercase());
        }
        acc
    });
    println!("{:?}", t);

    println!("{:?}", res);
}
