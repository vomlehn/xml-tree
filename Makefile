TEST = --test=test3

test:
	clear
	cargo test --jobs=1 -- --nocapture 
