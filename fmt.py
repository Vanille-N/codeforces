#!/usr/bin/python3

exec(open(".data.py").read())
exec(open(".meta.py").read())

for id in participations:
    exec(open("{}/.data.py".format(id)).read())
