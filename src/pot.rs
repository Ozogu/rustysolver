use std::collections::HashMap;
use crate::player::Player;
use crate::action::Action;

#[derive(Clone, Debug)]
pub struct Pot {
    pot: HashMap<Player, f64>,
}

impl Pot {
    pub fn new(ip: f64, oop: f64) -> Self {
        Pot {
            pot: HashMap::from([(Player::IP, ip), (Player::OOP, oop)]),
        }
    }

    pub fn total(&self) -> f64 {
        self.pot[&Player::IP] + self.pot[&Player::OOP]
    }

    pub fn update(&mut self, player: Player, action: Action) {
        match action {
            Action::Check | Action::Fold => (),
            Action::Bet(amount) => {
                *self.pot.get_mut(&player).unwrap() +=
                    self.bet_amount(self.total(), amount);
            },
            Action::Raise(amount) => {
                let to_call = self.to_call();
                *self.pot.get_mut(&player).unwrap() += 
                    self.bet_amount(self.total() +  to_call, amount) + to_call;
            },
            Action::Call => {
                self.pot.insert(player, self.pot[&player.opponent()]);

            },
        }
    }

    pub fn payoff(&self, player: Player, won: Option<bool>) -> f64 {
        match won {
            Some(true) => match player {
                // Win what the opponent contributed
                player => self.pot[&player.opponent()],
            },
            Some(false) => {
                // Lose what you contributed
                -self.pot[&player]
            },
            None => {
                // In case of draw, pot size should be the same
                assert!(self.pot[&Player::IP] == self.pot[&Player::OOP]);
                self.pot[&Player::IP]
            },
        }
    }

    fn to_call(&self) -> f64 {
        (self.pot[&Player::IP] - self.pot[&Player::OOP]).abs()
    }

    fn bet_amount(&self, pot: f64, bet_size: u32) -> f64 {
        let bet_fraction = (bet_size as f64) / 100.0;
        pot * bet_fraction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_new() {
        let pot = Pot::new(1.0, 1.0);
        assert_eq!(pot.pot, HashMap::from([(Player::IP, 1.0), (Player::OOP, 1.0)]));
    }

    #[test]
    fn test_get_win_amount_winning() {
        let pot = Pot::new(1.0, 2.0);
        assert_eq!(pot.payoff(Player::IP, Some(true)), 2.0);
        assert_eq!(pot.payoff(Player::OOP, Some(true)), 1.0);
    }

    #[test]
    fn test_get_win_amount_losing() {
        let pot = Pot::new(1.0, 2.0);
        assert_eq!(pot.payoff(Player::IP, Some(false)), -1.0);
        assert_eq!(pot.payoff(Player::OOP, Some(false)), -2.0);
    }

    #[test]
    fn test_get_win_amount_on_draw() {
        let pot = Pot::new(1.0, 1.0);
        assert_eq!(pot.payoff(Player::IP, None), 1.0);
    }

    #[test]
    fn test_total() {
        let pot = Pot::new(1.0, 1.0);
        assert_eq!(pot.total(), 2.0);
    }

    #[test]
    fn test_update_bet() {
        let mut pot = Pot::new(1.0, 1.0);
        pot.update(Player::OOP, Action::Bet(50));
        assert_eq!(pot.pot, HashMap::from([(Player::IP, 1.0), (Player::OOP, 2.0)]));
    }

    #[test]
    fn test_update_raise() {
        let mut pot = Pot::new(3.0, 1.0);
        pot.update(Player::OOP, Action::Raise(50));
        assert_eq!(pot.pot, HashMap::from([(Player::IP, 3.0), (Player::OOP, 6.0)]));
    }

    #[test]
    fn test_update_call() {
        let mut pot = Pot::new(2.0, 1.0);
        pot.update(Player::OOP, Action::Call);
        assert_eq!(pot.pot, HashMap::from([(Player::IP, 2.0), (Player::OOP, 2.0)]));
    }

    #[test]
    fn test_update_bet_call() {
        let mut pot = Pot::new(1.0, 1.0);
        pot.update(Player::OOP, Action::Bet(50));
        pot.update(Player::IP, Action::Call);
        assert_eq!(pot.pot, HashMap::from([(Player::IP, 2.0), (Player::OOP, 2.0)]));
    }

    #[test]
    fn test_update_bet_raise() {
        let mut pot = Pot::new(1.0, 1.0);
        pot.update(Player::OOP, Action::Bet(100));
        pot.update(Player::IP, Action::Raise(50));
        assert_eq!(pot.pot, HashMap::from([(Player::IP, 6.0), (Player::OOP, 3.0)]));
    }

    #[test]
    fn test_update_bet_raise_call() {
        let mut pot = Pot::new(1.0, 1.0);
        pot.update(Player::OOP, Action::Bet(100));
        pot.update(Player::IP, Action::Raise(50));
        pot.update(Player::OOP, Action::Call);
        assert_eq!(pot.pot, HashMap::from([(Player::IP, 6.0), (Player::OOP, 6.0)]));
    }
}