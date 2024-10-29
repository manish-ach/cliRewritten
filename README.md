# cliRewritten
CLI tools rewritten by beginner in rust

# prerequisites(for installing)
1. rust must be installed

# installing
1. clone the repo
```
https://github.com/manish-ach/cliRewritten.git
```

2. cd into desired directory(cat/echo)
```
cd cat
```
or
```
cd echo
```

3. build the project
```
cargo build --release
```
do it for both directory or desired ones

4. open the shell configuration file(.zshrc or .bashrc)
```
nano $HOME/.zshrc
```
if bash open .bashrc instead


5. add path
```
alias recho = path/to/the/target/release/echo
alias rcat = path/to/the/target/release/cat
```
always have the path end with cat or echo which are the build files.

6. refresh the shell
```
source $HOME/.zshrc #for zsh
```

```
source $HOME/.bashrc #for bash
```

7. running
run then by using recho or rcat
