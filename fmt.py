#!/usr/bin/python3

class Module:
    def __init__(self, path, **kwargs):
        # exec(open(path).read(), locals())
        # print(web)
        d = kwargs
        exec(open(path).read(), d)
        d.pop("__builtins__")
        for k in d.keys():
            instr = "self.{elem} = d['{elem}']".format(elem=k)
            exec(instr)

meta = Module(".meta.py")
data = Module(".data.py")
verdict = Module(".verdict.py")

prev_rating = 0
next_rating = 0
main = meta.main_template

time_data = []
rating_data = []

for cnt, ref in enumerate(data.participations):
    num, id = ref
    try:
        contest = Module("{}/.data.py".format(id), verdict=verdict)
    except FileNotFoundError:
        break
    print("Reading data for contest", id)
    prev_rating = next_rating
    next_rating += contest.rating_change
    prev_title, prev_color = meta.find_title(prev_rating)
    next_title, next_color = meta.find_title(next_rating)
    rating_data.append(next_rating)
    time_data.append(contest.date)
    report = meta.report_template.format(
        contest_name=contest.name,
        num=num,
        participation_badge=meta.count_template.format(number=cnt+1),
        rank_badge=meta.rank_template.format(rank=contest.rank),
        score=(
            meta.tot_penalty_template.format(penalty=contest.penalty) if contest.educ
            else meta.tot_points_template.format(points=contest.points)
            ),
        prev_rating=meta.rating_template.format(title=prev_title, rating=prev_rating, color=prev_color),
        next_rating=meta.rating_template.format(title=next_title, rating=next_rating, color=next_color),
        rating_change=(
            meta.gain_template.format(gain=contest.rating_change) if contest.rating_change > 0
            else meta.loss_template.format(loss=abs(contest.rating_change))
            ))
    main += meta.main_entry.format(
        cnt=cnt+1,
        contest_name=contest.name,
        contest_url=meta.contest_url_template.format(num=num),
        badge=meta.badge_template.format(title=prev_title, color=prev_color, handle=data.handle),
        rating_change=(
            meta.gain_template.format(gain=contest.rating_change) if contest.rating_change > 0
            else meta.loss_template.format(loss=abs(contest.rating_change))
            ))
    report += meta.solution_banner.format(mode=("Penalty" if contest.educ else "Points"))
    for sol in contest.solutions:
        report += meta.solution_template.format(
            problem=sol["problem"],
            name=sol["name"],
            num=num,
            time=meta.time_template.format(hours=sol["time"][0], minutes=sol["time"][1]),
            score=(
                meta.penalty_template.format(penalty=sol["penalty"]) if contest.educ
                else meta.points_template.format(points=sol["points"][0], maxi=sol["points"][1])
                ))
    print("    ", len(contest.solutions), "problems solved")
    report += meta.submission_banner
    for sub in contest.submissions:
        if "lang" in sub:
            lang, color = sub["lang"]
            report += meta.submission_template_lang.format(
                problem=sub["problem"],
                ident=sub["id"],
                lang=lang,
                color=color,
                num=num,
                verdict=meta.verdict_template.format(
                    status=sub["status"][0],
                    color=sub["status"][1],
                    )
                )
        else:
            report += meta.submission_template.format(
                problem=sub["problem"],
                ident=sub["id"],
                num=num,
                verdict=meta.verdict_template.format(
                    status=sub["status"][0],
                    color=sub["status"][1],
                    )
                )
    print("    ", len(contest.submissions), "files submitted")

    with open(meta.file_template.format(id=id), 'w') as f:
        f.write(report)

current_rating = next_rating
current_title, current_color = meta.find_title(current_rating)
main = main.format(
    web=meta.web,
    current_badge=meta.badge_template.format(
        title=current_title,
        color=current_color,
        handle=data.handle
        ),
    profile=meta.profile.format(handle=data.handle),
    )

import time
from datetime import datetime, timedelta
import matplotlib.pyplot as plt

fig, ax = plt.subplots()

time_data = [datetime(year=y, month=m, day=d) for (y, m, d) in time_data]

ax.plot(time_data, rating_data, marker="o", color='black', alpha=1)
ax.grid(color='black', linestyle='--', linewidth=0.5)
ax.set_yticks(meta.yticks)
fig.autofmt_xdate()
max_rating = max(rating_data) * 1.3
min_rating = min(rating_data) - 100

ydelta = (max_rating - min_rating) / 25

for i in range(1, len(meta.titles)):
    if meta.titles[i-1][0] < max_rating:
        ymax = min(meta.titles[i][0], max_rating)
        ymin = max(meta.titles[i-1][0], min_rating)
        ax.axhspan(
            ymax,
            ymin,
            color=meta.titles[i][2],
            alpha=0.5
            )
        if ymax - ymin > ydelta * 2:
            ax.text(time_data[0], ymax - ydelta, meta.titles[i][1].replace("%20", " "))

ax.set_ylim((min_rating, max_rating))

main += meta.graph_template.format(src=meta.graph_fname)

with open(meta.main_file, 'w') as f:
    f.write(main)

plt.savefig(meta.graph_fname)
# plt.show()
