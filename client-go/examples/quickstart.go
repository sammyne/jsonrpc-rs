// go:build ignore

package main

// 这个示例程序和 ../../examples/quickstart/server.rs 对应测服务端兼容。

import (
	"fmt"

	"github.com/sammyne/jsonrpc-rs/client-go"
)

type Params struct {
	Msg string `json:"msg"`
}

type Result struct {
	Msg string `json:"msg"`
}

func mustDoRequests(addr string) {
	c, err := client.New(addr)
	if err != nil {
		panicf("build client: %v", err)
	}
	defer c.Close()

	{
		params := Params{Msg: "hello"}
		req := client.NewRequest("service.hello_world", params, "1234567890")

		var result Result
		if err := c.DoRequest(&req, &result); err != nil {
			panicf("#1 do request: %v", err)
		}
	}

	{
		params := Params{Msg: "you're late"}
		req := client.NewRequest("service.notify", params)

		if err := c.DoRequest(&req, nil); err != nil {
			panicf("#1 notify: %v", err)
		}
	}
}

func main() {
	const remoteAddr = "127.0.0.1:9123"

	mustDoRequests(remoteAddr)
	mustDoRequests(remoteAddr)
}

func panicf(format string, args ...interface{}) {
	panic(fmt.Sprintf(format, args...))
}
