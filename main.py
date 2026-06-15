import os
import random
import readline
import subprocess
import my_modules


def main():
    terminal_size = os.get_terminal_size()
    print(terminal_size)
    print(terminal_size.columns)
    print(terminal_size.lines)

    subprocess.run("clear")
    print(my_modules.Color.BlackBg)
    print(my_modules.Color.Dim)
    for i in range(terminal_size.lines):
        for j in range(terminal_size.columns):
            print(f" ", end='')
    print(my_modules.Color.Reset, end='')
    input()  # locks the screen so you can review the output


if __name__ == "__main__":
    main()
