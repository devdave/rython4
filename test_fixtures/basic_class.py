class Basic:
    """
    A basic class definition
    """
    def __init__(self):
        """
        A basic init
        """
        self.c = 0

    def add(self, a, b):
        self.c = a + b

    def get(self):
        return self.c
