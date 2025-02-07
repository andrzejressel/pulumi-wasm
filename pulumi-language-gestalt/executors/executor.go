// Copyright 2022, Pulumi Corporation.  All rights reserved.

package executors

import (
	"fmt"

	"github.com/andrzejressel/pulumi-gestalt/pulumi-language-gestalt/fsys"
)

// GestaltExecutor abstracts interactions with a gestalt project, ability to build, run
// gestalt code, and detect plugin dependencies.
type GestaltExecutor struct {
	// User-friendly name of the executor.
	Name string

	// Path to the command to run.
	Cmd string

	// Optional dir to run the command in.
	Dir string

	// Command to run the Gestalt code - the main entrypoint.
	RunArgs []string

	// Command args to resolve dependencies and build; this will
	// be called after `pulumi new` on Gradle templates. Optional.
	BuildArgs []string

	// Command to autodetect and print Pulumi plugins depended on
	// by the Gestalt program.
	PluginArgs []string

	// Command to print the version of the command.
	VersionArgs []string
}

// GestaltExecutorOptions contains information used to pick an executor.
type GestaltExecutorOptions struct {
	// Current working directory. Abstract to enable testing.
	WD fsys.ParentFS

	// The value of `runtime.options.binary` setting from
	// `Pulumi.yaml`. Optional.
	Binary string

	// The value of `runtime.options.use-executor` setting from
	// `Pulumi.yaml`. Optional.
	UseExecutor string
}

type gestaltExecutorFactory interface {
	// NewGestaltExecutor tries configuring an executor from the given options.
	// May return nil if options are not-applicable.
	NewGestaltExecutor(GestaltExecutorOptions) (*GestaltExecutor, error)
}

func NewGestaltExecutor(opts GestaltExecutorOptions) (*GestaltExecutor, error) {
	e, err := combineGestaltExecutorFactories(
		&justfile{},
	).NewGestaltExecutor(opts)
	if err != nil {
		return nil, err
	}
	if e == nil {
		return nil, fmt.Errorf("failed to configure executor, tried: justfile")
	}
	return e, nil
}

type combinedGestaltExecutorFactory []gestaltExecutorFactory

func (c combinedGestaltExecutorFactory) NewGestaltExecutor(opts GestaltExecutorOptions) (*GestaltExecutor, error) {
	for _, v := range c {
		e, err := v.NewGestaltExecutor(opts)
		if err != nil {
			return nil, err
		}
		if e != nil {
			return e, nil
		}
	}
	return nil, nil
}

func combineGestaltExecutorFactories(variations ...gestaltExecutorFactory) gestaltExecutorFactory {
	return combinedGestaltExecutorFactory(variations)
}
