OS = $(shell uname -s)

# Define PWD
ifeq ($(OS),Darwin)
	PWD = $(PWD)
else
	PWD = $(shell pwd)
endif

all: usage

# Template script

template_repo:
	git clone git@github.com:fsandel/template.git template_repo

template_repo/template: template_repo
	make -C template_repo all

template: template_repo/template
	cp template_repo/template .

# project script

toolkit/target/debug/toolkit:
	make -C toolkit build_dev
toolkit/target/release/toolkit:
	make -C toolkit build_release_binary

./toolkit_script: toolkit/target/debug/toolkit
#./toolkit: toolkit/target/release/toolkit
	cp $< $@

# project

clean:
	rm -rf template_repo

fclean: clean
	rm -f template
	rm -f toolkit_script

# ------------ Commands ------------

usage:
	@echo "Usage: make [command]"
	@echo ""
	@echo "Commands:"
	@echo "  new_project: Create a new project"
	@echo "  clean: Remove all generated files"
	@echo "  fclean: Remove all generated files and downloaded files"

next_project: ./toolkit_script
	./$< new_project
