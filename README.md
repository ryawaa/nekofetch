# nekofetch

uh cat based neofetch very early in development, some stuff broken idk its also untested on windows and linux

<img width="976" alt="image" src="https://github.com/user-attachments/assets/9569b9b8-d849-4846-8825-518f8bc17a2f">


##### Known Issues
- Storage is stupid, only shows latest drive (i used a for loop dont judge)
- Terminal Colors are wack
- config no work :(
- no linter
- untested on linux and winodws 
- host is unknown

### Args
```
      --no-ascii  Do not display the ASCII art (why)
      --mini      Display a minimal version (yes)
      --caps      Capitalize labels (why did i even put this)
      --blahaj    Display the Blahaj ASCII art (because why not)
      --colors    Display terminal colors (wack)
  -h, --help      Print help
  -V, --version   Print version 
```

### building/running
```bash
$ cargo build --release
```
```bash
$ cargo run dev
```

### credits
- [Joan Stark - Original ASCII](https://en.wikipedia.org/wiki/Joan_Stark)
