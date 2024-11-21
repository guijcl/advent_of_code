use std::{cell::RefCell, fs::read_to_string, io, rc::Rc, str::FromStr};

struct OperationError;
struct ContentError;
struct FileError;
struct DirError;
struct StateError;

#[derive(Clone)]
struct Dir {
    name: String,
    contents: Rc<RefCell<Vec<Content>>>,
    size: i32,
}

impl std::fmt::Debug for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dir")
            .field("name", &self.name)
            .field(
                "contents",
                &self
                    .contents
                    .borrow()
                    .iter()
                    .map(|content| format!("{:?}", content))
                    .collect::<Vec<_>>(),
            )
            .field("size", &self.size)
            .finish()
    }
}

impl FromStr for Dir {
    type Err = DirError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let (Some(f_type), Some(f_name)) = (parts.next(), parts.next()) else {
            return Err(DirError);
        };
        if f_type == "dir" {
            Ok(Dir {
                name: String::from(f_name),
                contents: Rc::new(RefCell::new(Vec::new())),
                size: 0,
            })
        } else {
            Err(DirError)
        }
    }
}

impl Dir {
    fn calculate_dir_size(&mut self) -> i32 {
        let size: i32 = self
            .contents
            .as_ref()
            .borrow_mut()
            .iter_mut()
            .fold(0, |acc, c| {
                acc + match c {
                    Content::File(f) => f.size,
                    Content::Dir(d) => d.calculate_dir_size(),
                }
            });
        self.size = size;
        size
    }
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: i32,
}

impl FromStr for File {
    type Err = FileError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let (Some(f_type), Some(f_name)) = (parts.next(), parts.next()) else {
            return Err(FileError);
        };
        Ok(File {
            name: String::from(f_name),
            size: f_type.parse().map_err(|_| FileError)?,
        })
    }
}

#[derive(Debug, Clone)]
enum Content {
    Dir(Dir),
    File(File),
}

impl FromStr for Content {
    type Err = ContentError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(dir) = s.parse::<Dir>() {
            Ok(Content::Dir(dir))
        } else if let Ok(file) = s.parse::<File>() {
            Ok(Content::File(file))
        } else {
            Err(ContentError)
        }
    }
}

#[derive(Clone)]
enum Operations {
    CD(String),
    LS,
}

impl FromStr for Operations {
    type Err = OperationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        if parts.len() == 3 {
            Ok(Operations::CD(String::from(parts[2])))
        } else if parts.len() == 2 {
            Ok(Operations::LS)
        } else {
            Err(OperationError)
        }
    }
}

#[derive(Clone)]
struct State {
    root: Rc<RefCell<Dir>>,
    current_path: Vec<Rc<RefCell<Dir>>>,
}

impl State {
    fn calculate_total_size(&self) -> i32 {
        self.root.as_ref().borrow_mut().calculate_dir_size()
    }

    fn path_string(&self) -> String {
        let path: Vec<String> = self
            .current_path
            .iter()
            .skip(1) // Skip root dir since we'll add the initial / anyway
            .map(|d| d.borrow().name.clone())
            .collect();

        if path.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", path.join("/"))
        }
    }

    fn add_contents(&mut self, contents: Vec<Content>) -> Result<(), StateError> {
        if let Some(current_dir) = self.current_path.last() {
            // Get current contents
            let current_dir_ref = current_dir.borrow_mut();
            let mut current_contents = current_dir_ref.contents.borrow_mut();

            // For each new content item
            for content in contents {
                // Check if we already have this item
                let exists = current_contents
                    .iter()
                    .any(|existing| match (existing, &content) {
                        (Content::File(f1), Content::File(f2)) => f1.name == f2.name,
                        (Content::Dir(d1), Content::Dir(d2)) => d1.name == d2.name,
                        _ => false,
                    });

                // Only add if it doesn't exist
                if !exists {
                    current_contents.push(content);
                }
            }

            Ok(())
        } else {
            Err(StateError)
        }
    }

    fn ls(&mut self, lines: &mut std::str::Lines) -> Result<(), StateError> {
        println!("\n=== LS ===");
        println!("Current path before ls: {}", self.path_string());

        let mut new_contents = Vec::new();
        let current_dir = self.current_path.last().ok_or(StateError)?.clone();
        {
            let current_borrow = current_dir.borrow();
            let current_contents = current_borrow.contents.borrow();
            new_contents.extend(current_contents.iter().cloned());
        }

        while let Some(line) = lines.next() {
            if line.starts_with('$') {
                break;
            }
            println!("Parsing line: {}", line);
            match line.parse::<Content>() {
                Ok(content) => {
                    println!("Parsed content: {:?}", content);
                    let exists = new_contents
                        .iter()
                        .any(|existing| match (existing, &content) {
                            (Content::File(f1), Content::File(f2)) => f1.name == f2.name,
                            (Content::Dir(d1), Content::Dir(d2)) => d1.name == d2.name,
                            _ => false,
                        });

                    if !exists {
                        if let Content::Dir(dir) = content {
                            // Create the directory with its own RefCell
                            let new_dir = Rc::new(RefCell::new(Dir {
                                name: dir.name,
                                contents: Rc::new(RefCell::new(Vec::new())),
                                size: 0,
                            }));

                            // Store the reference to it
                            new_contents.push(Content::Dir(Dir {
                                name: new_dir.borrow().name.clone(),
                                contents: new_dir.borrow().contents.clone(),
                                size: 0,
                            }));
                        } else {
                            new_contents.push(content);
                        }
                    }
                }
                Err(_) => {
                    println!("Failed to parse line: {}", line);
                    return Err(StateError);
                }
            }
        }

        // Update the directory's contents with merged list
        let mut current_dir_ref = current_dir.borrow_mut();
        *current_dir_ref.contents.borrow_mut() = new_contents.clone();

        println!("Contents of current directory after ls: {:?}", new_contents);
        Ok(())
    }

    fn cd(&mut self, dir: &str) -> Result<(), StateError> {
        println!("\n=== CD {} ===", dir);
        println!("Current path before cd: {}", self.path_string());

        match dir {
            ".." => {
                if self.current_path.len() > 1 {
                    self.current_path.pop();
                }
                println!(
                    "Moved up one level. Current path after cd: {}",
                    self.path_string()
                );
                Ok(())
            }
            "/" => {
                self.current_path = vec![self.root.clone()];
                println!("Moved to root directory. Current path after cd: /");
                Ok(())
            }
            target_dir => {
                let current = self.current_path.last().ok_or(StateError)?.clone();
                let current_borrow = current.borrow();
                let contents = current_borrow.contents.borrow();

                // Look in current directory first
                let target = contents.iter().find_map(|content| {
                    if let Content::Dir(d) = content {
                        if d.name == target_dir {
                            Some(Content::Dir(d.clone()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                });

                drop(contents);
                drop(current_borrow);

                match target {
                    Some(Content::Dir(dir)) => {
                        // Navigate to the directory
                        let dir_ref = Rc::new(RefCell::new(dir));
                        self.current_path.push(dir_ref);
                        println!(
                            "Moved into directory '{}'. Current path after cd: {}",
                            target_dir,
                            self.path_string()
                        );
                        Ok(())
                    }
                    _ => {
                        // Try root if not at root
                        if self.current_path.len() > 1 {
                            let root_borrow = self.root.borrow();
                            let root_contents = root_borrow.contents.borrow();

                            if let Some(Content::Dir(dir)) =
                                root_contents.iter().find_map(|content| {
                                    if let Content::Dir(d) = content {
                                        if d.name == target_dir {
                                            Some(Content::Dir(d.clone()))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                })
                            {
                                drop(root_contents);
                                drop(root_borrow);

                                self.current_path = vec![self.root.clone()];
                                let dir_ref = Rc::new(RefCell::new(dir));
                                self.current_path.push(dir_ref);
                                println!(
                                    "Moved into directory '{}' from root. Current path after cd: {}",
                                    target_dir,
                                    self.path_string()
                                );
                                Ok(())
                            } else {
                                println!(
                                    "Error: Directory '{}' not found in current directory or root.",
                                    target_dir
                                );
                                Err(StateError)
                            }
                        } else {
                            println!(
                                "Error: Directory '{}' not found in current directory.",
                                target_dir
                            );
                            Err(StateError)
                        }
                    }
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let root = Rc::new(RefCell::new(Dir {
        name: String::from("/"),
        contents: Rc::new(RefCell::new(Vec::new())),
        size: 0,
    }));
    let mut state: State = State {
        root: root.clone(),
        current_path: vec![root],
    };

    let contents = read_to_string("input.txt")?;
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        if line.starts_with('$') {
            let op = line
                .parse::<Operations>()
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "Invalid operation"))?;

            match op {
                Operations::CD(s) => {
                    state
                        .cd(&s)
                        .map_err(|_| io::Error::new(io::ErrorKind::Other, "CD operation failed"))?;
                }
                Operations::LS => {
                    state
                        .ls(&mut lines)
                        .map_err(|_| io::Error::new(io::ErrorKind::Other, "LS operation failed"))?;
                }
            }
        }
    }

    println!("{}", state.calculate_total_size());

    Ok(())
}
