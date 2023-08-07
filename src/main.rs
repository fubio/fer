use clap::Parser;
mod util;
mod bin;
use bin::model::FER_calculator;
use bin::simulator::caching;
use bin::simulator::Sampler;

#[derive(Parser, Debug)]
struct Args {
    //csv that stores tenancy and count
    #[arg(short, long)]
    td_csv: String,
}

fn main() {
    let parsed_args = Args::parse();
    let csv = parsed_args.td_csv;
    let mut reader = csv::Reader::from_path(csv.clone()).unwrap();
    let total = reader.records().fold(0, |acc, result| acc + result.unwrap()[1].parse::<u64>().unwrap());
    let mut reader = csv::Reader::from_path(csv).unwrap();
    let td: Vec<(u64, f64)> = reader.records().map(|result| {
        let record = result.unwrap();
        let tenancy: u64 = record[0].trim().parse().unwrap();
        let number: u64 = record[1].trim().parse().unwrap();
        (tenancy, number as f64 / total as f64)
    }).collect();

    let model = FER_calculator::new(td.clone());
    let (model_overage, model_unstored, model_fer, pcs) = model.get_results();
    println!("model overage : {}", model_overage);
    println!("model unstored : {}", model_unstored);
    println!("model fer : {}", model_fer);

    let (simulated_overage, simulated_unstored, simulated_fer) = caching(Sampler::new(td.into_iter()), pcs, 0.001);
    println!("simulated overage : {}", simulated_overage);
    println!("simulated unstored : {}", simulated_unstored);
    println!("simulated fer : {}", simulated_fer);

}
