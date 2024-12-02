import os


def main():
    script_dir = os.path.dirname(__file__)
    with open(f"{script_dir}/input.txt") as f:
        reports = f.readlines()

    reports = [line.strip() for line in reports]


    safe_reports = sum([
        1 for report in reports
        if is_safe(report)
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


if __name__ == '__main__':
    main()
