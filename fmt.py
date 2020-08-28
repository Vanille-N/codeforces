#!/usr/bin/python3

exec(open(".data.py").read())
exec(open(".meta.py").read())

prev_rating = 0
next_rating = 0
main = main_template

for id in participations:
    try:
        exec(open("{}/.data.py".format(id)).read())
    except FileNotFoundError:
        break
    print("<<<", id, ">>>")
    prev_rating = next_rating
    next_rating += rating_change
    prev_title, prev_color = find_title(prev_rating)
    next_title, next_color = find_title(next_rating)
    report = report_template.format(
        contest_name=contest_name,
        participation_badge=count_template.format(number=num),
        rank_badge=rank_template.format(rank=rank),
        score=(
            penalty_template.format(penalty=penalty) if educ
            else score_template.format(score=score)
            ),
        prev_rating=rating_template.format(title=prev_title, rating=prev_rating, color=prev_color),
        next_rating=rating_template.format(title=next_title, rating=next_rating, color=next_color),
        rating_change=(
            gain_template.format(gain=rating_change) if rating_change > 0
            else loss_template.format(loss=abs(rating_change))
            ))
    main += main_entry.format(
        contest_name=contest_name,
        contest_url=contest_url_template.format(id=id),
        badge=badge_template.format(title=prev_title, color=prev_color, handle=handle))
    print(report)
    print("----------------")

print(main)
