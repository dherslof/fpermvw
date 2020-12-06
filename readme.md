# fpermvw - File permission viewer

## Description
fpermvw is a simple service for checking basic linux file permissions with human
readable output. It can also be used for calculating permission numbers which can be used with `chmod`. 

fpermvw is mostly made for personal Rust testing/exploring but may or may not be useful for someone else. 

## Building
Clone sources and build with Cargo.

## Example usage
Calculate file permission:

```bash
$ fpermvw calc -u rwe -g rwe -o rwe
```
Will return output: 
```bash
Permission number representation: 777
```
Print number representation of file permission: 
```bash
$ fpermvw print <file_name> num
```
Possible output: 
```bash
Permission number representation of '<file_name>' is: 0644
```
