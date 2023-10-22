# FeOH Matchup Tool
A League of Legends Matchup Management Tool developed in Rust using [Iced](https://iced.rs/).

While most LoL Matchup Tools are based on statistics and winrates of the champions against each other, this information may not always reflect the current skill level, or champion proficiency of the player. For example, champions like Akali often have catastrophical winrates against most champions, even though the matchups are more than playable for an experienced Akali player.

Thus, this is a matchup tool solely for your own matchup experience. With this tool, you will be able to save matchups and counters in an efficient manner, with a distinction in matchup difficulty in three different levels.

But, since only your own experiences are taken into account, the tool will be empty from the start, with the ability to fill in counters and matchups step by step.

## How to install
1. Install [Rust and Cargo](https://www.rust-lang.org/tools/install)
2. Clone this repository using ```git clone https://github.com/fexfl/FeOHMatchupTool``` at your desired destination
3. Run the Tool using the Windows SuperShell script ```run.ps1``` on Windows, or on Linux using ```cargo run``` from inside ```/feoh_matchup_tool/```
4. An empty matchup file will be automatically created in ```/feoh_matchup_tool/data/``` that can be customized from inside the Tool

## How to use
Select a champion from the combo box to reveal all its counters recorded in the matchup data file.

To add a new counter, select the counter from the adjacent combo box, as well as a matchup difficulty, and press the "Add Counter" button.
The changes will automatically be saved to the data file.

To remove a counter that was previously added, pick the counter from the combo box, and press the "Remove Counter" button.

## Notes
This tool uses Riot's Data Dragon for champion squares.

FeOH Matchup Tool is not endorsed by Riot Games and does not reflect the views or opinions of Riot Games or anyone officially involved in producing or managing Riot Games properties. Riot Games and all associated properties are trademarks or registered trademarks of Riot Games, Inc
