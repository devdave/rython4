def foo(first, second):
    print(len(first))
    print(len(second))


foo(
    """Hello world this is  the first
    multiline string""", """
    This is the second multiline"""
)
