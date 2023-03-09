use std::{
    sync::{
        mpsc::{channel, Receiver},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

const HAIR_CUT_TIME: Duration = Duration::from_secs(3);
const BARBER_NAP: Duration = Duration::from_secs(1);
const BARBERS: usize = 3;

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
        loop {
            if let Some(client) = get_client(&client_channel) {
                println!("{} is cutting {}'s hair", barber, client);
                thread::sleep(HAIR_CUT_TIME);
                println!("{} is finnished cutting {}'s hair", barber, client);
            } else {
                println!("{} gone to a nap", barber);
                thread::sleep(BARBER_NAP);
            }

            if barbers_done.lock().ok().unwrap().try_recv().is_ok() {
                break;
            }
        }
    })
}

fn main() {
    let (client_tx, client_rx) = channel::<String>();
    let (barber_tx, barber_rx) = channel();
    let client_rx = Arc::new(Mutex::new(client_rx));
    let barber_rx = Arc::new(Mutex::new(barber_rx));
    let mut handles = Vec::new();

    for i in 0..BARBERS {
        let handle = barber(
            format!("Barber{}", i),
            Arc::clone(&client_rx),
            Arc::clone(&barber_rx),
        );
        handles.push(handle);
    }

    for i in 0..10 {
        client_tx
            .send(format!("Client{}", i))
            .ok()
            .expect("Unable to send client");
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
