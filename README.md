# rust-new-project-template
A good starting point for a new Rust project

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)

## Introduction
This is a simple Rust project that reads a CSV file and calculates the mean of each column of numerical data. It demonstrates how to use Rust's standard library and external crates to perform basic data analysis tasks.

## Prerequisites
To run this project, you will need to have Rust and Cargo installed on your system. You can download Rust and Cargo from the official Rust website.

## Getting Started
Create a new CSV file in the project directory with numerical data. Each row should have the same number of columns, and the columns should contain only numerical values. Here's an example CSV file:

![image](https://user-images.githubusercontent.com/122952572/223771370-72599f5c-74f4-4525-aab7-732a980280a3.png)

Save this file as data.csv.

Finally, run the program with the following command:
`cargo run`

The program will read the CSV file and print the mean of each column to the console:

![image](https://user-images.githubusercontent.com/122952572/223771681-5990243a-0ca7-409a-979c-81790139befb.png)
