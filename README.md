# Copilot, for your terminal

A CLI tool that generates shell scripts from a human readable description using Ollama.

## Installation

### Nix

Add `plz-cli` to your flake inputs:

```nix
{
  description = "Your nixos configuration";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    
    plz-cli = {
      url = "github:rukh-debug/plz-cli";
    };
  };
}
```

Then create a wrapper script in your home-manager configuration to set up the required environment variables:

```nix
{ inputs, pkgs, ... }:
let
  plz-wrapper = pkgs.writeShellScriptBin "plz" ''
    export OLLAMA_MODEL="qwen2.5-coder:7b"
    export OLLAMA_URL="http://localhost:11434"
    exec ${inputs.plz-cli.packages."${pkgs.system}".default}/bin/plz "$@"
  '';
in
{
  home.packages = with pkgs; [
    plz-wrapper
  ];
}
```

This wrapper automatically sets the required environment variables, so you don't need to configure them manually.


You may need to close and reopen your terminal after installation. Alternatively, you can download the binary corresponding to your OS from the [latest release](https://github.com/m1guelpf/plz-cli/releases/latest).

## Usage

`plz` uses [Ollama](https://ollama.com/) to generate shell commands. To use it, you'll need to have Ollama running and specify which model to use.

### Required Environment Variables

**OLLAMA_MODEL** (required): The Ollama model to use for generating commands
```bash
export OLLAMA_MODEL='qwen2.5-coder:7b'
```

**OLLAMA_URL** (optional): The Ollama server URL
```bash
export OLLAMA_URL='http://localhost:11434'
```
If not set, defaults to `https://ollama.rukh.me`. For local Ollama installations, use `http://localhost:11434`.

### Setup

1. First, make sure you have Ollama installed and running. Visit [ollama.com](https://ollama.com/) for installation instructions.

2. Pull the qwen2.5-coder:7b model:
   ```bash
   ollama pull qwen2.5-coder:7b
   ```

3. List available models on your system:
   ```bash
   ollama list
   ```

4. Set your environment variables (skip this step if using the Nix wrapper above):
   ```bash
   export OLLAMA_MODEL='qwen2.5-coder:7b'
   export OLLAMA_URL='http://localhost:11434'
   ```

   You can add these to your bash/zsh profile for persistence between sessions.

Once you have configured your environment, run `plz` followed by whatever it is that you want to do.

### Examples

```bash
plz "list all files in current directory"
plz "find all rust files and count lines of code"
plz "compress this folder into a tar.gz archive"
plz "show me disk usage for each directory"
```

To get a full overview of all available options, run `plz --help`

```sh
$ plz --help
Generates bash scripts from the command line

Usage: plz [OPTIONS] <PROMPT>

Arguments:
  <PROMPT>  Description of the command to execute

Options:
  -y, --force    Run the generated program without asking for confirmation
  -h, --help     Print help information
  -V, --version  Print version information
```

## Error Handling

If you encounter errors, here are some common issues and solutions:

- **"This program requires an Ollama model to be specified"**: You need to set the `OLLAMA_MODEL` environment variable
- **"model not found, try pulling it first"**: The specified model isn't available on your Ollama server. Run `ollama pull qwen2.5-coder:7b` to download it
- **Connection errors**: Check that your `OLLAMA_URL` is correct and the Ollama server is running

## Develop

Make sure you have the latest version of rust installed (use [rustup](https://rustup.rs/)). Then, you can build the project by running `cargo build`, and run it with `cargo run`.

## License

This project is open-sourced under the MIT license. See [the License file](LICENSE) for more information.
