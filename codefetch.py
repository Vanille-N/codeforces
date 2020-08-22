#!/usr/bin/python3

# *** CODEFETCH ***
#
# Automatic profile updater for Codeforces

import requests
import re

def get_title(rating):
    if rating == 0:
        return "Unrated", "white"
    elif rating <= 1200:
        return "Newbie", "lightgray"
    elif rating <= 1400:
        return "Pupil", "lightgreen"
    elif rating <= 1600:
        return "Specialist", "cyan"
    elif rating <= 1900:
        return "Expert", "blue"
    elif rating <= 2100:
        return "Candidate Master", "hotpink"
    elif rating <= 2300:
        return "Master", "gold"
    elif rating <= 2400:
        return "International Master", "yellow"
    elif rating <= 2600:
        return "Grandmaster", "red"
    elif rating <= 3000:
        return "International Grandmaster", "crimson"
    else:
        return "Legendary Grandmaster", "crimson"

def sanitize(s):
    return s.replace("+", "%2B").replace("-", "--").replace(":", "%3A").replace(" ", "%20").replace("/", "%2F")

web = "https://codeforces.com/"
handle = "Zwgtwz"
profile = web + "profile/" + handle
contests = web + "contests/with/" + handle
submissions = web + "submissions/" + handle

f = requests.get(profile)
text = f.text.replace("\n", "").replace("\r", "").replace(" ", "")
#print(f.text)

participations = eval(re.search("data.push\((.*)\);data.push", text).group(1))
rating_changes = [0]
successive_ranks = [0]
successive_ratings = [0]

for part in participations:
    rating_changes.append(part[5])
    successive_ranks.append(part[6])
    successive_ratings.append(part[1])

current_title, current_color = get_title(successive_ratings[-1])

front_template = """# Codeforces

Transcription of my submissions for the Codeforces contests: [https://codeforces.com/](https://codeforces.com/)

Handle: [![](https://img.shields.io/badge/{}-{}-{})](https://codeforces.com/profile/{})

All participations done in Rust.

""".format(current_title, handle, current_color, handle)

participation_template = "* [{}](https://codeforces.com/contest/{}) as ![](https://img.shields.io/badge/{}-{}-{})\n"


participations_names = []
participations_id = []
participations_folder = []
with open(".participations.py") as f:
    for idx, val in enumerate(eval(f.read())):
        name, id, folder = val
        participations_names.append(name)
        participations_id.append(id)
        participations_folder.append(folder)
        title, color = get_title(successive_ratings[idx])
        front_template += participation_template.format(name, id, title, handle, color)

print(front_template)
