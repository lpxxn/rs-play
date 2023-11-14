use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileState::Open => write!(f, "Open"),
            FileState::Closed => write!(f, "Closed"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} {}>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_file() {
        let f1 = File::new("f1.txt");
        let f2 = File::new("f2.txt");
        let f3 = File::new("f3.txt");
        println!("f1: {:?}", f1);
        println!("f2: {}", f2);
        println!("f3: {}", f3);
        assert_eq!(f1.state, FileState::Closed);
        assert_eq!(f2.state, FileState::Closed);
        assert_eq!(f3.state, FileState::Closed);
    }
}