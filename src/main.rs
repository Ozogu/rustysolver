use rustysolver::cfr::CFR;
use rustysolver::kuhn::Kuhn;

fn main() {
    let mut cfr = CFR::new(Kuhn::new());
    let ev = cfr.train(100000);
    println!("Expected value: {}", ev);
    cfr.print_strategy();
}
