web = "https://codeforces.com"
profile = web + "/profile/{handle}"

main_template = """# Codeforces

Transcription of my submissions for the Codeforces contests: [{web}]({web})

Handle: [![]({current_badge})]({profile})

All participations done in Rust.

"""

main_entry = "* [{contest_name}]({contest_url}) as ![]({badge})\n"

contest_url_template = web + "/contest/{num}"

def accepted(): return ("Accepted", "green")
def wrong_ans(num, pre=False):
    return ("Wrong%20answer%20on%20{pre}test%20{num}".format(pre=("pre" if pre else ""), num=num), "yellow" if num == 1 else "red")
def runtime(num, pre=False):
    return ("Runtime%20error%20on%20{pre}test%20{num}".format(pre=("pre" if pre else ""), num=num), "yellow" if num == 1  else "red")

shield_base = "https://img.shields.io/badge/"
badge_template = shield_base + "{title}-{handle}-{color}"
rating_template = shield_base + "{title}-{rating}-{color}"
time_template = shield_base + "Time-{hours:>02}%3A{minutes:>02}-yellowgreen"
score_template = shield_base + "Points-{points}-blue"
points_template = shield_base + "Points-{points}%2F{maxi}-blue"
penalty_template = shield_base + "Penalty-{penalty}-red"
count_template = shield_base + "Participation-{number}-blueviolet"
rank_template = shield_base + "Rank-{rank}-orange"
gain_template = shield_base + "-%2B{gain}-green"
loss_template = shield_base + "---{loss}-red"
verdict_template = shield_base + "-{status}-{color}"

def find_title(rating):
    if rating == 0: return ("Unrated", "white")
    elif rating <= 1200: return ("Newbie", "lightgrey")
    elif rating <= 1400: return ("Pupil", "lightgreen")
    elif rating <= 1600: return ("Specialist", "cyan")
    elif rating <= 1900: return ("Expert", "blue")
    elif rating <= 2100: return ("Candidate%20Master", "pink")
    elif rating <= 2300: return ("Master", "gold")
    elif rating <= 2400: return ("International%20Master", "yellow")
    elif rating <= 2600: return ("Grandmaster", "red")
    elif rating <= 3000: return ("International%20Grandmaster", "crimson")
    else: return ("Legendary%20Grandmaster", crimson)

report_template = """# {contest_name}

![]({participation_badge})
![]({rank_badge})
![]({score})

![]({prev_rating}) →
![]({next_rating})
![]({rating_change})
"""

solution_template = """* [{problem} - {name}](https://codeforces.com/contest/{num}/problem/{problem})
![]({time})
![]({score})
"""

submission_template = """* [{problem} - {ident}](https://codeforces.com/contest/{num}/submission/{ident})
![]({verdict})
"""
