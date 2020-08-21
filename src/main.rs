use rustyline::{At, Cmd, Editor, KeyPress, Movement, Word};

fn main() {
    let src = std::env::args().nth(1).unwrap();
    let mut rl = Editor::<()>::new();
    rl.bind_sequence(
        KeyPress::ControlLeft,
        Cmd::Move(Movement::BackwardWord(1, Word::Emacs)),
    );
    rl.bind_sequence(
        KeyPress::ControlRight,
        Cmd::Move(Movement::ForwardWord(1, At::AfterEnd, Word::Emacs)),
    );
    let dst = rl.readline_with_initial("Move to:\n", (&src, "")).unwrap();
    std::fs::rename(src, dst).unwrap()
}
