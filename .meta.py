web = "https://codeforces.com"
profile = "{web}/profile/{handle}"

main_header = """# Codeforces

Transcription of my submissions for the Codeforces contests: [{web}]({web})

Handle: [![]({current_badge})]({profile})

All participations done in Rust.

"""

main_entry = "* [{contest_name}]({contest_url}) as ![]({badge})\n"

contest_url_template = web + "/contest/{id}"

def accepted(): return ("Accepted", "green")
def wrong_ans(num, pre=False):
    return ("Wrong answer on {pre}test {num}".format(pre=("pre" if pre else ""), num=num), "yellow" if num == 1 else "red")
def runtime(num, pre=False):
    return ("Runtime error on {pre}test {num}".format(pre=("pre" if pre else ""), num=num), "yellow" if num == 1  else "red")

shield_base = "https://img.shields.io/badge/"
badge_template = shield_base + "{title}-{handle}-{color}"
time_template = shield_base + "Time-{hours:>2}%3A{minutes:>2}-yellowgreen"
points_template = shield_base + "Points-{points}%2F{maxi}-blue"
penalty_template = shield_base + "Penalty-{penalty}-red"
count_template = shield_base + "Participation-{number}-blueviolet"
rank_template = shield_base + "Rank-{rank}-orange"
gain_template = shield_base + "-%2B{gain}-green"
loss_template = shield_base + "---{loss}-red"
