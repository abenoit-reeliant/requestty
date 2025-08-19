fn main() {
    reequestty::questions! [
        OrderSelect {
            name: "name",
            choices: [
                sep "separator",
            ],
        }
    ];
}
