#! /usr/bin/bash

for i in {0..13}
do
	diesel migration revert
done

diesel migration run
