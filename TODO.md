TODO: make a program that creates space themed backgrounds/screensavers with ASCII art
STEP 1. random distribution of stars
  - star distribution should be proportional to terminal area (set by a hard-coded number)
  - there should be a vocabulary of symbols that the program can use to represent stars
      * to make it more interesting
      * the user/programmer should be able to specify more stylized / more realistic
STEP 2. specify new astronomical bodies (such as galaxies / extra big stars)
STEP 3. accommodate for higher resolutions and make black holes / quasars etc
STEP 4. define solar systems and include them at medium range
STEP 5. define what terrestrial / gas giant planets could look like and
create zoomed-in graphics of them
STEP 6. define different galaxy types
STEP 7. define different nebula types

TODO: get a 2d array the size of the terminal
do passes of star arrangement algorithms on the 2d array to progressively add symbols to the image
need priority queue for which celestial objects need to be in fg/bg? or just the correct order in which to run the algorithms?


KEY: 🟢 completed
     🟡 WIP
     🔴 found issues

| Feature                  | Status |
|--------------------------|--------|
| save to .png, .pdf, etc. |        |
