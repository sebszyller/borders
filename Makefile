.PHONY: all clean

all:
	python3 resize.py *.jpg

clean:
	rm -f *.jpg
	rm -rf resized
