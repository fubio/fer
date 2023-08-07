use clap::Parser;
use std::collections::HashMap;
use std::cmp;

#[derive(Parser, Debug)]
struct Args {
    //csv that stores reference and tenancy for each reference
    #[arg(short, long)]
    lease_csv: String,

    //csv that stores the reference and reuse interval (RI) for each reference
    #[arg(short, long)]
    ri_csv: String,
}


fn csv_to_map(file_path_str: String) -> HashMap<String, u64> {
    let mut map = HashMap::new();
    println!("{}", file_path_str);
    let mut reader = csv::Reader::from_path(file_path_str).unwrap();
    reader.records().for_each(|result| {
        let record = result.unwrap();
        let reference: String = record[0].trim().parse().unwrap();
        let val: u64 = record[1].trim().parse().unwrap();
        map.insert(reference, val);
    });
    map
}

fn convert_to_td(reference_ri_map: HashMap<String, u64>, reference_lease_map: HashMap<String, u64>) -> HashMap<u64, u64> {
    let mut td = HashMap::new();
    reference_ri_map.iter().for_each(|(reference, ri)| {
        let lease = reference_lease_map.get(reference).unwrap();
        let tenancy = cmp::min(ri, lease);
        td.insert(*tenancy, td.get(tenancy).unwrap_or(&0) + 1);
    });
    td
}

fn main () {
    let parsed_args = Args::parse();
    let reference_lease_map = csv_to_map(parsed_args.lease_csv);
    let reference_ri_map = csv_to_map(parsed_args.ri_csv);
    let td = convert_to_td(reference_ri_map, reference_lease_map);
    let mut writer = csv::Writer::from_path("td.csv").unwrap();
    writer.write_record(&["tenancy", "number"]).unwrap();
    td.iter().for_each(|(tenancy, number)| {
        writer.write_record(&[tenancy.to_string(), number.to_string()]).unwrap();
    });
    writer.flush().unwrap();
}