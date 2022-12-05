finite_state_machine! {
    states = {A, B, C},
    init = A,
    accept = {C},
    alphabet = char,
    transitions = {
        A, 'a' => B,
        B, 'b' => C,
        B, 'a' => B,
    }
}
