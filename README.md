
# Directory Cleaner Tool

## **Overview**
This is a command-line tool written in Rust that helps users clean their directories by removing unwanted files based on various filters such as file type, size, age, and more. The tool offers a range of features to enhance file management efficiency, including safe mode (dry run), backups, duplicate file removal, logging, and configuration file support.

---

## **Features**
1. **File Type Filtering**  
   Clean files based on specific file extensions such as `.log`, `.tmp`, or `.bak`.  
   **Use Case**: Target specific types of files for removal.

2. **File Size Filtering**  
   Remove files that exceed or fall below a certain size.  
   **Use Case**: Clean large old backups or small unwanted files.

3. **File Age Filtering**  
   Clean files based on the last modified or accessed date.  
   **Use Case**: Remove files that haven't been accessed or modified in a long time.

4. **Safe Mode (Dry Run)**  
   Preview files that will be deleted without actually removing them.  
   **Use Case**: Review the list of files to avoid accidental deletion.

5. **Duplicate File Finder and Cleaner**  
   Identify and remove duplicate files in the directory.  
   **Use Case**: Clean up storage by eliminating redundant file copies.

6. **Customizable Ignore List**
   Specify files or directories to ignore during the cleaning process.  
   **Use Case**: Protect important files or directories from deletion.

7. **Logging and Reporting**  
   Generate logs or reports of all actions taken, including files deleted and errors encountered.  
   **Use Case**: Keep a record of operations for future reference.

---

## **Installation**

### **Pre-requisites**
- **Rust**: You need to have Rust installed on your system. You can install Rust by following [this link](https://www.rust-lang.org/tools/install).

### **Build from Source**
1. Clone the repository:
   ```bash
   git clone https://github.com/yahayaohinoyi/dir-cleaner-rust.git
   ```
2. Navigate into the directory:
   ```bash
   cd dir-cleaner-rust.git
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. Run the tool:
   ```bash
   ./target/release/dir-cleaner-rust
   ```

### **Using Homebrew (Mac Only)**
You can install the tool using **brew** by running the following command:
```bash
brew tap mubarak-ramadan/homebrew-dir-cleaner-rust
```
then:
```bash
brew install dir-cleaner-rust
```

---

## **Usage**

### **Basic Command**

You can run `dir-cleaner-rust -h` to see the available commands

```bash
dir-cleaner-rust clean <directory> --filter <options>
```

### **Command Examples**
- **Dry Run**: Preview files that would be deleted
  ```bash
  dir-cleaner-rust -n -d <DIR>
  ```

- **Clean Specific File Types**:
  ```bash
    dir-cleaner-rust -d <DIR> -s <SIZE>
  ```

- **Remove Files Based on Size**:
  ```bash
    dir-cleaner-rust -d <DIR> -s <SIZE>
  ```

- **Clean Files Base on Age Than 30 Days**:
  ```bash
  dir-cleaner-rust -d <DIR> -a <AGE>
  ```

- **Cleanup Duplicate Files**:
  ```bash
  dir-cleaner-rust -r -d <DIR>
  ```

---

## **Contributing**

Contributions are welcome! If you find a bug or have a feature request, please open an issue or submit a pull request. Before submitting a pull request, ensure the following:

1. Your code passes all tests.
2. Your code follows Rust coding standards and is properly formatted.

### **Running Tests**
```bash
cargo test
```

### **CI/CD Pipeline**
The project uses a CI/CD pipeline for automatic deployment of the binaries. Once merged, changes are automatically tested and, if successful, deployed.

---

## **Planned Features**

Some features are currently out of scope but may be added in future versions:
- **Multithreading for Large Directories**: Improve performance for handling large directories.
- **Support for Network Drives or Cloud Storage**: Clean directories on remote locations.

---

## **License**
This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

---

## **Acknowledgements**
Special thanks to all contributors and users of this tool.

---

## **Contact**
For any queries or support, please open an issue on GitHub or reach out to the maintainers via email at `yahaya.suleiman2204@gmail.com` or `mohammedmubarak314@gmail.com`.
