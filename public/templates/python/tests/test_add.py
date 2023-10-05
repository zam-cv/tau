import pytest

def add(a, b):
    return a + b

def test_add():
    assert add(1, 2) == 3