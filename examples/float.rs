fn main() {
    let question = reequestty::Question::float("number")
        .message("What is your favourite number?")
        .validate(|num, _| {
            if num.is_finite() {
                Ok(())
            } else {
                Err("Please enter a finite number".to_owned())
            }
        })
        .build();

    println!("{:#?}", reequestty::prompt_one(question));
}
