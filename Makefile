.PHONY: build
build:
	@PATH=/opt/local/bin:/opt/local/sbin:/opt/local/bin:/opt/local/sbin:/opt/local/bin:/opt/local/sbin:/Users/jessejones/.cargo/bin/:/usr/local/git/bin:/Users/jessejones/Library/Haskell/bin:/opt/local/bin:/opt/local/sbin:/opt/local/bin:/opt/local/sbin:/opt/local/bin:/opt/local/sbin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin cargo build

# This will update minor version numbers.
# To upate a major version number you need to edit the cargo file.
.PHONY: update
update:
	@cargo update

.PHONY: clean
clean:
	@rm -rf target
