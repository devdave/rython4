import token as ttype
from tokenize import tokenize
from tokenize import _generate_tokens_from_c_tokenizer
from pathlib import Path
from argparse import ArgumentParser


def token_type_from_python_to_rust(typefield):
    match typefield:
        case ttype.ENCODING:
            return "TType::Encoding"
        case ttype.STRING:
            return "TType::String"
        case ttype.NAME:
            return "TType::Name"
        case ttype.OP:
            return "TType::Op"
        case ttype.NEWLINE:
            return "TType::NL"
        case ttype.NUMBER:
            return "TType::Number"
        case ttype.INDENT:
            return "TType::Indent"
        case ttype.DEDENT:
            return "TType::Dedent"
        case ttype.ENDMARKER:
            return "TType::EndMarker"
        case ttype.NL:
            return "TType::NL"

        case ttype.COMMENT:
            return "TType::Comment"

        case default:
            # Assume these are operators
            return "TType::Op".format(typefield)
            # raise ValueError("Token type Not handled yet {}".format(typefield))


def process_file(element:Path):
    with element.open("r") as my_file:
        print(f"Processing: {element}")
        print("=" * 80)
        try:
            # tokens = tokenize(my_file.readline)
            tokens = _generate_tokens_from_c_tokenizer(my_file.read())

            for idx, token in enumerate(tokens):

                type_str = f"{token_type_from_python_to_rust(token.type)}"
                positions = f"({token.start[1]}, {token.start[0]}), ({token.end[1]}, {token.end[0]})"

                if token.string in ("\r\n", "\n", "\r") or token.type in [ttype.NEWLINE, ttype.NL]:
                    positions = f"({token.start[1]}, {token.start[0]}), ({token.end[1]}, {token.end[0]})"
                    print(f"test_token_w_position!(tokens[{idx}], {type_str}, {positions}, \"\" );")
                elif token.type in (ttype.INDENT, ttype.DEDENT):
                    print(f"test_token_w_position!(tokens[{idx}], {type_str}, {positions}, \"\" );")
                else:
                    positions = f"({token.start[1]}, {token.start[0]}), ({token.end[1]}, {token.end[0]})"
                    print(f"test_token_w_position!(tokens[{idx}], {type_str}, {positions}, \"{token.string}\" );")
        except Exception as exc:
            print(f"Failed to tokenize because {exc}")
            raise

        print("Finished\n")



def walk_workingpath(work_path:Path):
    if work_path.is_dir():
        for element in work_path.glob("*.py"):
            if element.is_file():
                process_file(element)
    elif work_path.is_file():
        process_file(work_path)







def main():
    parser = ArgumentParser()
    parser.add_argument("work_path", help="Path filled with python files to be tokenized.", type=Path)

    args = parser.parse_args()

    walk_workingpath(args.work_path)



if __name__ == '__main__':
    main()

