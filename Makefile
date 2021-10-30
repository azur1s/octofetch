help:
	@echo -e "Commands\n"
	@echo -e "  build\t 	 compile octofetch "
	@echo -e "  install\t compile and install octofetch"
build:
	@cargo build
install: 
	@cargo install --path .