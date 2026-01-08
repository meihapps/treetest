# treetest

one cli for all your test frameworks


This is useful if a single repo uses multiple test frameworks (e.g. a multi-language project), or if you switch between projects with different stacks and want one consistent command to run tests everywhere.

`treetest` lets you run or list tests across multiple frameworks from a single command, automatically filtering out frameworks that aren’t installed.

it’s configurable via a simple json file.

---

## features

- run all tests in all available frameworks (default command):

```fish
treetest
# or
treetest run
```

- list all tests without executing them:

```fish
treetest list
```

- update frameworks index:

```fish
treetest update
```

- supports multiple frameworks via a json config
- automatically filters out missing frameworks
- easy to add custom frameworks without modifying rust code

---

## installation

install via cargo:

```fish
cargo install treetest
```

Or build from source:

```fish
git clone https://github.com/meihapps/treetest.git
cd treetest
cargo build --release
```

---

## configuration

the config file is located at:

linux/macos: ~/.config/treetest/frameworks.json (or `$XDG_CONFIG_HOME/treetest/frameworks.json` if `XDG_CONFIG_HOME` is set)

windows: %appdata%\treetest\frameworks.json

each framework is defined as a json object with the following fields:

- `name` (string): a unique name for the test framework.
- `list_cmd` (string): command to list all tests in the framework.
- `run_cmd` (string): command to run all tests in the framework.

example:

```json
[
  {
    "name": "PyTest",
    "list_cmd": "pytest --collect-only",
    "run_cmd": "pytest"
  },
  {
    "name": "Cargo",
    "list_cmd": "cargo test -- --list",
    "run_cmd": "cargo test"
  }
]
```

- only frameworks with available executables will be used.
- json allows adding new frameworks without touching rust code.

---

## future plans

- [x] make frameworks updatable without being destructive to user configs
- [ ] improve error messages
- [ ] add test filtering (run only tests matching a pattern or tag)

---

## license

`treetest` is licensed under mit see [license](license) for details.
