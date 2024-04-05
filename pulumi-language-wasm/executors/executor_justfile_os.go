// Copyright 2022, Pulumi Corporation.  All rights reserved.

package executors

import (
	"github.com/andrzejressel/pulumi-wasm/pulumi-language-wasm/fsys"
)

type justfile struct{}

var _ wasmExecutorFactory = &justfile{}

func (s justfile) NewWasmExecutor(opts WasmExecutorOptions) (*WasmExecutor, error) {
	exists, err := fsys.FileExists(opts.WD, "justfile")
	if err != nil {
		return nil, err
	}
	if !exists {
		return nil, nil
	}
	return s.newWasmCliExecutor()
}

func (justfile) newWasmCliExecutor() (*WasmExecutor, error) {
	return &WasmExecutor{
		Name:        "just",
		Cmd:         "just",
		BuildArgs:   []string{"build"},
		RunArgs:     []string{"run"},
		PluginArgs:  []string{"plugin"},
		VersionArgs: []string{"version"},
	}, nil
}
