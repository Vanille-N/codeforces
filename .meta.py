web = "https://codeforces.com"
profile = "{web}/profile/{handle}"

main_header = """# Codeforces

Transcription of my submissions for the Codeforces contests: [{web}]({web})

Handle: [![]({current_badge})]({profile})

All participations done in Rust.

"""

main_entry = "* [{contest_name}]({contest_url}) as ![]({badge})\n"

def accepted(): return ("Accepted", "green")
def wrong_ans(num, pre=False):
    return ("Wrong answer on {pre}test {num}".format(pre=("pre" if pre else ""), num=num), "yellow" if num == 1 else "red")
def runtime(num, pre=False):
    return ("Runtime error on {pre}test {num}".format(pre=("pre" if pre else ""), num=num), "yellow" if num == 1  else "red")
