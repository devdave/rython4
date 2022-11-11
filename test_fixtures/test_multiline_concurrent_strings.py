def foo(first, second):
    print(len(first))
    print(len(second))


foo(
    """Hello world this is  the firs
    multiline string""", """
    This is the second multiline"""
)
