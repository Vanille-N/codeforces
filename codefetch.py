#!/usr/bin/python3

# *** CODEFETCH ***
#
# Automatic profile updater for Codeforces

import requests
from bs4 import BeautifulSoup
from selenium import webdriver
from selenium.webdriver.chrome.options import Options
from requests_html import HTMLSession
import re

def create_browser(webdriver_path):
    #create a selenium object that mimics the browser
    browser_options = Options()
    #headless tag created an invisible browser
    browser_options.add_argument("--headless")
    browser_options.add_argument('--no-sandbox')
    browser = webdriver.Chrome(webdriver_path, chrome_options=browser_options)
    print("Done Creating Browser")
    return browser

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
scoreboard = web + "contest/{id}/standings/friends/true"

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

Handle: [![](https://img.shields.io/badge/{title}-{handle}-{color})](https://codeforces.com/profile/{handle})

All participations done in Rust.

""".format(title=current_title, handle=handle, color=current_color)

participation_template = "* [{name}](https://codeforces.com/contest/{id}) as ![](https://img.shields.io/badge/{title}-{handle}-{color})\n"

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
        front_template += participation_template.format(name=name, id=id, title=sanitize(title), handle=sanitize(handle), color=color)

#######################
### WRITE README.md ###
#######################

header_template = """# {name}

![](https://img.shields.io/badge/Participation-{count}-blueviolet)
![](https://img.shields.io/badge/Rank-{rank}-blue)"""

score_template = """![](https://img.shields.io/badge/Points-{points}-orange)"""

rating_template = """
![](https://img.shields.io/badge/{prev_title}-{prev_rating}-{prev_color}) →
![](https://img.shields.io/badge/{new_title}-{new_rating}-{new_color})
![](https://img.shields.io/badge/-{delta}-{delta_color})
"""

solved_title = """# Solved
"""

solved_template = """* [{type} - {title}](https://codeforces.com/contest/{id}/problem/{type})
"""
solved_time_template = """![](https://img.shields.io/badge/Time-{time}-yellowgreen)
"""
solved_points_template = """![](https://img.shields.io/badge/Points-{obtained}%2F{available}-blue)
"""

browser = create_browser('/snap/bin/chromium.chromedriver') #DON'T FORGET TO CHANGE THIS AS YOUR DIRECTORY
session = HTMLSession()

standings = []
for id in participations_id:
    print(id)
    # browser.get(scoreboard.format(id=id))
    # text = browser.page_source #.replace("\n", "").replace("\r", "").replace(" ", "")
    r = session.get(scoreboard.format(id=id))
    text = r.html.render()
    print(text)
    print(1 / 0)


for i in range(len(participations_names)):
    name = participations_names[i]
    id = participations_id[i]
    folder = participations_folder[i]
    previous_rating = successive_ratings[i]
    new_rating = successive_ratings[i+1]
    delta = rating_changes[i+1]
    previous_title, previous_color = get_title(previous_rating)
    new_title, new_color = get_title(new_rating)
    count = i + 1
    rank = successive_ranks[i+1]
    delta_color = "green" if delta >= 0 else "red"
    print(header_template.format(count=count, rank=rank, prev_title=sanitize(previous_title),
        prev_rating=previous_rating, prev_color=previous_color, new_title=sanitize(new_title),
        new_rating=new_rating, new_color=new_color, delta=sanitize("{0:+d}".format(delta)),
        delta_color=delta_color, name=name))
