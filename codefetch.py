#!/usr/bin/python3

# *** CODEFETCH ***
#
# Automatic profile updater for Codeforces

import requests
import re

web = "https://codeforces.com/"
handle = "Zwgtwz"
profile = web + "profile/" + handle
contests = web + "contests/with/" + handle
submissions = web + "submissions/" + handle

f = requests.get(profile)
text = f.text.replace("\n", " ").replace("\r", "")
#print(f.text)

current_rating = int(re.search("Contest rating.*?([0-9]+)", text).group(1))
print(current_rating)
