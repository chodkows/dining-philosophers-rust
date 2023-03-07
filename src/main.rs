use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Barrier, Mutex,
    },
    thread,
    time::Duration,
};

const EAT_TIME: Duration = Duration::from_secs(1);
const THINK_TIME: Duration = Duration::from_secs(3);
const EATING_COUNTER: usize = 3;

struct Philosopher {
    name: String,
    left_fork: usize,
    right_fork: usize,
    done_channel: Sender<bool>,
}

impl Philosopher {
    fn new(
        name: &str,
        left_fork: usize,
        right_fork: usize,
        done_channel: Sender<bool>,
    ) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left_fork,
            right_fork,
            done_channel,
        }
    }

    fn done(&self) {
        self.done_channel
            .send(true)
            .ok()
            .expect("Unable to send to done_channel");
    }

    fn eat(&self, table: &Table) {
        let _left_fork = table.forks[self.left_fork]
            .lock()
            .ok()
            .expect("Unable to lock left_fork mutex");
        let _right_fork = table.forks[self.right_fork]
            .lock()
            .ok()
            .expect("Unable to lock right_fork mutex");
        println!("        {} started eating", self.name);
        thread::sleep(EAT_TIME);
        println!("        {} ended eating", self.name);
    }

    fn think(&self) {
        println!("        {} started thinking", self.name);
        thread::sleep(THINK_TIME);
        println!("        {} ended thinking", self.name);

        self.done();
    }
}

struct Table {
    forks: Vec<Mutex<bool>>,
}

fn main() {
    println!("Dinning philosophers problem");
    println!("----------------------------");
    println!("Table is empty");

    let (tx, rx) = channel::<bool>();
    let (tx_seat, rx_seat) = channel();

    let philosophers = vec![
        Philosopher::new("Plato", 0, 1, tx.clone()),
        Philosopher::new("Socrtes", 1, 2, tx.clone()),
        Philosopher::new("Aristotele", 2, 3, tx.clone()),
        Philosopher::new("Pascal", 3, 4, tx.clone()),
        Philosopher::new("Locke", 4, 0, tx.clone()),
    ];

    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(true),
            Mutex::new(true),
            Mutex::new(true),
            Mutex::new(true),
            Mutex::new(true),
        ],
    });
    let event_number = philosophers.len() * EATING_COUNTER;

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let table = table.clone();
            let tx = tx_seat.clone();
            thread::spawn(move || {
                tx.send(format!("{} is seated", p.name))
                    .ok()
                    .expect("Unable to send message to seat channel");
                for i in 0..EATING_COUNTER {
                    println!("    {} meal for {}", i + 1, p.name);
                    p.eat(&table);
                    p.think();
                }
            })
        })
        .collect();

    rx_seat.recv().into_iter().for_each(|message| {
        println!("{}", message);
    });

    for _ in 0..event_number {
        rx.recv().ok().expect("Unable to receive done messages");
    }

    for handle in handles {
        handle.join().ok().expect("Unable to join threads");
    }

    println!("Table is empty");
}
