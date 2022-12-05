pub mod identifier {
    fsm::finite_state_machine! {
        states = {A, B},
        init = A,
        accept = {B},
        alphabet = char,
        transitions = {
            A, 'a'..='z' | 'A'..='Z' => B,
            B, 'a'..='z' | 'A'..='Z' | '0'..='9' => B,
        }
    }
}

pub mod constant {
    fsm::finite_state_machine! {
        states = {A, B, C},
        init = A,
        accept = {B, C},
        alphabet = char,
        transitions = {
            A, '0' => C,
            A, '1'..='9' => B,
            B, '0'..='9' => B,
        }
    }
}
