use std::sync::mpsc;
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let mut data = vec![1; 10];
    let mut snd_channel = Vec::new();
    let mut rcv_channel = Vec::new();

    for _ in 0..10 {
        let (snd_tx, snd_rx) = mpsc::channel();
        let (rcv_tx, rcv_rx) = mpsc::channel();

        snd_channel.push(snd_tx);
        rcv_channel.push(rcv_rx);

        handles.push(thread::spawn(move || {
            let mut data = snd_rx.recv().unwrap();
            data += 1;
            let _ = rcv_tx.send(data);
        }));
    }

    for x in 0..10 {
        let _ = snd_channel[x].send(data[x]);
    }

    for x in 0..10 {
        data[x] = rcv_channel[x].recv().unwrap();
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);

}
