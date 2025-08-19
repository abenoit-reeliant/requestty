fn main() {
    reequestty::questions![Select {
        name: "name",
        default: 0,
        on_esc: reequestty::OnEsc::Terminate,
        transform: |_, _, _| Ok(()),
        choices: ["choice"],
        page_size: 10,
        should_loop: true,
    }];
}
