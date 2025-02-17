# @tablex/core

## 0.3.1

### Patch Changes

- [#37](https://github.com/kareemmahlees/tablex/pull/37) [`9c475f5`](https://github.com/kareemmahlees/tablex/commit/9c475f5e02221b5ea0528ce04d0705e59d1da253) Thanks [@dbarnett](https://github.com/dbarnett)! - Support other common SQLite file extensions (like .sqlite)

## 0.3.0

### Minor Changes

- [#34](https://github.com/kareemmahlees/tablex/pull/34) [`1021466`](https://github.com/kareemmahlees/tablex/commit/102146644874808c5145f959482413d08c9eb6aa) Thanks [@kareemmahlees](https://github.com/kareemmahlees)! - Added pagination and used Tanstack Virtual for table view

## 0.2.6

### Patch Changes

- [`d62ad6e`](https://github.com/kareemmahlees/tablex/commit/d62ad6e204eb61284fca638a5a4d3fa56b303257) Thanks [@kareemmahlees](https://github.com/kareemmahlees)! - Fix annoying lint error in setup

## 0.2.5

### Patch Changes

- [`482136a`](https://github.com/kareemmahlees/tablex/commit/482136af44950f459545814a5cb14210d01de4df) Thanks [@kareemmahlees](https://github.com/kareemmahlees)! - Fix lint error in main.rs

## 0.2.4

### Patch Changes

- [#29](https://github.com/kareemmahlees/tablex/pull/29) [`6806384`](https://github.com/kareemmahlees/tablex/commit/6806384861018d57358797280b22a52e4174bfd6) Thanks [@kareemmahlees](https://github.com/kareemmahlees)! - ## Modifications

  ### Backend

  - Fix a bug while creating the connections file, previously it didn't create the parent directories before creating the file.
  - Return empty Vector if there are not tables in the database

  ### Frontend

  - Make creating a connection more verbose with toast info

## 0.2.3

### Patch Changes

- [`68a5127`](https://github.com/kareemmahlees/tablex/commit/68a5127b5b8bf87e4aad62143356dd82b86079a3) Thanks [@kareemmahlees](https://github.com/kareemmahlees)! - make tauri action work on release

## 0.2.2

### Patch Changes

- [`070018d`](https://github.com/kareemmahlees/tablex/commit/070018de800bba51a564fcb08489082b73be3149) Thanks [@kareemmahlees](https://github.com/kareemmahlees)! - fix some errors in release ci

## 0.2.1

### Patch Changes

- [`6fe8cd7`](https://github.com/kareemmahlees/tablex/commit/6fe8cd7b1909f27024754ed66bbaacf71a7583ca) Thanks [@kareemmahlees](https://github.com/kareemmahlees)! - fix some issues in readme

## 0.2.0

### Minor Changes

- [#20](https://github.com/kareemmahlees/tablex/pull/20) [`232d431`](https://github.com/kareemmahlees/tablex/commit/232d431637e0cc6edd86dd687fddd518214076f1) Thanks [@kareemmahlees](https://github.com/kareemmahlees)! - _TableX_ now uses _MetaX_ as a sidecar to expose a RESTfull and GraphQL APIs for the connected database.

  For optimization reasons, this is only enabled in **release builds**, dev and debug builds are opted out.

## 0.1.7

### Patch Changes

- [`574e072`](https://github.com/kareemmahlees/tablex/commit/574e0724f5c777ab639a4f9986555646f82d32c2) Thanks [@kareemmahlees](https://github.com/kareemmahlees)! - make tauri.config.json use package.json for versioning

## 0.1.6

### Patch Changes

- [`e6764be`](https://github.com/kareemmahlees/tablex/commit/e6764be67a244db399b62f0e237e20aaaf8a66d4) Thanks [@kareemmahlees](https://github.com/kareemmahlees)! - fixing download links

## 0.1.5

### Patch Changes

- [`f79f512`](https://github.com/kareemmahlees/tablex/commit/f79f512ab14647e23e5cea27efd90aedbe290632) Thanks [@kareemmahlees](https://github.com/kareemmahlees)! - use RELEASE_TOKEN in release workflow

## 0.1.4

### Patch Changes

- [`934e9d3`](https://github.com/kareemmahlees/tablex/commit/934e9d3b8532f933c5fd016b6b6ea4136dbd37b2) Thanks [@kareemmahlees](https://github.com/kareemmahlees)! - remove branch-ignores in workflows

## 0.1.3

### Patch Changes

- [`5fa4e83`](https://github.com/kareemmahlees/tablex/commit/5fa4e830ce854f795b6184a542af297399d6843d) Thanks [@kareemmahlees](https://github.com/kareemmahlees)! - minor fix to changeset action

## 0.1.2

### Patch Changes

- [`c92efee`](https://github.com/kareemmahlees/tablex/commit/c92efee21cd409e24d0e55e8e24cc8e01e0777d5) Thanks [@kareemmahlees](https://github.com/kareemmahlees)! - adding a fake publish script to create github releases

## 0.1.1

### Patch Changes

- [#9](https://github.com/kareemmahlees/tablex/pull/9) [`2c72c41`](https://github.com/kareemmahlees/tablex/commit/2c72c410190a25409ebab039d737c517f87c5302) Thanks [@kareemmahlees](https://github.com/kareemmahlees)! - First stable version of TableX
