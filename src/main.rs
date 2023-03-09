use std::{
    sync::{
        mpsc::{channel, sync_channel, Receiver},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

const HAIR_CUT_TIME: Duration = Duration::from_secs(3);
const BARBER_NAP: Duration = Duration::from_secs(1);
const BARBERS: usize = 2;

fn get_client(client_channel: &Arc<Mutex<Receiver<String>>>) -> Option<String> {
    let client_channel = client_channel.lock().ok().unwrap();
    let mut client_iter = client_channel.try_iter();
    if let Some(client) = client_iter.next() {
        Some(client)
    } else {
        None
    }
}

fn barber(
    barber: String,
    client_channel: Arc<Mutex<Receiver<String>>>,
    barbers_done: Arc<Mutex<Receiver<bool>>>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        println!("{} goes to the waiting room and check for clients", barber);
        let mut is_sleeping = false;
        loop {
            if let Some(client) = get_client(&client_channel) {
                if !is_sleeping {
                    println!("{} is cutting {}'s hair", barber, client);
                    thread::sleep(HAIR_CUT_TIME);
                    println!("{} is finnished cutting {}'s hair", barber, client);
                } else {
                    println!("{} wakes {} up", client, barber);
                    is_sleeping = false;
                    println!("{} is cutting {}'s hair", barber, client);
                    thread::sleep(HAIR_CUT_TIME);
                    println!("{} is finnished cutting {}'s hair", barber, client);
                }
            } else {
                if !is_sleeping {
                    println!("{} gone to a nap", barber);
                    is_sleeping = true;
                    thread::sleep(BARBER_NAP);
                }
            }

            if barbers_done.lock().ok().unwrap().try_recv().is_ok() {
                break;
            }
        }
    })
}

fn main() {
    println!("The sleeping barber problem");
    println!("---------------------------");
    //let (client_tx, client_rx) = channel::<String>();
    let (client_tx, client_rx) = sync_channel::<String>(10);
    let (barber_tx, barber_rx) = channel();
    let client_rx = Arc::new(Mutex::new(client_rx));
    let barber_rx = Arc::new(Mutex::new(barber_rx));
    let mut handles = Vec::new();

    println!("The shop is open for the day!");

    for i in 0..BARBERS {
        let handle = barber(
            format!("Barber{}", i),
            Arc::clone(&client_rx),
            Arc::clone(&barber_rx),
        );
        handles.push(handle);
    }

    for i in 0..100 {
        if let Err(_) = client_tx.try_send(format!("Client{}", i)) {
            println!(
                "The waiting room is full, so {} leaves",
                format!("Client{}", i)
            );
        }
        thread::sleep(Duration::from_secs(1));
    }

    for _ in 0..BARBERS {
        barber_tx
            .send(true)
            .ok()
            .expect("Unable to send to barbers_done channel");
    }

    handles
        .into_iter()
        .for_each(|handle| handle.join().ok().unwrap());
}
