# Changelog

## [v0.1.7] - 2024-11-08

### Changed
- Upgraded dependencies.
- Fixed broken links.

## [v0.1.3] - 2023-04-20

### Changed
- Messages informing about differences between files:
  - option `-a` `--absolute`
    
    before:
    ```
    differences exceed the limit <limit>: <diff>
    ```
    after:
    ```
    <file_1> <file_2> differ: limit <limit> exceeded by value <diff>
    ```
  - option `-p` `--percent`
    before:
    ```
    differences exceed the limit <limit>%: <diff>%
    ```
    after:
    ```
    <file_1> <file_2> differ: limit <limit>% exceeded by value <diff>%
    ``` 

### Removed
- Removed messages displayed when files are equal or similar:
  - before:
    ```
    differences within the limit <limit>: <diff>
    ```
  - after change: - (no message)
