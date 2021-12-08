package client

import (
	"encoding/binary"
	"encoding/json"
	"fmt"
	"io"
	"net"
)

type Client struct {
	conn net.Conn
}

func (c *Client) DoRequest(request *Request, result interface{}) error {
	requestJSON, err := json.Marshal(request)
	if err != nil {
		return fmt.Errorf("marshal request: %w", err)
	}

	var lengthLE [8]byte
	binary.LittleEndian.PutUint64(lengthLE[:], uint64(len(requestJSON)))

	if _, err := c.conn.Write(lengthLE[:]); err != nil {
		return fmt.Errorf("send length prefix: %w", err)
	}
	if _, err := c.conn.Write(requestJSON); err != nil {
		return fmt.Errorf("send request: %w", err)
	}

	if request.ID == "" { // no wait for notification
		return nil
	}

	var replyLen uint64
	if err := binary.Read(c.conn, binary.LittleEndian, &replyLen); err != nil {
		return fmt.Errorf("recv response length: %w", err)
	}

	var reply Response
	reader := &io.LimitedReader{R: c.conn, N: int64(replyLen)}
	if err := json.NewDecoder(reader).Decode(&reply); err != nil {
		return fmt.Errorf("recv and unmarshal response: %w", err)
	}

	if reply.Error != nil {
		return fmt.Errorf("no result: %w", reply.Error)
	}

	if err := json.Unmarshal(reply.Result, result); err != nil {
		return fmt.Errorf("unmarshal result: %w", err)
	}

	return nil
}

func (c *Client) Close() error {
	return c.conn.Close()
}

func New(remoteAddr string) (*Client, error) {
	conn, err := net.Dial("tcp", remoteAddr)
	if err != nil {
		return nil, fmt.Errorf("dial: %w", err)
	}

	return &Client{conn: conn}, nil
}
