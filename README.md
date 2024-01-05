# Project Forge
Easy CLI tool for creating projects from templates

# Usage
```bash
project-forge --help
```
```
Usage: project-template.exe [OPTIONS] <PROJECT_NAME> <PATH> <TEMPLATE>

Arguments:
  <PROJECT_NAME>  Project name
  <PATH>          Output project directory
  <TEMPLATE>      Template content path. Can be directory or ZIP file (must have "zip" extension)

Options:
  -p, --params <KEY:VALUE>  Additional parameters map
  -v, --verbose             Print CLI information
  -h, --help                Print help
  -V, --version             Print version
```

Typical use:
```bash
project-forge awesome-project ./template/output ./template/input
```
Produces files structure from `./template/input` source folder or ZIP file, which gets generated into `./template/output` directory, with `awesome-project` as `PROJECT_NAME` for `*.chrobry` file templates being processed into regular text files.

Passing additional parameters for replacement in `*.chrobry` file templates.
```bash
project-forge other-project ./template/output ./template/input.zip -p "EVERYTHING:42"
```
More about `*.chrobry` file templates: https://github.com/PsichiX/Chrobry