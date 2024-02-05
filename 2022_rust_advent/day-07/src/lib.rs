use nom::{IResult, bytes::streaming::tag, character::complete::alpha1, branch::alt};

fn cd(input: &str) -> IResult<&str, &str> {
    let (input, dir) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;
    Ok((input, dir))
}

fn ls(input: &str) {
    let (input, _) = tag("$ ls")(input)?;
    let 
}




pub fn process_part1(input: &str) -> String {
    "".to_string()
}















// use nom::{
//     bytes::complete::{tag, is_a},
//     character::complete::{alpha1, newline},
//     sequence::separated_pair,
//     multi::separated_list1,
//     branch::alt,
//     *
// };



// enum Operation<'a> {
//     Cd(Cd<'a>),
//     Ls(Vec<Files<'a>>)
// }

// enum Cd<'a> {
//     Root,
//     Up,
//     Down(&'a str),
// }

// enum Files<'a> {
//     File {size: u32, name: &'a str},
//     Dir(&'a str),
// }

// fn file(input: &str) -> IResult<&str, Files> {
//     let (input, (size, name)) = separated_pair(
//         nom::character::complete::u32,
//         tag(" "),
//         is_a("qwertyuiopasdfghjklzxcvbnm.")
//     )(input)?;
//     Ok((input, Files::File { size, name }))
// }

// fn directory(input: &str) -> IResult<&str, Files> {
//     let (input, _) = tag("dir ")(input)?;
//     let (input, name) = alpha1(input)?;
//     Ok((input, Files::Dir(name)))
// }

// fn ls(input: &str) -> IResult<&str, Operation> {
//     let (input, _) = tag("$ ls")(input)?;
//     let (input, _) = newline(input)?;
//     let (input, files) = separated_list1(
//         newline, 
//         alt((file, directory)),
//     )(input)?;
//     Ok((input, Operation::Ls(files)))
// }

// fn cd(input: &str) -> IResult<&str, Operation> {
//     let (input, dir) = tag("$ cd ")(input)?;
//     let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;

//     let operation = match dir {
//         "/" => Operation::Cd(Cd::Root),
//         ".." => Operation::Cd(Cd::Up),
//         name => Operation::Cd(Cd::Down(name)),
//     };
//     Ok((input, operation))
// }





// fn command(input: &str) -> IResult<&str, Vec<Operation>> {
//     let (input, cmd) = separated_list1(newline, alt((ls, cd)))(input)?;
//     Ok((input, cmd))
// }

// struct 


// pub fn process_part1(input: &str) -> String {

//     "works".to_string()
// }







// // pub fn process_part2(input: &str) -> String {
// //     "works".to_string()
// // }