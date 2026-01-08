# treetest

one cli for all your test frameworks

`treetest` lets you run or list tests across multiple frameworks from a single command, automatically filtering out frameworks that aren’t installed.

it’s configurable via a simple json file.

---

## features

- run all tests in all available frameworks (default command):

```fish
treetest
```

- list all tests without executing them:

\```bash
treetest list
treetest -l
\```

- supports multiple frameworks via a json config (`frameworks.json`)
- automatically filters out missing frameworks
- easy to add custom frameworks without modifying rust code

---

## installation

install via cargo:

\```bash
cargo install treetest
\```

Or build from source:

\```bash
git clone https://github.com/YOUR_USERNAME/treetest.git
cd treetest
cargo build --release
\```

---

## configuration

create a `frameworks.json` in your project directory:

\```json
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
\```

- only frameworks with available executables will be used.
- json allows adding new frameworks without touching rust code.

---

## future plans

- [ ] add test filtering (run only tests matching a pattern or tag)

---

## license

`treetest` is licensed under **mit**. see [license](license) for details.
