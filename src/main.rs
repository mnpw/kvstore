use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let operation = args.next().unwrap();

    let mut db = Database::new().unwrap();
    if operation == "GET" {
        let key = args.next().unwrap();
        let value: &String = db.get(key);

        println!("Value is {}", value);
    } else if operation == "SET" {
        let key = args.next().unwrap();
        let value = args.next().unwrap();

        db.add(key, value);
        db.flush();
    }
}

struct Database {
    inner: std::collections::HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let contents = std::fs::read_to_string("kv.db")?;
        let mut inner = std::collections::HashMap::new();

        for line in contents.lines() {
            let chunks: Vec<&str> = line.split('\t').collect();
            if chunks.len() != 2 {
                continue;
            }
            let key = chunks[0];
            let value = chunks[1];
            inner.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { inner: inner })
    }

    fn flush(&self) {
        let mut contents = String::from("");
        for line in self.inner.iter() {
            dbg!(line);
            contents = contents + &format!("{}\t{}\n", &line.0, &line.1);
        }
        std::fs::write("kv.db", contents);
    }

    fn add(&mut self, key: String, value: String) {
        self.inner.insert(key, value);
    }

    fn get(&self, key: String) -> &String {
        return self.inner.get(&key).unwrap();
    }
}
