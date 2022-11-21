import symtable
import os, sys

def walk_table(st: symtable.SymbolTable, depth=0):
    def print_d(txt, *args):
        prefix = " " * depth
        print(f"{prefix}{txt}", *args)

    assert isinstance(st, symtable.SymbolTable)

    assert isinstance(st, symtable.SymbolTable)
    print_d('Symtable: type=%s, id=%s, name=%s' % (
        st.get_type(), st.get_id(), st.get_name()))
    print_d('  nested:', st.is_nested())
    print_d('  has children:', st.has_children())
    for ident in st.get_identifiers():
        sym = st.lookup(ident)
        if sym.is_namespace() == False:
            describe_symbol(sym, depth+5)

    for child_st in st.get_children():
        walk_table(child_st, depth + 5)




def describe_symbol(sym: symtable.Symbol, depth=0):
    def print_d(txt, *args):
        prefix = " " * depth
        print(f"{prefix}{txt}", *args)

    assert type(sym) == symtable.Symbol
    print_d("Symbol: ", sym.get_name())
    for prop in [
        'referenced',
        'imported',
        'parameter',
        'global',
        'declared_global',
        'local',
        'free',
        'assigned',
        'namespace'
    ]:
        if sym.is_namespace() == True:
            continue

        if property := getattr(sym, f"is_{prop}", False): # type: symtable.Symbol
            if property() == True:
                print_d('   is ', prop)





def main():
    with open(sys.argv[1]) as f:
        src = f.read()

    mod = symtable.symtable(src, os.path.split(sys.argv[0])[1], "exec")

    walk_table(mod, 0)




if __name__ == '__main__':
    main()

