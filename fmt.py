#!/usr/bin/python3

exec(open(".data.py").read())
exec(open(".meta.py").read())

prev_rating = 0
next_rating = 0
main = main_template

time_data = []
rating_data = []

for cnt, ref in enumerate(participations):
    num, id = ref
    try:
        exec(open("{}/.data.py".format(id)).read())
    except FileNotFoundError:
        break
    print("Reading data for contest", id)
    prev_rating = next_rating
    next_rating += rating_change
    prev_title, prev_color = find_title(prev_rating)
    next_title, next_color = find_title(next_rating)
    rating_data.append(next_rating)
    time_data.append(date)
    report = report_template.format(
        contest_name=contest_name,
        num=num,
        participation_badge=count_template.format(number=cnt+1),
        rank_badge=rank_template.format(rank=rank),
        score=(
            penalty_template.format(penalty=penalty) if educ
            else score_template.format(points=points)
            ),
        prev_rating=rating_template.format(title=prev_title, rating=prev_rating, color=prev_color),
        next_rating=rating_template.format(title=next_title, rating=next_rating, color=next_color),
        rating_change=(
            gain_template.format(gain=rating_change) if rating_change > 0
            else loss_template.format(loss=abs(rating_change))
            ))
    main += main_entry.format(
        cnt=cnt+1,
        contest_name=contest_name,
        contest_url=contest_url_template.format(num=num),
        badge=badge_template.format(title=prev_title, color=prev_color, handle=handle),
        rating_change=(
            gain_template.format(gain=rating_change) if rating_change > 0
            else loss_template.format(loss=abs(rating_change))
            ))
    report += solution_banner.format(mode=("Penalty" if educ else "Points"))
    for sol in solutions:
        report += solution_template.format(
            problem=sol["problem"],
            name=sol["name"],
            num=num,
            time=time_template.format(hours=sol["time"][0], minutes=sol["time"][1]),
            score=(
                penalty_template.format(penalty=sol["penalty"]) if educ
                else points_template.format(points=sol["points"][0], maxi=sol["points"][1])
                ))
    print("    ", len(solutions), "problems solved")
    report += "\n## Submissions\n"
    for sub in submissions:
        report += submission_template.format(
            problem=sub["problem"],
            ident=sub["id"],
            num=num,
            verdict=verdict_template.format(
                status=sub["status"][0],
                color=sub["status"][1],
                )
            )
    print("    ", len(submissions), "files submitted")

    with open(file_template.format(id=id), 'w') as f:
        f.write(report)

current_rating = next_rating
current_title, current_color = find_title(current_rating)
main = main.format(
    web=web,
    current_badge=badge_template.format(title=current_title, color=current_color, handle=handle),
    profile=profile.format(handle=handle),
    )

import time
from datetime import datetime, timedelta
import matplotlib.pyplot as plt

time_data = [datetime(year=y, month=m, day=d) for (y, m, d) in time_data]

plt.plot(time_data, rating_data, marker="o", color='black')
plt.gcf().autofmt_xdate()
max_rating = max(rating_data) * 1.3
min_rating = min(rating_data) - 100

total_delta = time_data[-1] - time_data[0]
time_margin = total_delta * 0.05
extended_time = [time_data[0] - time_margin, time_data[-1] + time_margin]

for i in range(1, len(titles)):
    if titles[i-1][0] < max_rating:
        plt.fill_between(extended_time, max(titles[i-1][0], min_rating), min(titles[i][0], max_rating), color=titles[i][2])

main += graph_template.format(src=graph_fname)

with open(main_file, 'w') as f:
    f.write(main)

plt.savefig(graph_fname)
