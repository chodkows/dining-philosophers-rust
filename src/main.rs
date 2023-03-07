use std::sync::{Arc, Barrier, Mutex};
use std::{thread, time::Duration};

const HUNGER: i32 = 3;
const EAT_TIME: Duration = Duration::from_secs(1);
const THINK_TIME: Duration = Duration::from_secs(3);

struct Philosopher {
    name: String,
    left_fork: Arc<Fork>,
    right_fork: Arc<Fork>,
}

impl Philosopher {
    fn new(name: String, left_fork: Arc<Fork>, right_fork: Arc<Fork>) -> Philosopher {
        Philosopher {
            name,
            left_fork,
            right_fork,
        }
    }
}

struct Fork {
    mutex: Mutex<i32>,
    number: i32,
}

impl Fork {
    fn new(mutex: Mutex<i32>, number: i32) -> Fork {
        Fork { mutex, number }
    }
}

fn main() {
    let first = Arc::new(Fork::new(Mutex::new(0), 0));
    let second = Arc::new(Fork::new(Mutex::new(1), 1));
    let third = Arc::new(Fork::new(Mutex::new(2), 2));
    let fourth = Arc::new(Fork::new(Mutex::new(3), 3));
    let fifth = Arc::new(Fork::new(Mutex::new(4), 4));
    let philosophers: Vec<Philosopher> = vec![
        Philosopher::new(
            "Socrates".to_string(),
            Arc::clone(&fifth),
            Arc::clone(&first),
        ),
        Philosopher::new("Plato".to_string(), Arc::clone(&first), Arc::clone(&second)),
        Philosopher::new(
            "Aristotle".to_string(),
            Arc::clone(&second),
            Arc::clone(&third),
        ),
        Philosopher::new(
            "Thales".to_string(),
            Arc::clone(&third),
            Arc::clone(&fourth),
        ),
        Philosopher::new(
            "Pythagoras".to_string(),
            Arc::clone(&fourth),
            Arc::clone(&fifth),
        ),
    ];

    println!("The dinning philosophers problem");
    println!("Table is empty");

    dine(philosophers);

    println!("Table is empty");
}

fn dine(philosophers: Vec<Philosopher>) {
    let wg = Arc::new(Barrier::new(philosophers.len()));
    let mut wg_handles = Vec::with_capacity(philosophers.len());

    let seated = Arc::new(Barrier::new(philosophers.len()));
    //   let mut seated_handles = Vec::with_capacity(philosophers.len());

    for philosopher in philosophers {
        let w = Arc::clone(&wg);
        let s = Arc::clone(&seated);

        wg_handles.push(thread::spawn(move || {
            dinning_philosophers(&philosopher, w, s);
        }));
    }

    for handle in wg_handles {
        handle.join().unwrap();
    }
}

fn dinning_philosophers(philosopher: &Philosopher, wg: Arc<Barrier>, seated: Arc<Barrier>) {
    println!("{} is sited at the table", philosopher.name);
    seated.wait();

    for _ in 0..HUNGER {
        if philosopher.left_fork.number > philosopher.right_fork.number {
            if let Ok(_) = philosopher.right_fork.mutex.lock() {
                println!("        {} has right fork", philosopher.name);
                if let Ok(_) = philosopher.left_fork.mutex.lock() {
                    println!("        {} has left fork", philosopher.name);
                    println!("    {} has both forks and is eating", philosopher.name);
                    thread::sleep(EAT_TIME);

                    println!("    {} is thinking", philosopher.name);
                    thread::sleep(THINK_TIME);
                }
            }
        } else {
            if let Ok(_) = philosopher.left_fork.mutex.lock() {
                println!("        {} has left fork", philosopher.name);
                if let Ok(_) = philosopher.right_fork.mutex.lock() {
                    println!("        {} has right fork", philosopher.name);
                    println!("    {} has both forks and is eating", philosopher.name);
                    thread::sleep(EAT_TIME);

                    println!("    {} is thinking", philosopher.name);
                    thread::sleep(THINK_TIME);
                }
            }
        }
        println!("    {} put down the forks", philosopher.name);
    }
    println!("{} is satisfied", philosopher.name);
    println!("{} left the table", philosopher.name);

    wg.wait();
}
