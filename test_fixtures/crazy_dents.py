# not meant to be syntactically/logically correct!

def toptier():
    still_depth = True
    def midtier(still_depth):
        more_to_go_true = True
        def bottom_tier():
            if still_depth:
                still_depth = False
                return True
# NO DEDENTS
            else:
                if more_to_go_true:
                    more_to_go_true = False
                    return False


from foo import (thing1,
                 thing2_no_indent,
                 thing3)