use rustysolver::cfr::CFR;

#[test]
fn test_kuhn_poker_ev() {
    let mut cfr = CFR::new();
    let ev = cfr.train(12000);
    let ideal_ev = -1.0/18.0;
    let ev_diff = (ev - ideal_ev).abs();
    
    debug_assert!(ev_diff < 0.0011, 
        "Expected value: {:.4}, Ideal expected value: {:.4}, Difference: {:.4}",
        ev, ideal_ev, ev_diff);
}