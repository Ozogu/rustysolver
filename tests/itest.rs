use rustysolver::cfr::CFR;
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
