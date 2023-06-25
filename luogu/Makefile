.PHONY: new build

build:
	cmake -Bbuild && cmake --build build

new:
	@echo "Please input the problem ID:"
	@read id; \
	if [ -z "$$id" ]; then \
		echo "Error: ID is empty."; \
		exit 1; \
	fi; \
	if [ -d "$$id" ]; then \
		echo "Error: $$id already exists."; \
		exit 1; \
	fi; \
	mkdir "$$id"; \
	cp -r template/* "$$id"; \
	mv "$$id/template.cpp" "$$id/$$id.cpp"; \
	echo "Problem $$id created."