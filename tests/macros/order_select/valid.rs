fn main() {
    let choice = "choice";

    reequestty::questions![OrderSelect {
        name: "name",
        on_esc: reequestty::OnEsc::Terminate,
        transform: |_, _, _| Ok(()),
        validate: |_, _| Ok(()),
        filter: |t, _| t,
        choices: ["choice", choice],
        page_size: 10,
        should_loop: true,
    }];
}
