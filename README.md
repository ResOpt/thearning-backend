# About This Project
Thearning is an LMS (Learning Management System) written in Rust.
It is my project for the new learning method for Software Engineering Major
at Vocational Highschool: PJBL or "Project-Based Learning".

## Why Rust?
Because it's the best language.

## No, seriously. Why are you writing it in Rust?
I was serious, tho. But well of course the main reason is for learning purpose.
I want to learn [rocket.rs](https://rocket.rs) and as well as
rest API. Also writing Rust is really fun - that's why.

# Will it actually be used?
Hope so. If this project's finished, I will offer this to my school.
The project will remain open source, but I probably will ask for some fee
for maintaining to my school.

# Contribution
PRs are welcome and they would probably save my time, too.

# Building
This project uses rocket v5 so you don't need to use nightly toolchain!

Here are the steps to build this project:

1. Install rustup
2. Install the stable rust toolchain
3. Install postgresql
4. Setup your postgresql database
5. Install diesel_cli with `cargo install diesel_cli --no-default-features --features "postgres"` to only install the postgres feature
6. Clone and `cd` to this project's directory
7. Configure your database URL in the `.env` file
8. Run `diesel migration run` to migrate the models
9. Run `cargo run` to run the development server

