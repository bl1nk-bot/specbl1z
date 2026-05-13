#!/bin/bash
protoc -I=../schema --plugin=./node_modules/.bin/protoc-gen-ts_proto --ts_proto_out=. --ts_proto_opt=esModuleInterop=true,forceLong=string,useFormatter=false ../schema/bl1nk.proto