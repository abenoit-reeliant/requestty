fn main() {
    let question = reequestty::Question::editor("description")
        .message("Please enter a short description about yourself")
        .extension(".md")
        .validate(|answer, _| {
            if answer.lines().count() < 3 {
                Err("Must be at least 3 lines.".into())
            } else {
                Ok(())
            }
        })
        .build();

    println!("{:#?}", reequestty::prompt_one(question));
}
