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
        return "Pupil", "green"
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
