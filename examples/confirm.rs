fn main() {
    let question = reequestty::Question::confirm("anonymous")
        .message("Do you want to remain anonymous?")
        .build();

    println!("{:#?}", reequestty::prompt_one(question));
}
