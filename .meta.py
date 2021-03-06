web = "https://codeforces.com"
profile = web + "/profile/{handle}"

file_template = "{id}/README.md"
main_file = "README.md"

main_template = """# Codeforces

Transcription of my submissions for the Codeforces contests: [{web}]({web})

Handle: [![]({current_badge})]({profile})

All participations done in Rust.

"""

main_entry = "{cnt}.  [{contest_name}]({contest_url}) as ![]({badge}) ![]({rating_change})\n"

contest_url_template = web + "/contest/{num}"

shield_base = "https://img.shields.io/badge/"
badge_template = shield_base + "{title}-{handle}-{color}"
rating_template = shield_base + "{title}-{rating}-{color}"
time_template = shield_base + "-{hours:>02}%3A{minutes:>02}-yellowgreen"
tot_points_template = shield_base + "Points-{points}-blue"
points_template = shield_base + "-{points}%2F{maxi}-blue"
tot_penalty_template = shield_base + "Penalty-{penalty}-red"
penalty_template = shield_base + "-{penalty}-red"
count_template = shield_base + "Participation-{number}-blueviolet"
rank_template = shield_base + "Rank-{rank}-orange"
gain_template = shield_base + "-%2B{gain}-green"
loss_template = shield_base + "---{loss}-red"
verdict_template = shield_base + "-{status}-{color}"

titles = [
    (0, "Unrated", "white"),
    (1200, "Newbie", "lightgrey"),
    (1400, "Pupil", "lightgreen"),
    (1600, "Specialist", "cyan"),
    (1900, "Expert", "blue"),
    (2100, "Candidate%20Master", "hotpink"),
    (2300, "Master", "gold"),
    (2400, "International%20Master", "yellow"),
    (2600, "Grandmaster", "red"),
    (3000, "International%20Grandmaster", "crimson"),
    (5000, "Legendary%20Grandmaster", "darkred"),
]

rating_steps = [500, 350, 250, 150, 100, 50]

yticks = [0, 500, 1000, 1200, 1400, 1600, 1900, 2100, 2300, 2400, 2600, 2800, 3000, 3200]

def find_title(rating):
    for (r, t, c) in titles:
        if rating <= r:
            return (t, c)

report_template = """# [{contest_name}](https://codeforces.com/contest/{num})

![]({participation_badge}) ![]({rank_badge}) ![]({score})

![]({prev_rating}) >>> ![]({next_rating}) ![]({rating_change})
"""

solution_banner = """
## Solutions
| ID | Title | Time | {mode} |
| --- | --- | --- | --- |
"""

solution_template = """| [{problem}](https://codeforces.com/contest/{num}/problem/{problem}) | {name} | ![]({time}) | ![]({score}) |
"""

submission_banner = """
## Submissions
| Problem | ID | Verdict |
| --- | --- | --- |
"""

submission_template = """| [{problem}](https://codeforces.com/contest/{num}/problem/{problem}) | [{ident}](https://codeforces.com/contest/{num}/submission/{ident}) | ![]({verdict}) |
"""

submission_template_lang = """| [{problem}](https://codeforces.com/contest/{num}/problem/{problem}) ![](https://img.shields.io/badge/-{lang}-{color}) | [{ident}](https://codeforces.com/contest/{num}/submission/{ident}) | ![]({verdict}) |
"""

graph_template = """
![]({src})
"""

graph_fname = ".graph.png"
