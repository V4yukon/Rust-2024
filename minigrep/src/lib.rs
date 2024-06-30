use std::{ env, error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub command: String,
    pub filename: String,
    pub ignorecase: bool,
}

impl Config {
    // fn new(args: &[String]) -> Self {
    //     if args.len() < 3 {
    //         panic!("Parameters isn't enough")
    //     } else {
    //         let command = args[1].clone();
    //         let filename = args[2].clone();
    //         Config {command, filename}
    //     }
    // }

    pub fn build(mut args: impl Iterator<Item = String> ) -> Result<Self, &'static str> {
        // if args.len() < 3 {
        //     return Err("parameters isn't enough");
        // }
        // let command = args[1].clone();
        // let filename = args[2].clone();


        /* use iterator to improve the build */
        args.next();
        let command = match args.next() {
             Some(arg) => arg,
             None => return Err("parameters isn't enough"),
        };

        let filename = match  args.next() {
            Some(arg) => arg,
            None => return Err("parameters isn't enough"),
        };
        let ignorecase = env::var("IGNORE_CASE").is_ok();
        Ok(Config{command, filename, ignorecase})
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        let text = fs::read_to_string(self.filename)?;

        let result = if self.ignorecase {
            search_case_insensitive(&self.command, &text)
        } else {
            search(&self.command, &text)
        };
        if result.is_empty() {
            println!("Oops!There is no same char in the file");
        } else {
            println!("Well Done!It's this: \n");
            for line in result{
                println!("{line}");
            }
        }
        Ok(())

    }
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    /*origin way */
    // let mut result = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query){
    //         result.push(line);
    //     }
    // }
    // result

    /* Iterator */
    contents
        .lines()
        .filter(|line| line.contains(query) )
        .collect()

    //let content: Vec<&str> = contents.split("\n").collect();
    // let mut result = Vec::new();
    // for item in &content {
    //     if *item == query {
    //         println!("{:?}", content.clone());
    //         result.push(*item);
    //     } else {
    //         println!("the content have not this query: {query}");
    //     }
    // }
    // result
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // // let contents = contents.to_lowercase();
    // let mut result = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query){
    //         result.push(line);
    //     }
    // }
    // result
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    
    }
}