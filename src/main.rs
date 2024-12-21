use rustysolver::cfr::CFR;
use rustysolver::kuhn::Kuhn;

fn main() {
    let mut cfr = CFR::new(Kuhn::new());
    let ev = cfr.train(100000);
    println!("Expected value: {}", ev);

    for card in 0..3 {
        for history in ["", "p", "b", "pp", "bp", "bb"].iter() {
            let info_set = format!("{}{}", card, history);
            let avg_strategy = cfr.get_average_strategy(&info_set);
            if avg_strategy.is_some() {
                let avg_strategy = avg_strategy.unwrap();
                println!("Average strategy for {}: [{:.2}, {:.2}]",
                    info_set, avg_strategy[0], avg_strategy[1]);
            }        
        }
    }
}
