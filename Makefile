TEST = --test=test3
TEST =

test:
	clear
	cargo test --jobs=1 -- --nocapture 
