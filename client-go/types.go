package client

import "encoding/json"

type Error struct {
	Code    int             `json:"code"`
	Message string          `json:"message"`
	Data    json.RawMessage `json:"data"`
}

type Request struct {
	version string
	Method  string
	Params  interface{}
	ID      string
}

type Response struct {
	Version string          `json:"jsonrpc"`
	Result  json.RawMessage `json:"result,omitempty"`
	Error   *Error          `json:"error,omitempty"`
	ID      string          `json:"id,omitempty"`
}

type requestJSON struct {
	Version string      `json:"jsonrpc"`
	Method  string      `json:"method"`
	Params  interface{} `json:"params,omitempty"`
	ID      string      `json:"id,omitempty"`
}

func (err *Error) Error() string {
	out, _ := json.Marshal(err)
	return string(out)
}

func (r Request) MarshalJSON() ([]byte, error) {
	rr := requestJSON{
		Version: r.version,
		Method:  r.Method,
		Params:  r.Params,
		ID:      r.ID,
	}

	return json.Marshal(rr)
}

func NewRequest(method string, params interface{}, id ...string) Request {
	var requestID string
	if len(id) > 0 {
		requestID = id[0]
	}
	return Request{version: "2.0", Method: method, Params: params, ID: requestID}
}
