use clap::Parser;
mod bin;
mod util;
use util::markov;
use bin::model::FERCalculator;
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

    let model = FERCalculator::new(td.clone(), 1.0);
    let (model_overage, tenancy_remaining_per_access, model_fer, pcs, model_overage_normalized, efficiency) = model.get_results();
    let model_unstored = (tenancy_remaining_per_access /pcs as f64)*(model_overage_normalized + pcs as f64);
    println!("model overage :            {:.5}", model_overage);
    // println!("model overage normalized : {}", model_overage_normalized);
    // println!("model tenancy remaining per access : {}", tenancy_remaining_per_access);
    // println!("model unstored : {}", model_unstored);
    println!("old model fer :            {:.5}", model_fer);
    println!("new model fer :            {:.5}", model_overage/model_unstored);

    let (simulated_overage, simulated_unstored, simulated_fer) = caching(Sampler::new(td.into_iter()), pcs, 0.0002);
    // println!("simulated overage : {}", simulated_overage);
    // println!("simulated unstored : {}", simulated_unstored);
    let efficiency_adjusted_fer = model_overage/(efficiency*model_unstored);
    println!("PCS:                       {:.5}", pcs);
    println!("lower bound simulated fer: {:.5}", simulated_overage/simulated_unstored);
    println!("simulated fer :            {:.5}", simulated_fer);
    println!("efficiency :               {:.5}", efficiency);
    println!("efficiency calculated fer: {:.5}", efficiency_adjusted_fer); 
    println!("fer model vs simulation    {:.5}", efficiency_adjusted_fer/simulated_fer)

}
