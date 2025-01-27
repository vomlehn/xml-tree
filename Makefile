TEST = test3

test:
	clear
	cargo test --jobs=1 -- --nocapture --test=$(TEST)
