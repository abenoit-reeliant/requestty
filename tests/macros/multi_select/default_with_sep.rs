fn main() {
    reequestty::questions! [
        MultiSelect {
            name: "name",
            choices: [
                sep "separator" default true,
            ],
        }
    ];
}
