use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

const HAIR_CUT_TIME: Duration = Duration::from_secs(3);

fn barber(
    barber: String,
    client_channel: Arc<Mutex<Receiver<String>>>,
    barbers_done: Sender<bool>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        println!("{} goes to the waiting room and check for clients", barber);
        client_channel
            .lock()
            .ok()
            .unwrap()
            .recv()
            .into_iter()
            .for_each(|client| {
                println!("{} is cutting {}'s hair", barber, client);
                thread::sleep(HAIR_CUT_TIME);
                println!("{} is finnished cutting {}'s hair", barber, client);
            });
        barbers_done
            .send(true)
            .ok()
            .expect("Unable to send to barbers_done channel");
    })
}

fn main() {
    let (client_tx, client_rx) = channel::<String>();
    let (barber_tx, barber_rx) = channel();
    let client_rx = Arc::new(Mutex::new(client_rx));
    let mut handles = Vec::new();

    for i in 0..1 {
        let handle = barber(
            format!("Barber{}", i),
            Arc::clone(&client_rx),
            barber_tx.clone(),
        );
        handles.push(handle);
    }

    for i in 0..100 {
        client_tx
            .send(format!("Client{}", i))
            .ok()
            .expect("Unable to send client");
        thread::sleep(Duration::from_secs(1));
    }

    handles
        .into_iter()
        .for_each(|handle| handle.join().ok().unwrap());
}
