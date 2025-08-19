fn main() {
    reequestty::questions![Confirm {
        name: "name",
        default: true,
        transform: |_, _, _| Ok(()),
        on_esc: reequestty::OnEsc::Terminate,
    }];
}
