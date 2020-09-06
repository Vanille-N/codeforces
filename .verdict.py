def accepted():
    return ("Accepted", "brightgreen")

def wrong_ans(num, pre=False):
    return (
        "Wrong%20answer%20on%20{pre}test%20{num}".format(
            pre=("pre" if pre else ""),
            num=num,
        ),
        "yellow" if num == 1 else "red"
    )

def runtime(num, pre=False):
    return (
        "Runtime%20error%20on%20{pre}test%20{num}".format(
            pre=("pre" if pre else ""),
            num=num,
        ),
        "yellow" if num == 1  else "red"
    )
