# Soduku - December 2025 Challenge

Solve a soduku board (3x3 quads of 3x3) as fast as you can

![alt text](https://github.com/samcolak/dec25_soduku/blob/main/boardpic.png?raw=true)

## What is Soduku

Sudoku is a logic-based number puzzle played on a 9x9 grid, which is also divided into nine 3x3 blocks. The goal is to fill the grid with the numbers 1 through 9 so that each number appears only once in every row, column, and 3x3 block. It is a game of logic and deduction, not guessing

## Rules

1. A horitonal and vertical line cannot contain more than one instance of a number [1..9] at any time
2. A quad (a collection of 3x3) cannot contain more than one instance of a number [1..9] at any time
3. The game ends when all spaces contain numbers [1..9]

## Solver

1. You must create a "solver" than resolves the board (without modifying the original board code) - using the supplied "solver.rs"
2. Example board manipulation can be found in "main.rs"

## Challenge

Part of the Rust developers challenge

## Info

1. Run using the command <code>cargo run ./board1.json</code>
2. Board information is a json array of moves in the form "column.row.value" - One move per array item
3. Not specifying a file will result in a blank board