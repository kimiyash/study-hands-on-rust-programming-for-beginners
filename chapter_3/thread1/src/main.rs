//use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let mut handles = Vec::new();
    // let mut data = vec![1; 10];
    // let mut data = Rc::new(vec![1; 10]);
    // let mut data = Arc::new(vec![1; 10]);
    let data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone(); // Vec をクローンではなく Arc<Mutex<Vec<i32>>> をクローンしている
        handles.push(thread::spawn(move || {
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }))
    }

    for handle in handles {
        let _ = handle.join();
    }
    
    dbg!(data);
}
