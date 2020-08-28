#!/usr/bin/python3

exec(open(".data.py").read())
exec(open(".meta.py").read())

current_rating = 0
next_rating = 0

for id in participations:
    print(id)
    try:
        exec(open("{}/.data.py".format(id)).read())
    except FileNotFoundError:
        break
    current_rating = next_rating
    next_rating += rating_change
    title, color = find_title(current_rating)
    main_header += main_entry.format(
        contest_name=contest_name,
        contest_url=contest_url_template.format(id=id),
        badge=badge_template.format(title=title, color=color, handle=handle))

print(main_header)
