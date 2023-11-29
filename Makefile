.PHONY: thrift
thrift:
	thrift-0.17.0 -out ./src --gen rs -r tutorial.thrift
