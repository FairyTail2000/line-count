# Line Counter

Small program to count the lines in the specified or current directory. It does not care if the file is a binary or text file. If it contains the bytes `0xA` or `0xD 0xA` it's a line

## Usage

```shell 
line_count -i
```
(includes hidden directories and files)

```shell
linecount -l some_dir_or_file
```
(uses the directory as a root or if it's a file only counts the file)
