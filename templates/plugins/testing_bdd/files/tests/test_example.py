import pytest


def soma(a, b):
    return a + b


@pytest.mark.parametrize(
    "a,b,expected",
    [
        (2, 3, 5),
        (-1, 2, 1),
        (0, 0, 0),
        (-5, -2, -7),
    ],
)
def test_soma(a, b, expected):
    assert soma(a, b) == expected
