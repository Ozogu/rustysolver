use rustysolver::cfr::CFR;
use rustysolver::history::History;
use rustysolver::kuhn::Kuhn;
use rustysolver::leduc::Leduc;

#[test]
fn test_kuhn_poker_ev() {
    let mut cfr = CFR::new(Kuhn::new());
    let ev = cfr.train(12000);
    let ideal_ev = -1.0/18.0;
    let ev_diff = (ev - ideal_ev).abs();

    debug_assert!(ev_diff < 0.0011, 
        "EV: {:.4}, Ideal: {:.4}, Diff: {:.4}",
        ev, ideal_ev, ev_diff);
        
    let statistics = cfr.build_statistics();
    let statistics_ev = statistics.get_node_util(&History::new());
    let statistics_ev_diff = (statistics_ev - ideal_ev).abs();

    debug_assert!(statistics_ev_diff < 0.0008, 
        "Statistics EV: {:.4}, Ideal: {:.4}, Diff: {:.4}",
        statistics_ev, ideal_ev, statistics_ev_diff);
}

#[test]
fn test_leduc_poker_ev() {
    let mut cfr = CFR::new(Leduc::new());
    let ev = cfr.train(12000);
    let ideal_ev = 0.0;
    let ev_diff = (ev - ideal_ev).abs();
    
    debug_assert!(ev_diff < 0.0011, 
        "EV: {:.4}, Ideal: {:.4}, Diff: {:.4}",
        ev, ideal_ev, ev_diff);
}
