package bpm

import (
	"fmt"
	"io"
	"reflect"
	"strings"

	"github.com/4rchr4y/goray/constant"
)

type Command interface {
	Name() string
	Requires() []string
	SetCommand(cmd Command) error
	Execute(input interface{}) error

	bpmCmd()
}

const (
	BuildCommandName    = "build"
	ValidateCommandName = "validate"
)

// ----------------- Build Command ----------------- //

type tomlClient interface {
	Decode(data string, v interface{}) error
	Encode(w io.Writer, v interface{}) error
}

type bundleBuilder interface {
	Build(input *BundleBuildInput) error
}

type buildCommand struct {
	cmdName     string
	toml        tomlClient
	bbuilder    bundleBuilder
	subregistry cmdRegistry
}

func (cmd *buildCommand) bpmCmd()      {}
func (cmd *buildCommand) Name() string { return cmd.cmdName }

func (cmd *buildCommand) Requires() []string {
	return []string{
		ValidateCommandName,
	}
}

func (cmd *buildCommand) SetCommand(c Command) error {
	_, ok := cmd.subregistry[c.Name()]
	if ok {
		return fmt.Errorf("command '%s' in '%s' command is already exists", c.Name(), cmd.cmdName)
	}

	cmd.subregistry[c.Name()] = c
	return nil
}

type BuildCmdExecuteInput struct {
	*ValidateCmdExecuteInput

	_          [0]int
	SourcePath string
	DestPath   string
	BLWriter   io.Writer
}

func (cmd *buildCommand) Execute(input interface{}) error {
	typedInput, ok := input.(*BuildCmdExecuteInput)
	if !ok {
		return fmt.Errorf("type '%s' is invalid input type for '%s' command", reflect.TypeOf(input).Elem().Kind().String(), cmd.cmdName)
	}

	validateCmd := cmd.subregistry[ValidateCommandName]
	if err := validateCmd.Execute(typedInput.ValidateCmdExecuteInput); err != nil {
		return err
	}

	fmt.Println(typedInput.SourcePath)

	bundleName := strings.ReplaceAll(typedInput.ValidateCmdExecuteInput.BundleFile.Package.Name, ".", "_")
	bbInput := &BundleBuildInput{
		SourcePath: typedInput.SourcePath,
		DestPath:   typedInput.DestPath,
		BundleName: fmt.Sprintf("%s%s", bundleName, constant.BPMBundleExt),
		BLWriter:   typedInput.BLWriter,
	}
	if err := cmd.bbuilder.Build(bbInput); err != nil {
		return err
	}

	return nil
}

type BuildCmdInput struct {
	FsWrap           fsWrapper
	Tar              tarClient
	Toml             tomlClient
	RegoFileLoader   regoFileLoader
	BundleLockWriter io.Writer
}

func NewBuildCommand(input *BuildCmdInput) Command {
	bbuilder := &BundleBuilder{
		fswrap: input.FsWrap,
		tar:    input.Tar,
		toml:   input.Toml,
		loader: input.RegoFileLoader,
	}

	return &buildCommand{
		cmdName:     BuildCommandName,
		toml:        input.Toml,
		bbuilder:    bbuilder,
		subregistry: make(cmdRegistry),
	}
}

// ----------------- Validate Command ----------------- //

type validateClient interface {
	ValidateStruct(s interface{}) error
}

type validateCommand struct {
	cmdName  string
	validate validateClient
}

func (cmd *validateCommand) bpmCmd()                  {}
func (cmd *validateCommand) Name() string             { return cmd.cmdName }
func (cmd *validateCommand) Requires() []string       { return nil }
func (cmd *validateCommand) SetCommand(Command) error { return nil }

type ValidateCmdExecuteInput struct {
	BundleFile *BundleFile
}

func (cmd *validateCommand) Execute(input interface{}) error {
	typedInput, ok := input.(*ValidateCmdExecuteInput)
	if !ok {
		return fmt.Errorf("type '%s' is invalid input type for '%s' command", reflect.TypeOf(input), cmd.cmdName)
	}

	if err := typedInput.BundleFile.Validate(cmd.validate); err != nil {
		return fmt.Errorf("failed to execute '%s' command: %v", cmd.cmdName, err)
	}

	return nil
}

type ValidateCmdInput struct {
	Validate validateClient
}

func NewValidateCommand(input *ValidateCmdInput) Command {
	return &validateCommand{
		cmdName:  ValidateCommandName,
		validate: input.Validate,
	}
}
