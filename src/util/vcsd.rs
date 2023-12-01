use std::collections::HashMap;
use std::io;
use std::cmp;
use rgsl::randist::binomial::{binomial_pdf, self};

/*
 * This is the the example of how to use the program
 */
// fn main() {
//
//     let testg = generate_vcsd(input_to_hashmap());
//     write(&testg);
//
// }


fn check_nan<I : Iterator<Item = f64>>(mut iter : I) {
    for i in iter {
        if i.is_nan() {
            panic!("nan detected");
        }
    }
}

fn input_to_hashmap() -> HashMap<u64, f64> {//read input name and convert to Hashmap
    let mut rdr = csv::ReaderBuilder::new()
        .from_reader(io::stdin());
    let mut _result:HashMap<u64, f64> = HashMap::new();
    for result in rdr.records() {
        let record = result.unwrap();
        _result.insert(record.get(0).unwrap().parse().unwrap(), record.get(1).unwrap().parse().unwrap());
    }
    return _result;
}

macro_rules! panic_on_nan {
    ($v:expr, $($arg:tt)*) => {
        if $v.is_nan() {
            panic!($($arg)*);
        }
    };
}


//this is the convolution with hashmap
fn convolute(x: &HashMap<u64, f64>, y:HashMap<u64, f64>) -> HashMap<u64, f64>{
    let mut result:HashMap<u64, f64> = HashMap::new();
    for (key_x, val_x) in x.iter(){
        for (key_y, val_y)  in y.iter(){
            match result.get(&(key_x + key_y)){
                Some(a) => {
                    panic_on_nan!(a + (val_x * val_y), "{a} + ({val_x} * {val_y}) is nan");
                    result.insert(key_x + key_y, a + (val_x * val_y))
                },
                None => { 
                    panic_on_nan!(val_x * val_y, "{val_x} * {val_y} is nan");
                    result.insert(key_x + key_y, val_x * val_y)
                }
            };
        }
    }
    return result;
}


// main logic method, it takes a vector of tenancy distribution and return the according Hashmap of
// DCS distribution.
pub fn generate_vcsd(input:HashMap<u64, f64>) -> HashMap<u64, f64> {
    //https://stackoverflow.com/questions/70667002/can-i-iterate-in-order-on-hashmapu64-mystruct
    let mut keys: Vec<&u64> = input.keys().collect();
    keys.sort_unstable();
    let mut z: HashMap<u64, f64> = HashMap::new();
    let mut curr_front = 0;
    let mut curr_end;
    while curr_front < keys.len(){
        curr_end = curr_front;//searching start with this index
        let mut proba : f64 = 0.00;//initialize probability
        while curr_end < keys.len() {//iterate over rest of the lease
            proba = proba + input.get(keys.get(curr_end).unwrap()).unwrap();
            curr_end = curr_end + 1;
        }
        if curr_front == 0{
            assert!((f64::abs(proba - 1.0)) < 0.000000001);
            z = pack_to_binomial(**keys.get(curr_front).unwrap(), proba);//need to remove
            curr_front += 1;
            continue;
        }
        let temp = pack_to_binomial(**keys.get(curr_front).unwrap() - **keys.get(curr_front - 1).unwrap(), proba);
        check_nan(temp.values().copied());
        z = convolute(&mut z, temp);
        curr_front += 1;
        check_nan(z.values().copied());
    }
    return z;
}


fn write(output:&HashMap<u64, f64>){
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let mut keys: Vec<&u64> = output.keys().collect();
    keys.sort_unstable();
    wtr.write_record(&["DCS", "probability"]).expect("cannot write");
    for key in keys{
        wtr.write_record(&[key.to_string(), output.get(key).unwrap().to_string()]).expect("cannot write");
    }
}


//the same packing method, but return a hashmap
fn pack_to_binomial(times:u64, mut proba:f64) -> HashMap<u64, f64>{
    let mut curr = 0;
    let mut v =HashMap::new();
    while curr <= times {
        proba = if proba > 1.0 {1.0} else {proba};
        let result = binomial_pdf(curr as u32, proba, times as u32);
        if (result.is_nan()){
            println!("{curr} {proba} {times}");
            panic!("binomial_pdf is nan");
        }
        v.insert(curr,binomial_pdf(curr as u32, proba, times as u32)) ;
        curr += 1;
    }
    return  v;
}