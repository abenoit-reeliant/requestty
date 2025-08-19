fn main() {
    reequestty::questions![Expand {
        name: "name",
        default: 'c',
        on_esc: reequestty::OnEsc::Terminate,
        transform: |_, _, _| Ok(()),
        choices: [('c', "choice")],
        page_size: 10,
        should_loop: true,
    }];
}
