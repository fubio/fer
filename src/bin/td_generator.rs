use clap::Parser;
use std::collections::HashMap;
use std::cmp;
use rand::Rng;

#[derive(Parser, Debug)]
struct Args {
    //csv that stores reference and tenancy for each reference
    #[arg(short, long)]
    lease_csv: String,

    //csv that stores the reference and reuse interval (RI) for each reference
    #[arg(short, long)]
    ri_csv: String,

    //td that stores tenancy histogram (tenancy, count)
    #[arg(short, long)]
    td_csv: String,
}


fn lease_to_map(file_path_str: String) -> HashMap<u64, (u64, u64, f64)> {
    let mut map = HashMap::new();
    let mut reader = csv::ReaderBuilder::new().has_headers(false).from_path(file_path_str).unwrap();
    reader.records().for_each(|result| {
        let record = result.unwrap();
        let reference: u64 = record[1].trim().parse().unwrap();
        let short_lease = u64::from_str_radix(record[2].trim(), 16).unwrap();
        let long_lease = u64::from_str_radix(record[3].trim(), 16).unwrap();
        // println!("short_lease: {}, long_lease: {}", short_lease, long_lease);
        let short_lease_prob = record[4].trim().parse::<f64>().unwrap();
        let random_number = rand::thread_rng().gen_range(0.0..1.0);
        // let lease = if random_number < short_lease_prob {short_lease} else {long_lease};
        map.insert(reference, (short_lease, long_lease, short_lease_prob));
    });
    map
}

fn ri_to_vec(file_path_str: String) -> Vec<(u64, i64)> {
    let mut vec = Vec::new();
    let mut reader = csv::ReaderBuilder::new().has_headers(false).from_path(file_path_str).unwrap();
    reader.records().for_each(|result| {
        let record = result.unwrap();
        let reference: u64 = u64::from_str_radix(record[0].trim(), 16).unwrap();
        let val: i64 = i64::from_str_radix(record[1].trim(), 16).unwrap();
        vec.push((reference, val));
    });
    vec
}

fn convert_to_td(reference_ri_vec: Vec<(u64, i64)>, reference_lease_map: HashMap<u64, (u64, u64, f64)>) -> HashMap<u64, u64> {
    let mut td = HashMap::new();
    reference_ri_vec.iter().for_each(|(reference, ri)| {
        // println!("reference: {}, ri: {}, lease: {}", reference, ri, lease);
        let leases_tuple = reference_lease_map.get(reference).unwrap();
        let mut short_lease = &leases_tuple.0;
        let long_lease = &leases_tuple.1;
        let short_lease_prob = &leases_tuple.2;
        if *ri == 4294967295 {
            let tenancy = (short_lease.clone() as f64 * short_lease_prob + long_lease.clone() as f64 * (1.0-short_lease_prob)).round() as u64;
            td.insert(tenancy, td.get(&tenancy).unwrap_or(&0) + 1);
        } else if (ri.clone() as u64) <= *short_lease {
            // ri < short_lease
            td.insert(ri.clone() as u64, td.get(&(ri.clone() as u64)).unwrap_or(&0) + 1);

        } else if (ri.clone() as u64) <= *long_lease {
            //short_lease < ri <= long_lease
            let tenancy = (short_lease.clone() as f64 * short_lease_prob + ri.clone() as f64 * (1.0-short_lease_prob)).round() as u64;
            td.insert(tenancy, td.get(&tenancy).unwrap_or(&0) + 1);
        } else {
            // ri > long_lease
            let tenancy = (short_lease.clone() as f64 * short_lease_prob + long_lease.clone() as f64 * (1.0-short_lease_prob)).round() as u64;
            td.insert(tenancy, td.get(&tenancy).unwrap_or(&0) + 1);
        }
    });
    td
}

fn main () {
    let parsed_args = Args::parse();
    let reference_lease_map = lease_to_map(parsed_args.lease_csv);
    let reference_ri_vec = ri_to_vec(parsed_args.ri_csv);
    let td = convert_to_td(reference_ri_vec, reference_lease_map);
    let mut writer = csv::Writer::from_path(parsed_args.td_csv).unwrap();
    writer.write_record(&["tenancy", "number"]).unwrap();
    td.iter().for_each(|(tenancy, number)| {
        writer.write_record(&[tenancy.to_string(), number.to_string()]).unwrap();
    });
    writer.flush().unwrap();
}