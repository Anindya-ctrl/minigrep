pub mod utilities {
    use std::{ env, fs, error::Error };

    pub struct Config<'a> {
        pub query: &'a str,
        pub filename: &'a str,
        pub case_sensitive: bool,
    }
    impl Config<'_> {
        fn new<'a>(query: &'a str, filename: &'a str, case_sensitive: bool) -> Config<'a> {
            return Config {
                query,
                filename,
                case_sensitive,
            };
        }
    }
    pub fn parse_config(args: &[String]) -> Result<Config, &str> {
        let len = args.len();
        if len < 3 {
            return Err("Not enough arguments.");
        } else {
            return Ok(Config::new(
                &args[1],
                &args[2],
                env::var("CASE_SENTISIVE").is_err()
            ));
        }
    }

    pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
        let mut result = Vec::new();

        for line in content.lines() {
            if line.contains(query) {
                result.push(line);
            }
        }
        return result;
    }

    pub fn case_insensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
        let mut result = Vec::new();

        for line in content.lines() {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                result.push(line);
            }
        }
        return result;
    }

    pub fn execute(config: Config) -> Result<(), Box<dyn Error>> {
        let Config { query, filename, case_sensitive } = config;
        let content = fs::read_to_string(filename)?;

        let result;
        if case_sensitive {
            result = search(&query, &content);
        } else {
            result = case_insensitive_search(&query, &content);
        };
    
        for line in result {
            println!("{}", line);
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::utilities;

    #[test]
    fn case_sensitive() {
        let query = "S";
        let content = "/
Rust:
Fast, Secure, Productive.
Pick three.
        ";

        assert_eq!(vec!["Fast, Secure, Productive."], utilities::search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "S";
        let content = "/
Rust:
Fast, Secure, Productive.
Pick three.
        ";

        assert_eq!(vec!["Rust:", "Fast, Secure, Productive."], utilities::case_insensitive_search(query, content));
    }
}
