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
To build the project, you will need the nightly toolchain of Rust, 
since this project uses rocket 0.4.

Here are the steps to build this project:

1. Install [diesel_cli]("https://crates.io/crates/diesel_cli") with `cargo install diesel_cli`. Remember, you need it to be executable. You should add the Rust bin directory to PATH if you haven't.
2. clone this repository with
`git clone https://github.com/13ers/thearning-backend`.
3. Install postgresql if you haven't, and then create a new database with the name `thearningdb`.
4. `cd` into the thearning-backend directory.
5. Set your database url properly inside the `.env` file.
6. Run `diesel migration run` to migrate the models.
7. Run `cargo run` to compile and run the development server.

