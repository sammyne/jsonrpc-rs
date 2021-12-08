package client_test

import (
	"encoding/json"
	"testing"

	"github.com/sammyne/jsonrpc-rs/client-go"
)

func TestRequestMarshalJSON(t *testing.T) {
	testVector := []struct {
		req    client.Request
		expect string
	}{
		{
			client.NewRequest("hello", []string{"world"}, "1"),
			`{"jsonrpc":"2.0","method":"hello","params":["world"],"id":"1"}`,
		},
	}

	for i, c := range testVector {
		got, err := json.Marshal(&c.req)
		if err != nil {
			t.Fatalf("#%d marshal: %v", i, err)
		}

		if c.expect != string(got) {
			t.Fatalf("#%d failed: expect %q, got %q", i, c.expect, got)
		}
	}
}
