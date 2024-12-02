import os


def main():
    script_dir = os.path.dirname(__file__)
    with open(f"{script_dir}/input.txt") as f:
        reports = f.readlines()

    reports = [line.strip() for line in reports]


    safe_reports = sum([
        1 for report in reports
        if is_safe(report) or is_safe_v2(report)
    ])

    print(safe_reports)



def is_safe(report: str):
    max_diff = 3
    min_diff = 1

    report = list(map(int, report.split(" ")))

    is_increasing = report[0] < report[1]

    for i in range(1, len(report)):
        if is_increasing:
            if report[i-1] > report[i]:
                return False
        else:
            if report[i-1] < report[i]:
                return False

        diff = abs(report[i-1] - report[i])
        if diff > max_diff or diff < min_diff:
            return False

    return True


def is_safe_v2(report: str):
    """Return true if the report can become safe after removing one element"""
    report = list(map(int, report.split(" ")))

    for i in range(0, len(report)):
        temporary_report = report.copy()
        temporary_report.pop(i)

        if is_safe(" ".join(map(str, temporary_report))):
            return True

    return False


if __name__ == '__main__':
    main()
