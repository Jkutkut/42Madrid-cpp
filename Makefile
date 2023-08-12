OS = $(shell uname -s)

# Define PWD
ifeq ($(OS),Darwin)
	PWD = $(PWD)
else
	PWD = $(shell pwd)
endif

# Template script

template_repo:
	git clone git@github.com:fsandel/template.git template_repo

template_repo/template: template_repo
	make -C template_repo all

template: template_repo/template
	cp template_repo/template .

# project script

