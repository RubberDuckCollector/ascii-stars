import os
import random
import readline
import my_modules


# TODO: make a program that creates space themed backgrounds/screensavers with ASCII art
# STEP 1. random distribution of stars
# STEP 2. specify new astronomical bodies (such as galaxies / extra big stars)
# STEP 3. accommodate for higher resolutions and make black holes / quasars etc
# STEP 4. define solar systems and include them at medium range
# STEP 5. define what terrestrial / gas giant planets could look like and
# create zoomed-in graphics of them
# STEP 6. define different galaxy types
# STEP 7. define different nebula types


class Color:
    Reset = "\033[0m"
    Red = "\033[031m"
    Green = "\033[32m"
    Yellow = "\033[33m"
    Blue = "\033[34m"
    Magenta = "\033[35m"
    Cyan = "\033[96m"
    LightGray = "\033[37m"
    DarkGray = "\033[90m"
    LightRed = "\033[91m"
    LightGreen = "\033[92m"
    LightYellow = "\033[93m"
    LightBlue = "\033[94m"
    LightMagenta = "\033[95m"
    LightCyan = "\033[96m"
    White = "\033[97m"
    Warn = "\033[93m"
    Underline = "\033[4m"
    Bold = "\033[1m"
    Hidden = "\033[8m"
    Blink = "\033[5m"
    Dim = "\033[2m"
    Reverse = "\033[7m"
    BlackBg = "\033[40m"


def main():
    terminal_size = os.get_terminal_size()
    print(terminal_size)
    print(terminal_size.columns)
    print(terminal_size.lines)

    os.system("clear")
    print(Color.BlackBg)
    print(Color.Dim)
    for i in range(terminal_size.lines):
        for j in range(terminal_size.columns):
            print(f" ", end='')
    print(Color.Reset, end='')
    input()  # locks the screen so you can review the output


if __name__ == "__main__":
    main()
