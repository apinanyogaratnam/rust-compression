VERSION := 0.1.0

tag:
	git tag -m "v${VERSION}" v${VERSION}
	git push --tags

format:
	cargo fmt
