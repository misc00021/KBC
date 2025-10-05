import os
import sys

TEMPLATE_PATH = "egg_template.txt"

def main():
    if len(sys.argv) != 2:
        print("Usage: python3 make_egg_file.py <folder_path>")
        sys.exit(1)

    folder = sys.argv[1]
    if not os.path.isdir(folder):
        print(f"Error: {folder} is not a directory.")
        sys.exit(1)

    if not os.path.isfile(TEMPLATE_PATH):
        print(f"Error: Template file not found at {TEMPLATE_PATH}.")
        sys.exit(1)

    base = os.path.basename(os.path.normpath(folder))
    out_path = os.path.join("egg_files", f"kbc_{base}.rs")

    with open(TEMPLATE_PATH, "r") as t:
        template = t.read()

    with open(out_path, "w") as out:
        out.write(template)
        out.write("\n\n")
        for name in sorted(os.listdir(folder)):
            path = os.path.join(folder, name)
            if not os.path.isfile(path):
                continue
            with open(path, "r") as f:
                out.write(f"\n(\"{name.split('.')[0]}\".to_string(), vec![\n")
                out.write(f.read())
                out.write("]),\n")
        out.write("\n]}\n")

    print(f"Created {out_path}")

if __name__ == "__main__":
    main()