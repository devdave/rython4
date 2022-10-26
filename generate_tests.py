import token
from tokenize import tokenize
from tokenize import _generate_tokens_from_c_tokenizer
from pathlib import Path
from argparse import ArgumentParser


def token_type_from_python_to_rust(typefield):
    match typefield:
        case token.ENCODING:
            return "TType::Encoding"
        case token.STRING:
            return "TType::String"
        case token.NAME:
            return "TType::Name"
        case token.OP:
            return "TType::Op"
        case token.NEWLINE:
            return "TType::Newline"
        case token.NUMBER:
            return "TType::Number"
        case token.INDENT:
            return "TType::Indent"
        case token.DEDENT:
            return "TType::Dedent"
        case token.ENDMARKER:
            return "TType::EndMarker"
        case token.NL:
            return "TType::NL"

        case token.COMMENT:
            return "TType::Comment"

        case default:
            return "TType::Unhandled({})".format(typefield)
            # raise ValueError("Token type Not handled yet {}".format(typefield))


def process_file(element:Path):
    with element.open("r") as my_file:
        print(f"Processing: {element}")
        print("=" * 80)
        try:
            # tokens = tokenize(my_file.readline)
            tokens = _generate_tokens_from_c_tokenizer(my_file.read())

            for idx, token in enumerate(tokens):

                ttype = f"{token_type_from_python_to_rust(token.type)}"
                if token.string == "\r\n":
                    positions = f"({token.start[1]}, {token.start[0]}), ({token.end[1]-1}, {token.end[0]})"
                    print(f"test_token_w_position!(tokens[{idx}], {ttype}, {positions}, \"\\n\" );")
                else:
                    positions = f"({token.start[1]}, {token.start[0]}), ({token.end[1]}, {token.end[0]})"
                    print(f"test_token_w_position!(tokens[{idx}], {ttype}, {positions}, \"{token.string}\" );")
        except Exception as exc:
            print(f"Failed to tokenize because {exc}")

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

