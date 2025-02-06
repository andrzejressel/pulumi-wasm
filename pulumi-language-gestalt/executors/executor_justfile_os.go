// Copyright 2022, Pulumi Corporation.  All rights reserved.

package executors

import (
	"github.com/andrzejressel/pulumi-gestalt/pulumi-language-gestalt/fsys"
)

type justfile struct{}

var _ gestaltExecutorFactory = &justfile{}

func (s justfile) NewGestaltExecutor(opts GestaltExecutorOptions) (*GestaltExecutor, error) {
	exists, err := fsys.FileExists(opts.WD, "justfile")
	if err != nil {
		return nil, err
	}
	if !exists {
		return nil, nil
	}
	return s.newGestaltCliExecutor()
}

func (justfile) newGestaltCliExecutor() (*GestaltExecutor, error) {
	return &GestaltExecutor{
		Name:        "just",
		Cmd:         "just",
		BuildArgs:   []string{"build"},
		RunArgs:     []string{"run"},
		PluginArgs:  []string{"plugins"},
		VersionArgs: []string{"version"},
	}, nil
}
