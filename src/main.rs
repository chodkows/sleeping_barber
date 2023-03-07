use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

const HAIR_CUT_TIME: Duration = Duration::from_secs(3);

struct BarberShop {
    shop_capacity: usize,
    hair_cut_duration: Duration,
    number_of_barbers: usize,
    open: bool,
}

impl BarberShop {
    fn new(
        &self,
        barber: String,
        client_channel: Receiver<String>,
        barbers_done: Sender<bool>,
    ) -> JoinHandle<()> {
        let rx = Arc::new(Mutex::new(client_channel));
        let rx_clone = Arc::clone(&rx);
        thread::spawn(move || {
            println!("{} goes to the waiting room and check for clients", barber);
            rx_clone
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
}

fn main() {
    println!("Hello, world!");
}
