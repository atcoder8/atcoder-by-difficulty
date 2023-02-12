from . import problem_id_information


PROBLEM_URL_FORMAT = (
    "https://atcoder.jp/contests/{contest_id}/tasks/{contest_id}_{problem_index}"
)


def get_problem_url(
    problem_id_info: problem_id_information.ProblemIdInformation,
) -> str:
    return PROBLEM_URL_FORMAT.format(
        contest_id=problem_id_info.contest_id,
        problem_index=problem_id_info.problem_index,
    )
