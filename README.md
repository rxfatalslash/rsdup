<div align="center">
    <img src="https://i.imgur.com/om7h1jl.png" width="200px">

### 📋 RsDup 📋

RsDup is a simple program written in Rust that scans the user-specified path and its subdirectories for duplicate files, using the hash of each file to detect duplicates.
<br><br>
A Python version is available for those who prefer it. **[[PyDup](https://github.com/rxfatalslash/pydup.git)]**.
</div>

# Index
* ### [🗳️ Installation](#🗳️-installation)
* ### [🖱️ Use](#🖱️-use)
* ### [📋 License](#📋-license)

# 🗳️ Installation
You must have Rust and Cargo installed on your computer. **[[Installation Guide](https://www.rust-lang.org/tools/install)]**
<br>
Once you have installed the necessary software, clone the repository.
```
git clone git@github.com:rxfatalslash/rsdup.git
cd rsdup/
cargo build --release
```

# 🖱️ Use
Run the program and enter the path to be scanned.
```
$ ./target/release/rsdup
Enter the path of the directory you want to analyze: /home/user/
  Processing files... 2/2 [00:00:00] ██████████████████████████████████████████████████████████████████ 100%
Duplicate files with hash 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx':
 - /home/user/hello1.txt
 - /home/user/hello.txt
```

# 📋 License
This project is licensed under the terms of the [GNU General Public License, version 3](https://www.gnu.org/licenses/gpl-3.0.html) (GPLv3).

## LICENSE SUMMARY
### Permissions:

* **FREEDOM TO USE:** You are free to use, modify, and distribute this software.

* **SOURCE CODE ACCESS:** You must provide access to the source code of any modified versions of the software under the same GPLv3 license.

### Conditions:

* **COPYLEFT:** Any derivative work must also be open-source and distributed under the GPLv3 license.

* **NOTICES:** When distributing the software, you must include a copy of the GPLv3 license and provide appropriate notices.

### Limitations:

* **NO WARRANTY:** The software is provided as-is with no warranties or guarantees.

* **LIABILITY:** The authors or copyright holders are not liable for any damages or issues arising from the use of the software.

<a href="https://www.gnu.org/licenses/gpl-3.0.html" target="_blank">
  <img src="https://upload.wikimedia.org/wikipedia/commons/9/93/GPLv3_Logo.svg" width="80" height="15" />
</a>