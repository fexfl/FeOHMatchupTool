# FeOH Matchup Tool
A League of Legends Matchup Management Tool developed in Rust using [Iced](https://iced.rs/).

While most LoL Matchup Tools are based on statistics and winrates of the champions against each other, this information may not always reflect the current skill level, or champion proficiency of the player.
Thus, this is a matchup tool solely for your own matchup experience. With this tool, you will be able to save matchups and counters in an efficient manner, with a distinction in matchup difficulty in three different levels.

## How to install
1. Install [Rust and Cargo](https://www.rust-lang.org/tools/install)
2. Clone this repository using ```git clone https://github.com/fexfl/FeOHMatchupTool``` at your desired destination
3. (Optional, but recommended) Contact me to retrieve the image files used inside the Tool and drop them in the ```/feoh_matchup_tool/img/``` folder
4. Run the Tool using the Windows SuperShell script ```run.ps1``` on Windows, or on Linux using ```cargo run``` from inside ```/feoh_matchup_tool/```
5. An empty matchup file will be created in ```/feoh_matchup_tool/data/``` that can be customized from inside the Tool

## How to use
Select a champion from the combo box to reveal all its counters recorded in the matchup data file.
To add a new counter, select the counter from the adjacent combo box, as well as a matchup difficulty, and press the "Add Counter" button.
The changes will automatically be saved to the data file.
To remove a counter that was previously added, pick the counter from the combo box, and press the "Remove Counter" button.
