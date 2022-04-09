#! /usr/bin/bash

for i in {0..9}
do
	diesel migration revert
done

diesel migration run
