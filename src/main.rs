use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Philosopher {
    name: String,
    left_fork: usize,
    right_fork: usize,
    done: Sender<bool>,
}

impl Philosopher {
    fn new(name: &str, left_fork: usize, right_fork: usize, done: Sender<bool>) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left_fork,
            right_fork,
            done,
        }
    }

    fn done(&self) {
        println!("{} is satisfied", self.name);
        self.done
            .send(true)
            .ok()
            .expect("Unable to send message to channel");
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left_fork]
            .lock()
            .ok()
            .expect("Unable to take left fork");
        println!("        {} has left fork", self.name);
        let _right = table.forks[self.right_fork]
            .lock()
            .ok()
            .expect("Unable to take right fork");
        println!("        {} has right fork", self.name);
        println!("{} is eating", self.name);
        thread::sleep(Duration::from_secs(1));
        println!("{} is done eating", self.name);
    }

    fn think(&self) {
        println!("{} is thinking", self.name);
        thread::sleep(Duration::from_secs(3));
        println!("{} after thinking said: Eureka!", self.name);
        self.done();
    }
}

struct Table {
    forks: Vec<Mutex<bool>>,
}

fn main() {
    let (tx, rx) = channel();
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(true),
            Mutex::new(true),
            Mutex::new(true),
            Mutex::new(true),
            Mutex::new(true),
        ],
    });

    let philosophers = vec![
        Philosopher::new("Baruch Spinoza", 4, 0, tx.clone()),
        Philosopher::new("Gilles Deluze", 0, 1, tx.clone()),
        Philosopher::new("Karl Marks", 1, 2, tx.clone()),
        Philosopher::new("Friedrich Nietzsche", 2, 3, tx.clone()),
        Philosopher::new("Michael Foucault", 3, 4, tx.clone()),
    ];

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let table = table.clone();
            thread::spawn(move || {
                for _ in 0..3 {
                    p.eat(&table);
                    p.think();
                }
            })
        })
        .collect();
    for _ in 0..15 {
        rx.recv().unwrap();
    }

    for handle in handles {
        handle.join().ok().expect("Couldn't join threads");
    }
}
