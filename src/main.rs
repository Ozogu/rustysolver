use rustysolver::cfr::CFR;
use rustysolver::leduc::Leduc;
use rustysolver::history::History;

fn main() {
    let mut cfr = CFR::new(Leduc::new());
    let ev = cfr.train(100);
    println!("Expected value: {:.4}", ev);
    cfr.print_strategy();

    let statistics = cfr.build_statistics();
    let strategy_ev = statistics.node_util(&History::new());
    let br_ev = statistics.node_br_util(&History::new());
    let explitability = statistics.node_exploitability(&History::new());
    println!("Strategy EV: {:.4}, BR EV: {:.4} Exploitability: {:.2} %", strategy_ev, br_ev, explitability);
}
