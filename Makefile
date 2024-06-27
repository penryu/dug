all:
	false

publish:
	cargo readme > README.md
	cargo publish
