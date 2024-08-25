# NAND2Tetris

Practice of the book **The Elements of Computing Systems**: Building a Modern Computer from First Principles

## Overview
This repository contains my solutions and experiments for the NAND2Tetris course, a comprehensive program that teaches how to build a computer from the ground up.

## Prerequisites
- Access to the [NAND2Tetris Software Suite](http://www.nand2tetris.org/software.php).
- Understanding of digital logic and basic computer architecture.

## Usage
The repository is structured according to the course's projects:
# NAND2Tetris Practice Repository Structure

## Project Overview

This repository is organized based on the chapters of the NAND2Tetris book, with each project corresponding to a specific chapter.

> Note: Inside the folder ``practice/chips`` are add-on chips made for practice, but those are not related to the standard course

Each folder includes the necessary `.hdl` files and other relevant resources.

To use a project's files:
1. Open the NAND2Tetris software.
2. Load the `.hdl` file from the project folder.
3. Execute the corresponding test script provided by the NAND2Tetris course materials.

## Simulators

To execute the Hardware simulator
```bash
make hardware
```

To execute the CPU simulator
```bash
make software
```

## License
This project is open source under the [MIT License](LICENSE).
