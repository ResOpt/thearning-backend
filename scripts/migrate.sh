#! /usr/bin/bash

source "./.env"

for i in {0..12}
do
	diesel migration revert
	diesel migration revert --database-url $DATABASE_URL_TEST
done

diesel migration run
diesel migration run --database-url $DATABASE_URL_TEST
cargo test -- --test-threads 1