fn main() {
    let question = reequestty::Question::int("age")
        .message("What is your age?")
        .default(69)
        .validate_on_key(|age, _| age > 0 && age < 130)
        .validate(|age, _| {
            if age > 0 && age < 130 {
                Ok(())
            } else {
                Err(format!("You cannot be {} years old!", age))
            }
        })
        .build();

    println!("{:#?}", reequestty::prompt_one(question));
}
