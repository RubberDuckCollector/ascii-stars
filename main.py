import os
import random
import readline
import my_modules


# TODO: make a program that creates space themed backgrounds/screensavers with ASCII art
# STEP 1. random distribution of stars
#   - star distribution should be proportional to terminal area (set by a hard-coded number)
#   - there should be a vocabulary of symbols that the program can use to represent stars
#       * to make it more interesting
#       * the user/programmer should be able to specify more stylized / more realistic
# STEP 2. specify new astronomical bodies (such as galaxies / extra big stars)
# STEP 3. accommodate for higher resolutions and make black holes / quasars etc
# STEP 4. define solar systems and include them at medium range
# STEP 5. define what terrestrial / gas giant planets could look like and
# create zoomed-in graphics of them
# STEP 6. define different galaxy types
# STEP 7. define different nebula types


def main():
    terminal_size = os.get_terminal_size()
    print(terminal_size)
    print(terminal_size.columns)
    print(terminal_size.lines)

    os.system("clear")
    print(my_modules.Color.BlackBg)
    print(my_modules.Color.Dim)
    for i in range(terminal_size.lines):
        for j in range(terminal_size.columns):
            print(f" ", end='')
    print(my_modules.Color.Reset, end='')
    input()  # locks the screen so you can review the output


if __name__ == "__main__":
    main()
