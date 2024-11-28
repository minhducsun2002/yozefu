// https://extism.org/docs/quickstart/plugin-quickstart

package main

import (
	"errors"
	"fmt"
	"strings"

	"github.com/extism/go-pdk"
)

type KafkaRecord struct {
	Value     string      `json:"value"`
	Key       string      `json:"key"`
	Topic     string      `json:"topic"`
	Timestamp int64       `json:"timestamp"`
	Partition int         `json:"partition"`
	Offset    int         `json:"offset"`
	Headers   interface{} `json:"headers"`
}

type FilterParams = []string

type FilterInput struct {
	Record KafkaRecord  `json:"record"`
	Params FilterParams `json:"params"`
}

type FilterResult struct {
	Match bool `json:"match"`
}

//export matches
func Matches() int32 {
	// TODO - Edit the code as per your requirements
	input := FilterInput{}
	err := pdk.InputJSON(&input)
	param := input.Params[0]
	if err != nil {
		pdk.SetError(err)
		return 1
	}
	if strings.HasSuffix(input.Record.Key, param) {
		err = pdk.OutputJSON(FilterResult{Match: true})
		if err != nil {
			pdk.SetError(err)
			return 1
		}
		return 0
	}
	_ = pdk.OutputJSON(FilterResult{Match: false})
	return 0
}

//export parse_parameters
func ParseParameters() int32 {
	// TODO - Edit the code as per your requirements
	var params FilterParams
	err := pdk.InputJSON(&params)
	if err != nil {
		pdk.SetError(err)
		return 1
	}
	if len(params) != 1 {
		pdk.SetError(errors.New(fmt.Sprintf("This search filter expects a string argument. Found %v arguments", len(params))))
		return 1
	}
	return 0
}

func main() {}
